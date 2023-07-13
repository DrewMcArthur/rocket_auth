pub mod auth;
mod user_impl;
mod users;
use crate::prelude::*;
use argon2::verify_encoded as verify;

use rand::random;
use uuid::Uuid;
pub fn rand_string(size: usize) -> String {
    (0..)
        .map(|_| random::<char>())
        .filter(|c| c.is_ascii())
        .map(char::from)
        .take(size)
        .collect()
}

impl Users {
    fn is_auth(&self, session: &Session) -> bool {
        let option = self.sess.get(session.uuid);
        if let Some(auth_key) = option {
            auth_key == session.auth_key
        } else {
            false
        }
    }

    #[throws(Error)]
    async fn login(&self, form: &Login) -> String {
        let user = self.get_by_login(form).await?;
        let user_pwd = &user.password;
        let form_pwd = &form.password.as_bytes();
        if verify(user_pwd, form_pwd)? {
            self.set_auth_key(user.uuid)?
        } else {
            throw!(Error::UnauthorizedError)
        }
    }

    #[throws(Error)]
    fn logout(&self, session: &Session) {
        if self.is_auth(session) {
            self.sess.remove(session.uuid)?;
        }
    }

    #[throws(Error)]
    fn set_auth_key_for(&self, uuid: Uuid, time: Duration) -> String {
        let key = rand_string(10);
        self.sess.insert_for(uuid, key.clone(), time)?;
        key
    }

    #[throws(Error)]
    fn set_auth_key(&self, uuid: Uuid) -> String {
        let key = rand_string(15);
        self.sess.insert(uuid, key.clone())?;
        key
    }

    #[throws(Error)]
    async fn signup<'a>(&self, form: &Signup) {
        form.validate()?;

        let email = form.email.as_ref().map(|email| email.to_lowercase());
        let username = form.username.as_deref();
        let password = &form.password;

        let result = self
            .create_user(Uuid::new_v4(), email.as_deref(), username, password, false)
            .await;

        match result {
            Ok(_) => (),
            #[cfg(feature = "sqlx")]
            Err(Error::SqlxError(sqlx::Error::Database(error))) => {
                if error.code() == Some("23000".into()) {
                    throw!(Error::EmailAlreadyExists)
                } else {
                    throw!(Error::SqlxError(sqlx::Error::Database(error)))
                }
            }
            Err(error) => {
                throw!(error)
            }
        }
    }

    #[throws(Error)]
    async fn login_for(&self, form: &Login, time: Duration) -> String {
        let user = self.get_by_login(form).await?;
        let user_pwd = &user.password;
        let form_pwd = &form.password.as_bytes();
        if verify(user_pwd, form_pwd)? {
            self.set_auth_key_for(user.uuid, time)?
        } else {
            throw!(Error::UnauthorizedError)
        }
    }
}
