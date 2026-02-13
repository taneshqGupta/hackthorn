use anyhow::Result;
use reqwest::multipart;
use serde_json::Value;
use std::collections::HashMap;
use base64::prelude::*;

#[derive(Debug, Clone)]
pub struct CloudinaryConfig {
    pub cloud_name: String,
    pub api_key: String,
    pub api_secret: String,
}

impl CloudinaryConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            cloud_name: std::env::var("CLOUDINARY_CLOUD_NAME")
                .map_err(|_| anyhow::anyhow!("CLOUDINARY_CLOUD_NAME not set"))?,
            api_key: std::env::var("CLOUDINARY_API_KEY")
                .map_err(|_| anyhow::anyhow!("CLOUDINARY_API_KEY not set"))?,
            api_secret: std::env::var("CLOUDINARY_API_SECRET")
                .map_err(|_| anyhow::anyhow!("CLOUDINARY_API_SECRET not set"))?,
        })
    }
}

pub struct CloudinaryService {
    config: CloudinaryConfig,
    client: reqwest::Client,
}

impl CloudinaryService {
    pub fn new(config: CloudinaryConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    pub async fn upload_image(&self, base64_data: &str, public_id: Option<String>) -> Result<String> {
        tracing::info!("Cloudinary upload_image called");
        
        // Remove data URL prefix if present
        let image_data = if base64_data.starts_with("data:") {
            tracing::info!("Removing data URL prefix from base64");
            base64_data.split(',').nth(1).unwrap_or(base64_data)
        } else {
            tracing::info!("No data URL prefix found");
            base64_data
        };

        // Decode base64 to bytes
        tracing::info!("Decoding base64 to bytes, length: {}", image_data.len());
        let image_bytes = BASE64_STANDARD.decode(image_data)
            .map_err(|e| {
                tracing::error!("Base64 decode failed: {}", e);
                anyhow::anyhow!("Failed to decode base64: {}", e)
            })?;
        tracing::info!("Decoded to {} bytes", image_bytes.len());

        // Generate timestamp for signed upload
        let timestamp = chrono::Utc::now().timestamp();
        let timestamp_str = timestamp.to_string();
        tracing::info!("Generated timestamp: {}", timestamp_str);

        // Create parameters for signature
        let mut params_for_signature = std::collections::HashMap::new();
        params_for_signature.insert("timestamp", timestamp_str.as_str());
        params_for_signature.insert("folder", "profile_pictures");
        params_for_signature.insert("transformation", "c_fill,w_300,h_300,f_auto,q_auto");
        
        if let Some(ref id) = public_id {
            tracing::info!("Using public_id: {}", id);
            params_for_signature.insert("public_id", id);
        }

        // Generate signature
        tracing::info!("Generating signature");
        let signature = self.generate_signature(&params_for_signature)?;
        tracing::info!("Signature generated successfully");

        // Create multipart form with signed parameters
        tracing::info!("Creating multipart form");
        let mut form = multipart::Form::new()
            .part("file", multipart::Part::bytes(image_bytes).file_name("profile.jpg"))
            .text("timestamp", timestamp_str)
            .text("api_key", self.config.api_key.clone())
            .text("signature", signature)
            .text("folder", "profile_pictures")
            .text("transformation", "c_fill,w_300,h_300,f_auto,q_auto");

        // Add public_id if provided
        if let Some(id) = public_id {
            form = form.text("public_id", id);
        }

        let url = format!("https://api.cloudinary.com/v1_1/{}/image/upload", self.config.cloud_name);
        tracing::info!("Uploading to URL: {}", url);

        let response = self.client
            .post(&url)
            .multipart(form)
            .send()
            .await?;

        let status = response.status();
        tracing::info!("HTTP response status: {}", status);

        if status.is_success() {
            let json: Value = response.json().await?;
            tracing::info!("Cloudinary response received");
            
            let secure_url = json["secure_url"]
                .as_str()
                .ok_or_else(|| {
                    tracing::error!("No secure_url in Cloudinary response: {:?}", json);
                    anyhow::anyhow!("No secure_url in Cloudinary response")
                })?;

            tracing::info!("Upload successful, URL: {}", secure_url);
            Ok(secure_url.to_string())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            tracing::error!("Cloudinary upload failed with status {}: {}", status, error_text);
            Err(anyhow::anyhow!("Cloudinary upload failed: {}", error_text))
        }
    }

    fn generate_signature(&self, params: &HashMap<&str, &str>) -> Result<String> {
        use std::collections::BTreeMap;
        
        // Sort parameters for signature generation (exclude api_key and signature)
        let sorted_params: BTreeMap<_, _> = params.iter()
            .filter(|(k, _)| **k != "api_key" && **k != "signature")
            .collect();
        
        // Create query string
        let query_string = sorted_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        
        let to_sign = format!("{}{}", query_string, self.config.api_secret);
        
        // Generate SHA1 hash (Cloudinary requires SHA1)
        use sha1::{Sha1, Digest};
        let mut hasher = Sha1::new();
        hasher.update(to_sign.as_bytes());
        let hash = hasher.finalize();
        
        Ok(format!("{:x}", hash))
    }
}
