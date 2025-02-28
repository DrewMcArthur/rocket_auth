use uuid::Uuid;

use crate::prelude::*;
use std::time::Duration;
pub mod default;

#[cfg(feature = "redis")]
pub mod redis;

pub trait SessionManager: Send + Sync {
    fn insert(&self, uuid: Uuid, key: String) -> Result<()>;
    fn insert_for(&self, uuid: Uuid, key: String, time: Duration) -> Result<()>;
    fn remove(&self, uuid: Uuid) -> Result<()>;
    fn get(&self, uuid: Uuid) -> Option<String>;
    fn clear_all(&self) -> Result<()>;
    fn clear_expired(&self) -> Result<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthKey {
    expires: i64,
    secret: String,
}

impl From<String> for AuthKey {
    fn from(secret: String) -> AuthKey {
        AuthKey {
            expires: 31536000,
            secret,
        }
    }
}

impl From<&str> for AuthKey {
    fn from(secret: &str) -> AuthKey {
        AuthKey {
            expires: 31536000,
            secret: secret.into(),
        }
    }
}
