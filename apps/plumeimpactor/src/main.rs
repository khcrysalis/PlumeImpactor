#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod frame;
mod keychain;
mod pages;
mod handlers;

pub const APP_NAME: &str = concat!(env!("CARGO_PKG_NAME"), " – Version ", env!("CARGO_PKG_VERSION"));

#[tokio::main]
async fn main() {
    // its very picky
    _ = rustls::crypto::ring::default_provider().install_default().unwrap();

    let _ = wxdragon::main(|_| {
        frame::PlumeFrame::new().show();
    });
}
