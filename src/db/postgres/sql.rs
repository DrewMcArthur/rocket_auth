pub(crate) const CREATE_TABLE: &str = "
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR (254) UNIQUE NOT NULL,
    username VARCHAR (254) UNIQUE NOT NULL,
	password VARCHAR ( 255 ) NOT NULL,
    is_admin BOOL DEFAULT FALSE
);
";

pub(crate) const INSERT_USER: &str = "
INSERT INTO users (uuid, email, username, password, is_admin) VALUES ($1, $2, $3, $4, $5);
";

pub(crate) const UPDATE_USER: &str = "
UPDATE users SET
    email = $2,
    username = $3,
    password = $4,
    is_admin = $5
WHERE
    uuid = $1
";

pub(crate) const SELECT_BY_UUID: &str = "
SELECT * FROM users WHERE uuid = $1;
";

pub(crate) const SELECT_BY_EMAIL: &str = "
SELECT * FROM users WHERE email = $1;
";

pub(crate) const SELECT_BY_USERNAME: &str = "
SELECT * FROM users WHERE username = $1;
";

pub(crate) const REMOVE_BY_UUID: &str = "
DELETE FROM users WHERE uuid =$1;
";
pub(crate) const REMOVE_BY_EMAIL: &str = "
DELETE FROM users WHERE email =$1;
";
