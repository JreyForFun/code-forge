//! Everything that differs between the desktop app and the web version (phones, tablets, laptops in a
//! browser): the clock, files, dialogs and saving pictures. The rest of the game calls these and never
//! needs to know where it runs.
use std::cell::RefCell;

thread_local! {
    /// Text of a progress file the player picked or dropped, waiting to be imported (read
    /// asynchronously on the web, so the game picks it up on the next frame).
    static PENDING: RefCell<Option<Result<String, String>>> = const { RefCell::new(None) };
}

/// A picked/dropped progress file that finished loading, if any.
pub fn take_import() -> Option<Result<String, String>> { PENDING.with(|p| p.borrow_mut().take()) }
fn deliver(r: Result<String, String>) { PENDING.with(|p| *p.borrow_mut() = Some(r)); }

/// Seconds since 1970 (UTC). Works in browsers too (std's clock does not).
pub fn now_secs() -> u64 { chrono::Utc::now().timestamp().max(0) as u64 }
/// A changing number for seeding randomness.
pub fn seed() -> u64 { let t = chrono::Utc::now(); (t.timestamp() as u64).wrapping_mul(1_000_000_007) ^ t.timestamp_subsec_nanos() as u64 }

pub const IS_WEB: bool = cfg!(target_arch = "wasm32");

// ------------------------------------------------------------------------------------- desktop
#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
mod imp {
    use super::deliver;

    pub fn export(file_name: &str, text: &str) -> Result<String, String> {
        let Some(path) = rfd::FileDialog::new().set_title("Save your Code Forge progress").set_file_name(file_name).add_filter("Code Forge progress", &["codeforge"]).save_file() else { return Err(String::new()) };
        std::fs::write(&path, text).map_err(|e| format!("Couldn't save the file: {}", e))?;
        Ok(format!("Progress saved to {}", path.display()))
    }

    pub fn pick_import() {
        if let Some(p) = rfd::FileDialog::new().set_title("Open Code Forge progress").add_filter("Code Forge progress", &["codeforge"]).pick_file() {
            deliver(std::fs::read_to_string(p).map_err(|_| "Couldn't read that file.".to_string()));
        }
    }

    pub fn dropped(f: &dyn eframe::egui::DroppedFile) {
        if !f.path().as_os_str().is_empty() { deliver(std::fs::read_to_string(f.path()).map_err(|_| "Couldn't read that file.".to_string())); }
    }

    pub fn save_png(file_name: &str, img: &image::RgbaImage) -> Result<String, String> {
        let base = std::env::var("USERPROFILE").map(|h| std::path::Path::new(&h).join("Pictures")).ok().filter(|p| p.is_dir()).unwrap_or_else(crate::save::Save::dir);
        let path = base.join(file_name);
        img.save(&path).map_err(|e| e.to_string())?;
        Ok(path.display().to_string())
    }
}

// ------------------------------------------------------------------------------------- android
static ANDROID_DIRS: std::sync::OnceLock<(Option<std::path::PathBuf>, Option<std::path::PathBuf>)> = std::sync::OnceLock::new();

/// Android: remember the app's private folder (saves) and its files folder (exports, pictures).
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
pub fn set_android_dirs(internal: Option<std::path::PathBuf>, external: Option<std::path::PathBuf>) { let _ = ANDROID_DIRS.set((internal, external)); }
/// The app's private data folder on Android (None elsewhere).
pub fn android_data_dir() -> Option<std::path::PathBuf> { ANDROID_DIRS.get().and_then(|d| d.0.clone().or_else(|| d.1.clone())) }

#[cfg(target_os = "android")]
mod imp {
    use super::{deliver, ANDROID_DIRS};
    use std::path::PathBuf;

    /// Files the player can reach: Android/data/<app>/files (visible over USB), else the private folder.
    fn files_dir() -> PathBuf {
        let d = ANDROID_DIRS.get().and_then(|d| d.1.clone().or_else(|| d.0.clone())).unwrap_or_else(|| PathBuf::from("."));
        let _ = std::fs::create_dir_all(&d);
        d
    }
    fn where_() -> &'static str { "Android/data/com.codeforge.game/files" }

    pub fn export(file_name: &str, text: &str) -> Result<String, String> {
        let p = files_dir().join(file_name);
        std::fs::write(&p, text).map_err(|e| format!("Couldn't save the file: {}", e))?;
        // also try the shared Downloads folder (works on many phones)
        let dl = PathBuf::from("/storage/emulated/0/Download").join(file_name);
        let shared = std::fs::write(&dl, text).is_ok();
        Ok(if shared { format!("Saved to Downloads/{}", file_name) } else { format!("Saved {} in {} (connect the phone by USB to copy it)", file_name, where_()) })
    }

    pub fn pick_import() {
        let mut found: Vec<(std::time::SystemTime, PathBuf)> = vec![];
        for dir in [files_dir(), PathBuf::from("/storage/emulated/0/Download")] {
            if let Ok(rd) = std::fs::read_dir(&dir) {
                for e in rd.flatten() {
                    let p = e.path();
                    if p.extension().is_some_and(|x| x == "codeforge") && let Ok(m) = e.metadata().and_then(|m| m.modified()) { found.push((m, p)); }
                }
            }
        }
        found.sort();
        match found.last() {
            Some((_, p)) => deliver(std::fs::read_to_string(p).map_err(|_| "Couldn't read that file.".to_string())),
            None => deliver(Err(format!("No .codeforge file found. Put your exported file in Downloads or in {}, then tap Import again.", where_()))),
        }
    }

    pub fn dropped(_f: &dyn eframe::egui::DroppedFile) {}

    static APP: std::sync::OnceLock<winit::platform::android::activity::AndroidApp> = std::sync::OnceLock::new();

    /// Height (physical pixels) the on-screen keyboard covers at the bottom of a window `window_h` tall,
    /// from the visible content area Android reports; None when Android doesn't say.
    pub fn keyboard_inset_px(window_h: f32) -> Option<f32> {
        let r = APP.get()?.content_rect();
        let covered = window_h - r.bottom as f32;
        (covered > window_h * 0.15 && covered < window_h * 0.8).then_some(covered)
    }
    pub fn set_app(app: winit::platform::android::activity::AndroidApp) { let _ = APP.set(app); }

    /// Show or hide the on-screen keyboard. NativeActivity's own call is unreliable, so this asks
    /// Android's InputMethodManager directly.
    pub fn soft_keyboard(show: bool) {
        let Some(app) = APP.get() else { return };
        let r = (|| -> jni::errors::Result<()> {
            let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut jni::sys::JavaVM)? };
            let mut env = vm.attach_current_thread()?;
            let activity = unsafe { jni::objects::JObject::from_raw(app.activity_as_ptr() as jni::sys::jobject) };
            let name = env.new_string("input_method")?;
            let imm = env.call_method(&activity, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[(&name).into()])?.l()?;
            let window = env.call_method(&activity, "getWindow", "()Landroid/view/Window;", &[])?.l()?;
            let view = env.call_method(&window, "getDecorView", "()Landroid/view/View;", &[])?.l()?;
            if show {
                // SHOW_FORCED (2): NativeActivity has no text view for showSoftInput to target
                env.call_method(&imm, "toggleSoftInput", "(II)V", &[2.into(), 0.into()])?;
            } else {
                let token = env.call_method(&view, "getWindowToken", "()Landroid/os/IBinder;", &[])?.l()?;
                env.call_method(&imm, "hideSoftInputFromWindow", "(Landroid/os/IBinder;I)Z", &[(&token).into(), 0.into()])?;
            }
            Ok(())
        })();
        if r.is_err() { app.show_soft_input(show); }
    }

    pub fn save_png(file_name: &str, img: &image::RgbaImage) -> Result<String, String> {
        let p = PathBuf::from("/storage/emulated/0/Pictures").join(file_name);
        if img.save(&p).is_ok() { return Ok(p.display().to_string()); }
        let p = files_dir().join(file_name);
        img.save(&p).map_err(|e| e.to_string())?;
        Ok(p.display().to_string())
    }
}

// ------------------------------------------------------------------------------------- web
#[cfg(target_arch = "wasm32")]
mod imp {
    use super::deliver;
    use wasm_bindgen::{JsCast, closure::Closure};

    /// Offer bytes as a file download in the browser.
    pub fn download(file_name: &str, bytes: &[u8], mime: &str) -> Result<(), String> {
        let arr = js_sys::Uint8Array::from(bytes);
        let parts = js_sys::Array::new();
        parts.push(&arr.buffer());
        let opts = web_sys::BlobPropertyBag::new();
        opts.set_type(mime);
        let blob = web_sys::Blob::new_with_buffer_source_sequence_and_options(&parts, &opts).map_err(|_| "Couldn't create the file.".to_string())?;
        let url = web_sys::Url::create_object_url_with_blob(&blob).map_err(|_| "Couldn't create the file.".to_string())?;
        let doc = web_sys::window().and_then(|w| w.document()).ok_or("No page")?;
        let a: web_sys::HtmlAnchorElement = doc.create_element("a").map_err(|_| "No page")?.dyn_into().map_err(|_| "No page")?;
        a.set_href(&url);
        a.set_download(file_name);
        a.click();
        let _ = web_sys::Url::revoke_object_url(&url);
        Ok(())
    }

    pub fn export(file_name: &str, text: &str) -> Result<String, String> {
        download(file_name, text.as_bytes(), "text/plain")?;
        Ok(format!("Downloaded {}. Keep it safe and import it on your other device.", file_name))
    }

    fn read_file(file: web_sys::File) {
        wasm_bindgen_futures::spawn_local(async move {
            let r = wasm_bindgen_futures::JsFuture::from(file.text()).await;
            deliver(r.ok().and_then(|v| v.as_string()).ok_or_else(|| "Couldn't read that file.".to_string()));
        });
    }

    pub fn pick_import() {
        let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
        let Ok(el) = doc.create_element("input") else { return };
        let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>() else { return };
        input.set_type("file");
        input.set_accept(".codeforge,text/plain");
        let inp = input.clone();
        let on_change = Closure::once(move || {
            if let Some(f) = inp.files().and_then(|l| l.get(0)) { read_file(f); }
        });
        input.set_onchange(Some(on_change.as_ref().unchecked_ref()));
        on_change.forget();
        input.click();
    }

    pub fn dropped(f: &dyn eframe::egui::DroppedFile) {
        if let Some(file) = f.web_file() { read_file(file.clone()); }
    }

    pub fn save_png(file_name: &str, img: &image::RgbaImage) -> Result<String, String> {
        let mut bytes = std::io::Cursor::new(Vec::new());
        img.write_to(&mut bytes, image::ImageFormat::Png).map_err(|e| e.to_string())?;
        download(file_name, bytes.get_ref(), "image/png")?;
        Ok(format!("your downloads ({})", file_name))
    }

    fn storage() -> Option<web_sys::Storage> { web_sys::window().and_then(|w| w.local_storage().ok().flatten()) }
    pub fn load_text(key: &str) -> Option<String> { storage()?.get_item(key).ok().flatten() }
    pub fn store_text(key: &str, text: &str) -> Result<(), String> {
        storage().ok_or("This browser blocks saving (private mode?)")?.set_item(key, text).map_err(|_| "The browser refused to save (storage full or private mode).".to_string())
    }
}

pub use imp::*;
