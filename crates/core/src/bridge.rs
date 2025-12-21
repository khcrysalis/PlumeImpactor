use std::fmt;

#[uniffi::export(callback_interface)]
pub trait TwoFaCallback: Send + Sync {
    fn get_code(&self) -> String;
}

#[derive(uniffi::Record, Debug)]
pub struct LoginResult {
    pub username: String,
    pub adsid: String,
    pub xcode_gs_token: String,
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn start_account_login(
    username: String,
    password: String,
    config_path: String,
    two_fa: Box<dyn TwoFaCallback>,
) -> Result<LoginResult, FfiError> {
    log::debug!("Logging in user: {}", username);

    let appleid_closure = || Ok((username.clone(), password.clone()));
    let two_fa_closure = || Ok(two_fa.get_code());

    let config = omnisette::AnisetteConfiguration::default()
        .set_configuration_path(std::path::Path::new(&config_path).to_path_buf());

    let a = crate::auth::Account::login(appleid_closure, two_fa_closure, config)
        .await
        .map_err(|e| FfiError::Generic { message: format!("Login failed: {}", e) })?;

    let d = crate::developer::DeveloperSession::using_account(a)
        .await
        .map_err(|e| FfiError::Generic { message: format!("Developer session creation failed: {}", e) })?;

    d.qh_list_teams().await
        .map_err(|e| FfiError::Generic { message: format!("Failed to validate session: {}", e) })?;

    let adsid = d.adsid().clone();
    let xcode_gs_token = d.xcode_gs_token().clone();

    Ok(LoginResult {
        username,
        adsid,
        xcode_gs_token
    })
}

#[uniffi::export]
pub fn start_anisette_setup(
    config_path: String,
) -> Result<omnisette::AnisetteConfiguration, FfiError> {
    let config = omnisette::AnisetteConfiguration::default()
        .set_configuration_path(std::path::Path::new(&config_path).to_path_buf());

    Ok(config)
}

#[derive(Debug, uniffi::Error)]
pub enum FfiError {
    Generic { message: String },
}

impl fmt::Display for FfiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FfiError::Generic { message } => write!(f, "{}", message),
        }
    }
}

#[uniffi::export]
pub fn handle_provider_install() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    _ = rustls::crypto::ring::default_provider().install_default().unwrap();
}
