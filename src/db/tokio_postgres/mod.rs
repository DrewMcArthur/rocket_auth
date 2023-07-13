mod sql;

use std::convert::{TryFrom, TryInto};
use tokio_postgres::Client;
use uuid::Uuid;

use crate::{DBConnection, Error, Result, User};

#[rocket::async_trait]
impl DBConnection for Client {
    async fn init(&self) -> Result<()> {
        self.execute(sql::CREATE_TABLE, &[]).await?;
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
        self.execute(
            sql::INSERT_USER,
            &[&uuid.to_string(), &email, &username, &hash, &is_admin],
        )
        .await?;
        Ok(())
    }
    async fn update_user(&self, user: &User) -> Result<()> {
        self.execute(
            sql::UPDATE_USER,
            &[
                &user.uuid,
                &user.email,
                &user.username,
                &user.password,
                &user.is_admin,
            ],
        )
        .await?;
        Ok(())
    }
    async fn delete_user_by_uuid(&self, uuid: Uuid) -> Result<()> {
        self.execute(sql::REMOVE_BY_UUID, &[&uuid.to_string()])
            .await?;
        Ok(())
    }
    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        self.execute(sql::REMOVE_BY_EMAIL, &[&email]).await?;
        Ok(())
    }
    async fn get_user_by_uuid(&self, uuid: Uuid) -> Result<User> {
        let user = self
            .query_one(sql::SELECT_BY_UUID, &[&uuid.to_string()])
            .await?;
        user.try_into()
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let user = self.query_one(sql::SELECT_BY_EMAIL, &[&email]).await?;
        user.try_into()
    }

    async fn get_user_by_username(&self, username: &str) -> Result<User> {
        let user = self
            .query_one(sql::SELECT_BY_USERNAME, &[&username])
            .await?;
        user.try_into()
    }
}

impl TryFrom<tokio_postgres::Row> for User {
    type Error = Error;
    fn try_from(row: tokio_postgres::Row) -> Result<User> {
        Ok(User {
            id: row.get(0),
            uuid: row.get(1),
            email: row.get(2),
            username: row.get(3),
            password: row.get(4),
            is_admin: row.get(5),
        })
    }
}
