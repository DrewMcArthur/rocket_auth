use super::AuthKey;
use super::SessionManager;
use crate::prelude::*;
use chashmap::CHashMap;
use uuid::Uuid;

impl SessionManager for CHashMap<Uuid, AuthKey> {
    #[throws(Error)]
    fn insert(&self, uuid: Uuid, key: String) {
        self.insert(uuid, key.into());
    }

    #[throws(Error)]
    fn remove(&self, uuid: Uuid) {
        self.remove(&uuid);
    }

    fn get(&self, uuid: Uuid) -> Option<String> {
        let key = self.get(&uuid)?;
        Some(key.secret.clone())
    }

    #[throws(Error)]
    fn clear_all(&self) {
        self.clear();
    }

    #[throws(Error)]
    fn insert_for(&self, uuid: Uuid, key: String, time: Duration) {
        let key = AuthKey {
            expires: time.as_secs() as i64,
            secret: key,
        };
        self.insert(uuid, key);
    }

    #[throws(Error)]
    fn clear_expired(&self) {
        let time = now();
        self.retain(|_, auth_key| auth_key.expires > time);
    }
}
