// Embeds the app icon and version info into the Windows executable.
fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico")
            .set("ProductName", "Pixel Learn")
            .set("FileDescription", "Pixel Learn - learn to code in a pixel-art RPG")
            .set("OriginalFilename", "PixelLearn.exe");
        if let Err(e) = res.compile() {
            // A missing Windows SDK must not break the build; the exe just keeps the default icon.
            println!("cargo:warning=could not embed icon: {e}");
        }
    }
}
