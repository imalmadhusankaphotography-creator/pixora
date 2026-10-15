//! Windows only: embed the app icon and version info (VERSIONINFO) into `photocraft.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `PHOTOCRAFT_REQUIRE_WINRES=1` (set by the
//! release workflow) turns it into an error.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/photocraft.ico");
    println!("cargo:rerun-if-env-changed=PHOTOCRAFT_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=PIXORA_WIN_RES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    // PIXORA cross-builds from Linux (packaging/pixora/build-windows.sh) compile the icon and
    // version info with `zig rc` into a .res file; zig's linker takes it as an input.
    if let Some(res) = std::env::var_os("PIXORA_WIN_RES").filter(|v| !v.is_empty()) {
        println!("cargo:rustc-link-arg-bins={}", std::path::Path::new(&res).display());
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/photocraft.ico")
        .set("ProductName", "PIXORA")
        .set("FileDescription", "PIXORA photo editor")
        .set("CompanyName", "Imal Madhusanka")
        .set("LegalCopyright", "Copyright (c) 2026 Imal Madhusanka (PIXORA) and the PhotoCraft authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "PIXORA.exe")
        .set("InternalName", "pixora");
    if let Err(e) = res.compile() {
        if std::env::var_os("PHOTOCRAFT_REQUIRE_WINRES").is_some() {
            panic!("embedding Windows resources failed: {e}");
        }
        println!("cargo:warning=photocraft.exe built without icon/version resources: {e}");
    }
}
