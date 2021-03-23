use crate::prelude::*;
use argon2::verify_encoded as verify;
use std::time::Duration;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct User {
    pub id: u32,
    pub email: String,
    password: String,
    pub is_admin: bool,
}

pub struct Users {
    conn: Box<dyn DBConnection>,
    sess: Box<dyn SessionManager>,
}

impl Users {
    #[cfg(feature = "sqlite-db")]
    fn open_sqlite(path: &str) -> Resut<Self> {
        Users {
            conn: Box::new(rusqlite::Connection::open(path)?),
            sess: Box::new(chashmap::CHashMap::new()),
        }
    }
}

// User API
impl Users {
    pub fn login(&self, form: &impl Deref<Target = Login>) -> Result<String> {
        let form_pwd = &form.password.as_bytes();
        let user = self.conn.get_user_by_email(&form.email)?;
        let user_pwd = &user.password;
        if verify(user_pwd, form_pwd)? {
            let key = self.set_auth_key(user.id)?;
            Ok(key)
        } else {
            raise("Contraseña incorrecta.")
        }
    }
    pub fn logout(&self, session: &Session) -> Result<()> {
        if self.is_auth(session) {
            self.sess.remove(session.id)?;
        }
        Ok(())
    }

    pub fn signup(&self, form: &impl Deref<Target = Signup>) -> Result<()> {
        let email = &form.email;
        let password = &form.password;
        self.create_user(email, password, false)?;
        Ok(())
    }
    pub fn get_by_id(&self, user_id: u32) -> Result<User> {
        self.conn.get_user_by_id(user_id)
    }

    pub fn get_by_email(&self, email: &str) -> Result<User> {
        self.conn.get_user_by_email(email)
    }

    fn set_auth_key(&self, user_id: u32) -> Result<String> {
        let key = rand_string(10);
        self.sess.insert(user_id.into(), key.clone())?;
        Ok(key)
    }

    fn is_auth(&self, session: &Session) -> bool {
        let option = self.sess.get(session.id);
        if let Some(auth_key) = option {
            auth_key == session.auth_key
        } else {
            false
        }
    }

    pub fn create_user(&self, email: &str, password: &str, is_admin: bool) -> Result<()> {
        let password = password.as_bytes();
        let salt = rand_string(10);
        let config = argon2::Config::default();
        let hash = argon2::hash_encoded(password, &salt.as_bytes(), &config).unwrap();
        self.conn.create_user(email, &hash, is_admin)?;
        Ok(())
    }

    fn set_auth_key_for(&self, user_id: u32, time: Duration) -> Result<String> {
        let key = rand_string(10);
        self.sess.insert_for(user_id.into(), key.clone(), time)?;
        Ok(key)
    }
    pub fn login_for(&self, form: &impl Deref<Target = Login>, time: Duration) -> Result<String> {
        let form_pwd = &form.password.as_bytes();
        let user = self.conn.get_user_by_email(&form.email)?;
        let user_pwd = &user.password;
        if verify(user_pwd, form_pwd)? {
            let key = self.set_auth_key_for(user.id, time)?;
            Ok(key)
        } else {
            raise("Contraseña incorrecta.")
        }
    }
}

use rand::random;
fn rand_string(size: usize) -> String {
    let dissallowed = ['\\', '"', '{', '}', '(', ')', '`', '\''];
    (0..)
        .map(|_| random::<u8>())
        .filter(|n| 31 < *n && *n < 126)
        .map(|n| char::from(n))
        .filter(|c| !dissallowed.contains(c))
        .take(size)
        .collect()
}
