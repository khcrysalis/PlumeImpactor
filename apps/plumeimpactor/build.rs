#[cfg(windows)]
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let target = std::env::var("TARGET").unwrap_or_default();

    if target.contains("windows") {
        let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
        embed_windows_manifest(&pkg_name);
    }
}

#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
fn embed_windows_manifest(name: &str) {
    use embed_manifest::manifest::{ActiveCodePage, Setting, SupportedOS::*};
    use embed_manifest::{embed_manifest, new_manifest};

    let manifest = new_manifest(name)
        .supported_os(Windows7..=Windows10)
        .active_code_page(ActiveCodePage::Utf8)
        .heap_type(embed_manifest::manifest::HeapType::SegmentHeap)
        .dpi_awareness(embed_manifest::manifest::DpiAwareness::PerMonitorV2)
        .long_path_aware(Setting::Enabled);

    if let Err(e) = embed_manifest(manifest) {
        println!("cargo:warning=Failed to embed manifest: {e}");
        println!("cargo:warning=The application will still work but may lack optimal Windows theming");
    }
}
