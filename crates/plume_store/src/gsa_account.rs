use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GsaAccount {
    email: String,
    first_name: String,
    adsid: String,
    xcode_gs_token: String,
}

impl GsaAccount {
    pub fn new(email: String, first_name: String, adsid: String, xcode_gs_token: String) -> Self {
        GsaAccount {
            email,
            first_name,
            adsid,
            xcode_gs_token,
        }
    }
    pub fn email(&self) -> &String {
        &self.email
    }
    pub fn first_name(&self) -> &String {
        &self.first_name
    }
    pub fn adsid(&self) -> &String {
        &self.adsid
    }
    pub fn xcode_gs_token(&self) -> &String {
        &self.xcode_gs_token
    }
}

pub async fn account_from_session(
    email: String,
    account: plume_core::auth::Account,
) -> Result<GsaAccount, plume_core::Error> {
    let first_name = account.get_name().0;
    let s = plume_core::developer::DeveloperSession::using_account(account).await?;
    s.qh_list_teams().await?;
    let adsid = s.adsid().clone();
    let xcode_gs_token = s.xcode_gs_token().clone();

    Ok(GsaAccount::new(email, first_name, adsid, xcode_gs_token))
}
