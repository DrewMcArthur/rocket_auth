#[cfg(feature = "sqlx-postgres")]
mod postgres;

#[cfg(feature = "sqlx-mysql")]
mod mysql;
#[cfg(any(feature = "sqlx-sqlite", feature = "rusqlite"))]
mod sqlite;

#[cfg(feature = "tokio-postgres")]
mod tokio_postgres;

use uuid::Uuid;

use crate::prelude::*;

#[rocket::async_trait]
pub trait DBConnection: Send + Sync {
    async fn init(&self) -> Result<()>;
    async fn create_user(
        &self,
        uuid: Uuid,
        email: Option<&str>,
        username: Option<&str>,
        hash: &str,
        is_admin: bool,
    ) -> Result<(), Error>;
    async fn update_user(&self, user: &User) -> Result<()>;
    async fn delete_user_by_uuid(&self, uuid: Uuid) -> Result<()>;
    async fn delete_user_by_email(&self, email: &str) -> Result<()>;
    async fn get_user_by_uuid(&self, uuid: Uuid) -> Result<User>;
    async fn get_user_by_email(&self, email: &str) -> Result<User>;
    async fn get_user_by_username(&self, username: &str) -> Result<User>;
}

#[rocket::async_trait]
impl<T: DBConnection> DBConnection for std::sync::Arc<T> {
    async fn init(&self) -> Result<()> {
        T::init(self).await
    }
    async fn create_user(
        &self,
        uuid: Uuid,
        email: Option<&str>,
        username: Option<&str>,
        hash: &str,
        is_admin: bool,
    ) -> Result<(), Error> {
        T::create_user(self, uuid, email, username, hash, is_admin).await
    }
    async fn update_user(&self, user: &User) -> Result<()> {
        T::update_user(self, user).await
    }
    async fn delete_user_by_uuid(&self, uuid: Uuid) -> Result<()> {
        T::delete_user_by_uuid(self, uuid).await
    }
    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        T::delete_user_by_email(self, email).await
    }
    async fn get_user_by_uuid(&self, uuid: Uuid) -> Result<User> {
        T::get_user_by_uuid(self, uuid).await
    }
    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        T::get_user_by_email(self, email).await
    }
    async fn get_user_by_username(&self, username: &str) -> Result<User> {
        T::get_user_by_username(self, username).await
    }
}

#[rocket::async_trait]
impl<T: DBConnection> DBConnection for tokio::sync::Mutex<T> {
    async fn init(&self) -> Result<()> {
        self.init().await
    }
    async fn create_user(
        &self,
        uuid: Uuid,
        email: Option<&str>,
        username: Option<&str>,
        hash: &str,
        is_admin: bool,
    ) -> Result<(), Error> {
        self.lock()
            .await
            .create_user(uuid, email, username, hash, is_admin)
            .await
    }
    async fn update_user(&self, user: &User) -> Result<()> {
        self.lock().await.update_user(user).await
    }
    async fn delete_user_by_uuid(&self, uuid: Uuid) -> Result<()> {
        self.lock().await.delete_user_by_uuid(uuid).await
    }
    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        self.lock().await.delete_user_by_email(email).await
    }
    async fn get_user_by_uuid(&self, uuid: Uuid) -> Result<User> {
        self.lock().await.get_user_by_uuid(uuid).await
    }
    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        self.lock().await.get_user_by_email(email).await
    }
    async fn get_user_by_username(&self, username: &str) -> Result<User> {
        self.lock().await.get_user_by_username(username).await
    }
}
