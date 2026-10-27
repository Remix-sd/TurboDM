<div align="center">

# ⚡ TurboDM (Turbo Download Manager)
### مدير تنزيلات فائق السرعة، مفتوح المصدر، وعابر للمنصات (Cross-Platform)

[![Build Status](https://github.com/YOUR_USERNAME/TurboDM/actions/workflows/release.yml/badge.svg)](https://github.com/YOUR_USERNAME/TurboDM/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20Android%20%7C%20macOS-lightgrey)](https://github.com/YOUR_USERNAME/TurboDM)

إدارة تنزيلات متقدمة، تقسيم ملفات ديناميكي، سحب وسائط ذكي، وإضافات متصفحات شاملة.

[المميزات](#-المميزات-الرئيسية) • [التقنيات المستخدمة](#-التقنيات-المستخدمة) • [طريقة التثبيت](#-طريقة-التثبيت) • [المساهمة](#-المساهمة)

</div>

---

## 🚀 نظرة عامة (Overview)

برنامج **TurboDM** هو بديل عصري وعالي الأداء لمديرات التنزيل التقليدية (مثل IDM). تم تصميمه هندسيًا ليكون **خفيفاً جداً على موارد الجهاز**، سريع الاستجابة، ويعمل بسلاسة تامة على مختلف أنظمة التشغيل (Windows, Linux, Android) بفضل اعتماده على محرك مكتوب بلغة **Rust** وواجهة عصرية مرنة بـ **HTML/CSS/JS**.

---

## ✨ المميزات الرئيسية (Key Features)

* **⚡ تقسيم الملفات الديناميكي (Dynamic Segmentation):** تسريع التنزيلات عبر تقسيم الملف إلى عدة أجزاء وتحميلها بالتوازي مع ميزة سحب الاتصالات الميتة.
* **🎬 محرك سحب الوسائط الذكي:** دمج قوي لمكتبة `yt-dlp` و `FFmpeg` لسحب مقاطع الفيديو والبث المباشر (M3U8 / HLS) بجودات عالية ودمجها تلقائياً.
* **🌐 إضافات متصفحات موحدة (Browser Integration):** اعتراض التنزيلات من (Chrome, Edge, Firefox, Brave) تلقائياً، مع دعم سحب ملفات تعريف الارتباط (`Cookies`) للتحميل من المواقع المحمية.
* **📱 دعم الأجهزة الذكية (Android):** تطبيق متجاوب يعمل على الهواتف مع دعم التقاط وإدارة الملفات.
* **🛡️ استقرار وفائقة الأداء:** مبني باستخدام إطار **Tauri v2** ولغة **Rust** لضمان أمان الذاكرة وسرعة تنفيذ خرافية مع استهلاك معدوم للرام.

---

## 🛠️ التقنيات المستخدمة (Tech Stack)

* **Core Engine:** Rust (Tokio, Axum, Reqwest)
* **Desktop & Mobile Framework:** Tauri v2
* **Frontend UI:** HTML5, CSS3 (Soft-UI Dark Theme), Vanilla JavaScript
* **Video Parsing & Processing:** yt-dlp, FFmpeg
* **Automation:** GitHub Actions (CI/CD)

---

## 📦 التحميل والتثبيت (Downloads)

توجه مباشرة إلى صفحة [Releases](../../releases) أو قسم الـ **Actions Artifacts** في المستودع لتحميل الحزم الجاهزة بحسب نظام تشغيلك:

* **Windows:** ملفات التثبيت `(.exe` / `.msi`)
* **Linux:** حزم التوزيعات `(.deb` / `.AppImage`)
* **Android:** تطبيق الهاتف `(.apk)`
* **Browser Extension:** ملف الإضافة المضغوط `(.zip)`

---

## ⚙️ البناء والتطوير المحلي (Local Development)

إذا كنت ترغب في تشغيل المشروع أو التعديل عليه محلياً:

1. **استنساخ المستودع:**
   ```bash
   git clone [https://github.com/YOUR_USERNAME/TurboDM.git](https://github.com/YOUR_USERNAME/TurboDM.git)
   cd TurboDM
