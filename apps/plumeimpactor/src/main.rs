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

use thiserror::Error as ThisError;
use sparkle_updater::Updater;

#[tokio::main]
async fn main() {
    _ = rustls::crypto::ring::default_provider().install_default().unwrap();
    
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        #[cfg(target_os = "macos")]
        let updater = Updater::new();
        #[cfg(target_os = "windows")]
        let updater = Updater::new(
            "https://github.com/khcrysalis/PlumeImpactor/releases/latest/download/appcast-win.xml".into(),
            None,
        );
        
        updater.check_for_updates();
    }

    let _ = wxdragon::main(|_| {
        frame::PlumeFrame::new().show();
    });
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

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Plist error: {0}")]
    Plist(#[from] plist::Error),
    #[error("Idevice error: {0}")]
    Idevice(#[from] idevice::IdeviceError),
    #[error("Core error: {0}")]
    Core(#[from] plume_core::Error),
    #[error("Utils error: {0}")]
    Utils(#[from] plume_utils::Error),
}