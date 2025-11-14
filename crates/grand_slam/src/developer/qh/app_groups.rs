use serde::Deserialize;
use plist::{Dictionary, Value};

use crate::Error;

use crate::{SessionRequestTrait, developer_endpoint};
use super::{DeveloperSession, ResponseMeta};

impl DeveloperSession {
    pub async fn qh_list_app_groups(&self, team_id: &str) -> Result<AppGroupsResponse, Error> {
        let endpoint = developer_endpoint!("/QH65B2/ios/listApplicationGroups.action");

        let mut body = Dictionary::new();
        body.insert("teamId".to_string(), Value::String(team_id.to_string()));

        let response = self.qh_send_request(&endpoint, Some(body)).await?;
        let response_data: AppGroupsResponse = plist::from_value(&Value::Dictionary(response))?;

        Ok(response_data)
    }
    
    pub async fn qh_add_app_group(&self, team_id: &str, name: &str, identifier: &str) -> Result<AppGroupResponse, Error> {
        let endpoint = developer_endpoint!("/QH65B2/ios/addApplicationGroup.action");
        
        let mut body = Dictionary::new();
        body.insert("teamId".to_string(), Value::String(team_id.to_string()));
        body.insert("name".to_string(), Value::String(name.to_string()));
        body.insert("identifier".to_string(), Value::String(identifier.to_string()));
        
        let response = self.qh_send_request(&endpoint, Some(body)).await?;
        let response_data: AppGroupResponse = plist::from_value(&Value::Dictionary(response))?;

        Ok(response_data)
    }
    
    pub async fn qh_get_app_group(&self, team_id: &str, app_group_identifier: &str) -> Result<Option<ApplicationGroup>, Error> {
        let response_data = self.qh_list_app_groups(team_id).await?;
        
        let app_group = response_data.application_group_list.into_iter()
            .find(|group| group.identifier == app_group_identifier);
        
        Ok(app_group)
    }
    
    pub async fn qh_ensure_app_group(&self, team_id: &str, name: &str, identifier: &str) -> Result<ApplicationGroup, Error> {
        if let Some(app_group) = self.qh_get_app_group(team_id, identifier).await? {
            Ok(app_group)
        } else {
            let response = self.qh_add_app_group(team_id, name, identifier).await?;
            Ok(response.application_group)
        }
    }

    pub async fn qh_assign_app_group(&self, team_id: &str, app_id_id: &str, app_group_id: &str) -> Result<ResponseMeta, Error> {
        let endpoint = developer_endpoint!("/QH65B2/ios/assignApplicationGroupToAppId.action");
        
        let mut body = Dictionary::new();
        body.insert("teamId".to_string(), Value::String(team_id.to_string()));
        body.insert("appIdId".to_string(), Value::String(app_id_id.to_string()));
        body.insert("applicationGroups".to_string(), Value::String(app_group_id.to_string()));

        let response = self.qh_send_request(&endpoint, Some(body)).await?;
        let response_data: ResponseMeta = plist::from_value(&Value::Dictionary(response))?;

        Ok(response_data)
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppGroupsResponse {
    pub application_group_list: Vec<ApplicationGroup>,
    #[serde(flatten)]
    pub meta: ResponseMeta,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppGroupResponse {
    pub application_group: ApplicationGroup,
    #[serde(flatten)]
    pub meta: ResponseMeta,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationGroup {
    pub application_group: String, // this is the actual identifier
    pub name: String,
    pub status: String,
    prefix: String,
    pub identifier: String, // this is the group.identifier
}
