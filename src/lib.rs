use log::{info, warn};
use notifica_common::configuration::WebhookDto;
use reqwest::StatusCode;
use serde_json::json;
use uuid::Uuid;

pub struct Notifica;

impl Notifica {
    pub async fn send_message(body: WebhookDto, event: &str, tenant_id: Uuid) {
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
