use std::path::{Path, PathBuf};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use plume_core::Error;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    selected_account: Option<String>,
    accounts: HashMap<String, GsaAccount>,
    path: Option<PathBuf>,
}

impl Settings {
    pub async fn load(path: &Option<PathBuf>) -> Result<Self, Error> {
        if let Some(path) = path {
            if !path.exists() {
                return Ok(Self::default());
            }

            let contents = tokio::fs::read_to_string(path).await?;

            Ok(serde_json::from_str(&contents)?)
        } else {
            Ok(Self::default())
        }
    }

    pub async fn save(&self) -> Result<(), Error> {
        if let Some(path) = &self.path {
            if let Some(parent) = path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }

            let json = serde_json::to_string_pretty(self)?;
            let tmp = path.with_extension("tmp");

            tokio::fs::write(&tmp, json).await?;
            tokio::fs::rename(tmp, path).await?;
        }
        
        Ok(())
    }

    pub fn accounts(&self) -> &HashMap<String, GsaAccount> {
        &self.accounts
    }

    pub async fn accounts_add(&mut self, account: GsaAccount) -> Result<(), Error>{
        let email = account.email.clone();
        self.accounts.insert(email.clone(), account);
        self.selected_account = Some(email);
        self.save().await
    }

    pub async fn accounts_remove(&mut self, email: &str) -> Result<(), Error> {
        self.accounts.remove(email);
        self.save().await
    }

    pub async fn account_select(&mut self, email: &str) -> Result<(), Error> {
        if self.accounts.contains_key(email) {
            self.selected_account = Some(email.to_string());
            self.save().await
        } else {
            Err(Error::Parse)
        }
    }

    pub fn selected_account(&self) -> Option<&GsaAccount> {
        if let Some(email) = &self.selected_account {
            self.accounts.get(email)
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GsaAccount {
    pub email: String,
    pub name: String,
    pub anisette_provider: GsaAnisetteProvider,
    pub userhash: Option<String>,
    pub gs_token: Option<String>,
    pub dsid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GsaAnisetteProvider {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "remote")]
    Remote,
}
