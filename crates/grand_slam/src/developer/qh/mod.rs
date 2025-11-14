pub mod account;
pub mod app_groups;
pub mod app_ids;
pub mod certs;
pub mod devices;
pub mod teams;
pub mod profile;

use serde::Deserialize;
use plist::Integer;
use crate::developer::DeveloperSession;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMeta {
    pub creation_timestamp: String,
    pub user_string: Option<String>,
    pub result_string: Option<String>,
    pub result_code: Integer,
    pub http_code: Option<Integer>,
    pub user_locale: String,
    pub protocol_version: String,
    pub request_id: String,
    pub result_url: Option<String>,
    pub response_id: String,
    pub page_number: Option<Integer>,
    pub page_size: Option<Integer>,
    pub total_records: Option<Integer>,
}
