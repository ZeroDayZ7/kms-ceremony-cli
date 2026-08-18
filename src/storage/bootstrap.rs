use crate::crypto::keys::EncryptedContainer;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct BootstrapManifest {
    pub id: Uuid,
    pub version: u32,
    pub created_at: DateTime<Utc>,
    pub sss_shares_total: u8,
    pub sss_threshold: u8,
    pub encrypted_storage_key: EncryptedContainer,
}

impl BootstrapManifest {
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let json_data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json_data)?;
        Ok(())
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let manifest = serde_json::from_str(&content)?;
        Ok(manifest)
    }
}
