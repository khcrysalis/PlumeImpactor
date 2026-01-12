#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod appearance;
mod defaults;
mod screen;
mod subscriptions;
mod tray;

pub const APP_NAME: &str = "Impactor";
pub const APP_NAME_VERSIONED: &str = concat!("Impactor", " - Version ", env!("CARGO_PKG_VERSION"));

fn main() -> iced::Result {
    env_logger::init();
    let _ = rustls::crypto::ring::default_provider().install_default();

    #[cfg(target_os = "linux")]
    {
        gtk::init().expect("GTK init failed");
    }

    iced::daemon(
        screen::Impactor::new,
        screen::Impactor::update,
        screen::Impactor::view,
    )
    .subscription(screen::Impactor::subscription)
    .title(APP_NAME_VERSIONED)
    .theme(appearance::PlumeTheme::default().to_iced_theme())
    .settings(defaults::default_settings())
    .run()
}
