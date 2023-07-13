use super::SessionManager;
use crate::prelude::*;

use redis::{Client, Commands};
use uuid::Uuid;

const YEAR_IN_SECS: usize = 365 * 60 * 60 * 24;

impl SessionManager for Client {
    #[throws(Error)]
    fn insert(&self, uuid: Uuid, key: String) {
        let mut cnn = self.get_connection()?;
        cnn.set_ex(uuid.to_string(), key, YEAR_IN_SECS)?;
    }
    #[throws(Error)]
    fn insert_for(&self, uuid: Uuid, key: String, time: Duration) {
        let mut cnn = self.get_connection()?;
        cnn.set_ex(uuid.to_string(), key, time.as_secs() as usize)?;
    }
    #[throws(Error)]
    fn remove(&self, uuid: Uuid) {
        let mut cnn = self.get_connection()?;
        cnn.del(uuid.to_string())?;
    }
    #[throws(as Option)]
    fn get(&self, uuid: Uuid) -> String {
        let mut cnn = self.get_connection().ok()?;
        let key = cnn.get(uuid.to_string()).ok()?;
        key
    }
    #[throws(Error)]
    fn clear_all(&self) {
        let mut cnn = self.get_connection()?;
        redis::Cmd::new().arg("FLUSHDB").execute(&mut cnn);
    }
    #[throws(Error)]
    fn clear_expired(&self) {}
}
