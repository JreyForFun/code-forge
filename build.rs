// Embeds the app icon and version info into the Windows executable.
fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");
    #[cfg(windows)]
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico")
            .set("ProductName", "Code Forge")
            .set("FileDescription", "Code Forge - learn to code in a pixel-art RPG")
            .set("OriginalFilename", "CodeForge.exe");
        if let Err(e) = res.compile() {
            // A missing Windows SDK must not break the build; the exe just keeps the default icon.
            println!("cargo:warning=could not embed icon: {e}");
        }
    }
}
