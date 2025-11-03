#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod frame;
mod keychain;
mod pages;
mod handlers;

pub const APP_NAME: &str = concat!(env!("CARGO_PKG_NAME"), " – Version ", env!("CARGO_PKG_VERSION"));

#[tokio::main]
async fn main() {
    let _ = wxdragon::main(|_| {
        frame::PlumeFrame::new().show();
    });
}
