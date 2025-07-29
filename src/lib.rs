use std::collections::HashMap;

use log::{info, warn};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

pub struct Notifica;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificaBodyEmail {
    pub target_email: Option<String>,
    pub target_name: Option<String>,
    pub sender_name: Option<String>,
    pub subject: Option<String>,
    pub params: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificaBody {
    pub push: Option<HashMap<String, String>>,
    pub email: Option<NotificaBodyEmail>,
    pub request: Option<HashMap<String, String>>,
}

impl Notifica {
    pub async fn send_message(body: NotificaBody, event: &str, tenant_id: Uuid) {
        info!("[NOTIFICA-SDK] Tenant '{tenant_id}' event '{event}'");
        let client = reqwest::Client::new();
        let response: reqwest::Response = client
            .post(format!(
                "https://notifica.flippi.co/webhook/{tenant_id}/{event}"
            ))
            .json(&json!(body))
            .send()
            .await
            .unwrap();

        if response.status() == StatusCode::OK {
            info!("[NOTIFICA-SDK] Request has been accepted");
        } else {
            warn!("[NOTIFICA-SDK] Request hasnt been accepted");
        }
    }
}
