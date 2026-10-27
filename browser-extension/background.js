const ENGINE_URL = "http://127.0.0.1:45678/api/download";
chrome.downloads.onCreated.addListener(async (downloadItem) => {
    if (downloadItem.state !== "in_progress") return;
    const url = downloadItem.finalUrl || downloadItem.url;
    if (url.startsWith("blob:") || url.startsWith("data:") || url.startsWith("chrome:")) return;
    chrome.downloads.cancel(downloadItem.id);
    chrome.downloads.erase({ id: downloadItem.id });
    chrome.cookies.getAll({ url: url }, async (cookies) => {
        const cookieString = cookies ? cookies.map(c => `${c.name}=${c.value}`).join('; ') : '';
        const rawName = downloadItem.filename ? downloadItem.filename.split(/[/\\]/).pop() : url.split('/').pop().split('?')[0] || "file.bin";
        try {
            await fetch(ENGINE_URL, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    id: Date.now().toString(),
                    url: url,
                    filename: decodeURIComponent(rawName),
                    segments: 8,
                    is_video: false,
                    cookies: cookieString,
                    user_agent: navigator.userAgent
                })
            });
        } catch (err) {}
    });
});
chrome.runtime.onMessage.addListener((msg) => {
    if (msg.action === "DOWNLOAD_VIDEO") {
        fetch(ENGINE_URL, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                id: Date.now().toString(),
                url: msg.url,
                filename: (msg.title || "video").replace(/[\\/:*?"<>|]/g, "_") + ".mp4",
                segments: 1,
                is_video: true
            })
        }).catch(() => {});
    }
});
