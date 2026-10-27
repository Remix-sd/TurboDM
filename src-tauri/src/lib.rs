use axum::{extract::State, routing::{get, post}, Json, Router};
use futures_util::StreamExt;
use reqwest::header::{HeaderMap, COOKIE, RANGE, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::OpenOptions, io::{Seek, SeekFrom, Write}, path::PathBuf, process::Command, sync::{Arc, Mutex}};
use tauri::Manager;
use tower_http::cors::CorsLayer;

#[derive(Clone, Serialize, Deserialize)]
pub struct DownloadRequest {
    id: String, url: String, filename: String, segments: u64, is_video: bool, cookies: Option<String>, user_agent: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct TaskStatus {
    id: String, filename: String, progress: f64, status: String, is_video: bool,
}

pub struct AppState {
    tasks: Mutex<HashMap<String, TaskStatus>>, download_dir: PathBuf,
}

type SharedState = Arc<AppState>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let dl_dir = app.path().download_dir().unwrap_or_else(|_| app.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("./")));
            let _ = std::fs::create_dir_all(&dl_dir);
            let state = Arc::new(AppState { tasks: Mutex::new(HashMap::new()), download_dir: dl_dir });
            tauri::async_runtime::spawn(async move {
                let router = Router::new()
                    .route("/api/download", post(start_download_handler))
                    .route("/api/status", get(get_status_handler))
                    .layer(CorsLayer::permissive())
                    .with_state(state);
                if let Ok(listener) = tokio::net::TcpListener::bind("127.0.0.1:45678").await {
                    let _ = axum::serve(listener, router).await;
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn get_status_handler(State(state): State<SharedState>) -> Json<Vec<TaskStatus>> {
    Json(state.tasks.lock().unwrap().values().cloned().collect())
}

async fn start_download_handler(State(state): State<SharedState>, Json(req): Json<DownloadRequest>) -> Json<&'static str> {
    let state_clone = state.clone();
    state.tasks.lock().unwrap().insert(req.id.clone(), TaskStatus {
        id: req.id.clone(), filename: req.filename.clone(), progress: 0.0, status: "جاري الاتصال...".to_string(), is_video: req.is_video,
    });
    tokio::spawn(async move {
        if req.is_video && !cfg!(target_os = "android") { run_yt_dlp(req, state_clone).await; }
        else { run_segmented_download(req, state_clone).await; }
    });
    Json("Started")
}

async fn run_segmented_download(req: DownloadRequest, state: SharedState) {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    if let Some(ua) = &req.user_agent { if let Ok(v) = ua.parse() { headers.insert(USER_AGENT, v); } }
    if let Some(ck) = &req.cookies { if let Ok(v) = ck.parse() { headers.insert(COOKIE, v); } }
    let total_size = match client.head(&req.url).headers(headers.clone()).send().await {
        Ok(r) => r.headers().get(reqwest::header::CONTENT_LENGTH).and_then(|v| v.to_str().ok()).and_then(|v| v.parse::<u64>().ok()).unwrap_or(0),
        Err(_) => 0,
    };
    let file_path = state.download_dir.join(&req.filename);
    if let Ok(file) = OpenOptions::new().create(true).write(true).open(&file_path) {
        if total_size > 0 { let _ = file.set_len(total_size); }
    }
    let num_segments = if total_size > 1024 * 1024 { req.segments } else { 1 };
    let chunk_size = if num_segments > 0 { total_size / num_segments } else { 0 };
    let downloaded_bytes = Arc::new(Mutex::new(0u64));
    let mut handles = vec![];

    for i in 0..num_segments {
        let start = i * chunk_size;
        let end = if i == num_segments - 1 { total_size.saturating_sub(1) } else { (i + 1) * chunk_size - 1 };
        let (client_c, url_c, path_c, headers_c, dl_bytes_c, state_c, req_id) = (client.clone(), req.url.clone(), file_path.clone(), headers.clone(), downloaded_bytes.clone(), state.clone(), req.id.clone());
        handles.push(tokio::spawn(async move {
            let mut req_b = client_c.get(&url_c).headers(headers_c);
            if total_size > 0 && num_segments > 1 { req_b = req_b.header(RANGE, format!("bytes={}-{}", start, end)); }
            if let Ok(resp) = req_b.send().await {
                let mut stream = resp.bytes_stream();
                if let Ok(mut file) = OpenOptions::new().write(true).open(&path_c) {
                    let _ = file.seek(SeekFrom::Start(start));
                    while let Some(Ok(chunk)) = stream.next().await {
                        let _ = file.write_all(&chunk);
                        let mut dl = dl_bytes_c.lock().unwrap();
                        *dl += chunk.len() as u64;
                        if let Ok(mut map) = state_c.tasks.lock() {
                            if let Some(task) = map.get_mut(&req_id) {
                                task.progress = if total_size > 0 { ((*dl as f64 / total_size as f64) * 100.0).min(99.9) } else { 50.0 };
                                task.status = format!("{:.1} MB", *dl as f64 / 1048576.0);
                            }
                        }
                    }
                }
            }
        }));
    }
    for h in handles { let _ = h.await; }
    if let Ok(mut map) = state.tasks.lock() {
        if let Some(task) = map.get_mut(&req.id) { task.progress = 100.0; task.status = "✅ مكتمل".to_string(); }
    }
}

async fn run_yt_dlp(req: DownloadRequest, state: SharedState) {
    let out_template = state.download_dir.join("%(title)s.%(ext)s").to_string_lossy().to_string();
    let status = Command::new("yt-dlp").args(["-f", "best[ext=mp4]/best", "-o", &out_template, &req.url]).status();
    if let Ok(mut map) = state.tasks.lock() {
        if let Some(task) = map.get_mut(&req.id) {
            if let Ok(s) = status { if s.success() { task.progress = 100.0; task.status = "✅ تم حفظ الفيديو".to_string(); return; } }
            task.status = "❌ تأكد من وجود yt-dlp".to_string();
        }
    }
}
