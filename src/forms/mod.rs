use crate::prelude::*;

/// The `Login` form is used along with the [`Auth`] guard to authenticate users.
#[derive(FromForm, Deserialize, Clone, Hash, PartialEq, Eq, Validate)]
pub struct Login {
    #[validate(email)]
    pub email: Option<String>,
    pub username: Option<String>,
    pub(crate) password: String,
}

/// The `Signup` form is used along with the [`Auth`] guard to create new users.
#[derive(FromForm, Deserialize, Clone, PartialEq, Eq, Hash, Validate)]
pub struct Signup {
    #[validate(email)]
    pub email: Option<String>,
    pub username: Option<String>,
    #[validate(
        custom = "is_long",
        custom = "has_number",
        custom = "has_lowercase",
        custom = "has_uppercase"
    )]
    pub(crate) password: String,
}
impl Debug for Signup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Signup {{ email: {:?}, username: {:?}, password: \"*****\" }}",
            self.email, self.username
        )
    }
}
impl Debug for Login {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Login {{ email: {:?}, username: {:?}, password: \"*****\" }}",
            self.email, self.username
        )
    }
}

impl From<Signup> for Login {
    fn from(form: Signup) -> Login {
        Login {
            email: form.email,
            username: form.username,
            password: form.password,
        }
    }
}

impl From<Login> for Signup {
    fn from(form: Login) -> Signup {
        Self {
            email: form.email,
            username: form.username,
            password: form.password,
        }
    }
}

impl<T: Deref<Target = Signup>> From<T> for Login {
    fn from(form: T) -> Login {
        Login {
            email: form.email.clone(),
            username: form.username.clone(),
            password: form.password.clone(),
        }
    }
}
#[throws(ValidationError)]
pub(crate) fn is_secure(password: &str) {
    is_long(password)?;
    has_uppercase(password)?;
    has_lowercase(password)?;
    has_number(password)?;
}

#[throws(ValidationError)]
fn is_long(password: &str) {
    if password.len() < 12 {
        throw!(ValidationError::new(
            "The password must be at least 12 characters long.\n"
        ));
    }
}
#[allow(unreachable_code)]
#[throws(ValidationError)]
fn has_uppercase(password: &str) {
    for c in password.chars() {
        if c.is_uppercase() {
            return;
        }
    }
    throw!(ValidationError::new(
        "The password must include least one uppercase character.\n"
    ));
}
#[allow(unreachable_code)]
#[throws(ValidationError)]
fn has_lowercase(password: &str) {
    for c in password.chars() {
        if c.is_lowercase() {
            return;
        }
    }
    // throw!(Error::UnsafePasswordHasNoLower)
    throw!(ValidationError::new(
        "The password must include least one uppercase character.\n"
    ))
}
#[allow(unreachable_code)]
#[throws(ValidationError)]
fn has_number(password: &str) {
    for c in password.chars() {
        if c.is_numeric() {
            return;
        }
    }
    throw!(ValidationError::new(
        "The password has to contain at least one digit.\n"
    ))
    // throw!(Error::UnsafePasswordHasNoDigit)
}
