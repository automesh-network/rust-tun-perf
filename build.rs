fn main() {
    #[cfg(target_os = "windows")]
    std::fs::copy(
        wintun_file(),
        std::path::Path::new("target")
            .join(std::env::var("PROFILE").unwrap())
            .join("wintun.dll"),
    )
    .unwrap();
}

#[cfg(target_os = "windows")]
pub fn wintun_file() -> std::path::PathBuf {
    if cfg!(target_arch = "x86") {
        "wintun/bin/x86/wintun.dll"
    } else if cfg!(target_arch = "x86_64") {
        "wintun/bin/amd64/wintun.dll"
    } else if cfg!(target_arch = "arm") {
        "wintun/bin/arm/wintun.dll"
    } else if cfg!(target_arch = "aarch64") {
        "wintun/bin/arm64/wintun.dll"
    } else {
        panic!("Unsupported architecture")
    }
    .into()
}
