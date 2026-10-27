function attachVideoButton() {
    document.querySelectorAll('video').forEach(video => {
        if (video.dataset.turbodmReady) return;
        video.dataset.turbodmReady = "true";
        const btn = document.createElement('div');
        btn.innerHTML = "⚡ تحميل الفيديو (TurboDM)";
        Object.assign(btn.style, {
            position: 'fixed', top: '15px', left: '15px', zIndex: '2147483647',
            background: 'linear-gradient(135deg, #2563eb, #7c3aed)', color: '#fff',
            padding: '8px 14px', borderRadius: '8px', fontSize: '13px', fontWeight: 'bold', cursor: 'pointer'
        });
        btn.onclick = (e) => {
            e.stopPropagation();
            chrome.runtime.sendMessage({ action: "DOWNLOAD_VIDEO", url: window.location.href, title: document.title });
            btn.innerHTML = "✅ تم الإرسال!";
            setTimeout(() => { btn.innerHTML = "⚡ تحميل الفيديو (TurboDM)"; }, 3000);
        };
        document.body.appendChild(btn);
    });
}
setInterval(attachVideoButton, 2000);
