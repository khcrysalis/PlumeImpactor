#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod app;
mod listeners;
mod login;

#[cfg(target_os = "windows")]
use std::sync::{
    Arc,
    atomic::{AtomicIsize, Ordering},
};
use std::{
    cell::RefCell,
    env, fs,
    path::{Path, PathBuf},
    rc::Rc,
    sync::mpsc as std_mpsc,
};

use eframe::NativeOptions;
use eframe::egui;
use tokio::sync::mpsc;
use tray_icon::{Icon, TrayIcon, TrayIconBuilder, menu::MenuEvent};
#[cfg(target_os = "windows")]
use windows_sys::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{SW_RESTORE, SW_SHOW, SetForegroundWindow, ShowWindow},
};

pub const APP_NAME: &str = concat!("Impactor – Version ", env!("CARGO_PKG_VERSION"));

fn load_tray_icon() -> Icon {
    let bytes = include_bytes!("./tray.png");
    let image = image::load_from_memory(bytes)
        .expect("tray.png is invalid")
        .to_rgba8();
    let (width, height) = image.dimensions();
    Icon::from_rgba(image.into_raw(), width, height).expect("tray icon data invalid")
}

#[tokio::main]
async fn main() -> eframe::Result<()> {
    env_logger::init();
    _ = rustls::crypto::ring::default_provider()
        .install_default()
        .unwrap();

    #[cfg(target_os = "linux")]
    {
        // wayland is so fucking broken
        unsafe {
            env::set_var("WINIT_UNIX_BACKEND", "x11");
            env::remove_var("WAYLAND_DISPLAY");
            env::remove_var("WAYLAND_SOCKET");
        }
    }

    #[cfg(target_os = "linux")]
    {
        gtk::init().expect("GTK init failed");
    }

    let (tx, rx) = mpsc::unbounded_channel();
    listeners::spawn_usbmuxd_listener(tx.clone());
    listeners::spawn_store_handler(tx.clone());

    let mut options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([540.0, 400.0])
            .with_resizable(false),
        run_and_return: true,
        ..Default::default()
    };

    #[cfg(target_os = "macos")]
    {
        options.viewport.icon = Some(std::sync::Arc::new(egui::IconData::default()));
    }

    #[cfg(not(target_os = "macos"))]
    {
        let icon_bytes: &[u8] = include_bytes!(
            "../../../package/linux/icons/hicolor/32x32/apps/dev.khcrysalis.PlumeImpactor.png"
        );
        let d = eframe::icon_data::from_png_bytes(icon_bytes).expect("The icon data must be valid");
        options.viewport.icon = Some(std::sync::Arc::new(d));
    }

    let tray = Rc::new(RefCell::new(None::<TrayIcon>));
    #[cfg(target_os = "windows")]
    let win32_hwnd = Arc::new(AtomicIsize::new(0));

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|ctx| {
            #[cfg(target_os = "windows")]
            let tray_menu_events = setup_tray(&tray, &ctx.egui_ctx, win32_hwnd.clone());
            #[cfg(not(target_os = "windows"))]
            let tray_menu_events = setup_tray(&tray, &ctx.egui_ctx);

            Ok(Box::new(app::ImpactorApp {
                receiver: Some(rx),
                tray_menu_events: Some(tray_menu_events),
                tray_icon: tray.clone(),
                sender: Some(tx),
                #[cfg(target_os = "windows")]
                win32_hwnd: Some(win32_hwnd.clone()),
                ..Default::default()
            }))
        }),
    )
}

#[cfg(target_os = "windows")]
fn setup_tray(
    tray: &Rc<RefCell<Option<TrayIcon>>>,
    ctx: &egui::Context,
    win32_hwnd: Arc<AtomicIsize>,
) -> std_mpsc::Receiver<MenuEvent> {
    let icon = load_tray_icon();

    let tray_icon = TrayIconBuilder::new()
        .with_icon(icon)
        .with_tooltip(APP_NAME)
        .build()
        .unwrap();

    let (menu_tx, menu_rx) = std_mpsc::channel();

    MenuEvent::set_event_handler(Some({
        let ctx = ctx.clone();
        move |event: MenuEvent| {
            if event.id.as_ref() == "open" {
                restore_window_from_tray(win32_hwnd.load(Ordering::Acquire));
            }

            let _ = menu_tx.send(event);
            ctx.request_repaint();
        }
    }));

    tray.borrow_mut().replace(tray_icon);

    menu_rx
}

#[cfg(target_os = "windows")]
fn restore_window_from_tray(hwnd: isize) {
    if hwnd == 0 {
        return;
    }

    unsafe {
        _ = ShowWindow(hwnd as HWND, SW_SHOW);
        _ = ShowWindow(hwnd as HWND, SW_RESTORE);
        _ = SetForegroundWindow(hwnd as HWND);
    }
}

#[cfg(not(target_os = "windows"))]
fn setup_tray(
    tray: &Rc<RefCell<Option<TrayIcon>>>,
    ctx: &egui::Context,
) -> std_mpsc::Receiver<MenuEvent> {
    let icon = load_tray_icon();

    let tray_icon = TrayIconBuilder::new()
        .with_icon(icon)
        .with_tooltip(APP_NAME)
        .build()
        .unwrap();

    let (menu_tx, menu_rx) = std_mpsc::channel();

    MenuEvent::set_event_handler(Some({
        let ctx = ctx.clone();
        move |event: MenuEvent| {
            let _ = menu_tx.send(event);
            ctx.request_repaint();
        }
    }));

    tray.borrow_mut().replace(tray_icon);

    menu_rx
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
