use crate::prelude::{Result, *};
mod sql;
use sql::*;

use sqlx::postgres::PgPool;

use sqlx::*;
use uuid::Uuid;

#[rocket::async_trait]
impl DBConnection for PgPool {
    async fn init(&self) -> Result<()> {
        query(CREATE_TABLE).execute(self).await?;
        Ok(())
    }
    async fn create_user(
        &self,
        uuid: Uuid,
        email: Option<&str>,
        username: Option<&str>,
        hash: &str,
        is_admin: bool,
    ) -> Result<()> {
        query(INSERT_USER)
            .bind(uuid)
            .bind(email)
            .bind(username)
            .bind(hash)
            .bind(is_admin)
            .execute(self)
            .await?;
        Ok(())
    }
    async fn update_user(&self, user: &User) -> Result<()> {
        query(UPDATE_USER)
            .bind(user.id)
            .bind(&user.email)
            .bind(&user.username)
            .bind(&user.password)
            .bind(user.is_admin)
            .execute(self)
            .await?;

        Ok(())
    }
    async fn delete_user_by_uuid(&self, uuid: Uuid) -> Result<()> {
        query(REMOVE_BY_UUID).bind(uuid).execute(self).await?;
        Ok(())
    }
    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        query(REMOVE_BY_EMAIL).bind(email).execute(self).await?;
        Ok(())
    }
    async fn get_user_by_uuid(&self, uuid: Uuid) -> Result<User> {
        let user = query_as(SELECT_BY_UUID).bind(uuid).fetch_one(self).await?;

        Ok(user)
    }
    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let user = query_as(SELECT_BY_EMAIL)
            .bind(email)
            .fetch_one(self)
            .await?;
        Ok(user)
    }

    async fn get_user_by_username(&self, username: &str) -> Result<User> {
        let user = query_as(SELECT_BY_USERNAME)
            .bind(username)
            .fetch_one(self)
            .await?;
        Ok(user)
    }
}
