use uuid::Uuid;

use crate::{Login, Users};

#[tokio::test(flavor = "multi_thread")]
async fn happy_path() {
    let users = Users::open_rusqlite(":memory:").expect("error opening sqlite :memory: db");

    let email = "me@gmail.com";
    let username = "user";
    let pw = "Str0ngPw!";

    users
        .create_user(Uuid::new_v4(), Some(email), Some(username), pw, true)
        .await
        .expect("error creating user");

    let email_login = Login {
        email: Some(email.to_string()),
        username: None,
        password: pw.to_string(),
    };
    let email_user = users
        .get_by_login(&email_login)
        .await
        .expect("error fetching user by email");

    let username_login = Login {
        email: None,
        username: Some(username.to_string()),
        password: pw.to_string(),
    };
    let username_user = users
        .get_by_login(&username_login)
        .await
        .expect("error fetching user by username");

    assert_eq!(email_user, username_user);
}
