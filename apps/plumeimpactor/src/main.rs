#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod frame;
mod keychain;
mod pages;
mod handlers;

use std::{
    env, 
    fs, 
    path::{Path, PathBuf}
};

#[tokio::main]
async fn main() {
    _ = rustls::crypto::ring::default_provider().install_default().unwrap();

    let _ = wxdragon::main(|_| {
        frame::PlumeFrame::new().show();
    });
}

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Plist error: {0}")]
    Plist(#[from] plist::Error),
    #[error("Idevice error: {0}")]
    Idevice(#[from] idevice::IdeviceError),
    #[error("GrandSlam error: {0}")]
    GrandSlam(#[from] grand_slam::Error),
    #[error("Utils error: {0}")]
    Utils(#[from] utils::Error),
}

pub fn get_data_path() -> PathBuf {
    let base = if cfg!(windows) {
        env::var("APPDATA").unwrap()
    } else {
        env::var("HOME").unwrap() + "/.config"
    };

    let dir = Path::new(&base).join("PlumeImpactor");

    fs::create_dir_all(&dir).ok();
    
    dir
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub fn get_mac_udid() -> Option<String> {
    let exe_dir = env::current_exe().ok()?.parent()?.to_path_buf();
    let udid_path = exe_dir.join("udid");
    
    if !udid_path.exists() {
        return None;
    }
    
    let output = std::process::Command::new(&udid_path)
        .current_dir(&exe_dir)
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }

    let udid = String::from_utf8_lossy(&output.stdout).trim().to_string();
    (!udid.is_empty()).then_some(udid)
}
