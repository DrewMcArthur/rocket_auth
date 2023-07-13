use uuid::Uuid;

use crate::{Login, Users};

#[tokio::test(flavor = "multi_thread")]
async fn happy_path() {
    let users = Users::open_rusqlite(":memory:").unwrap();
    users
        .create_user(
            Uuid::new_v4(),
            Some("me@gmail.com"),
            Some("user"),
            "Str0ngPw!",
            true,
        )
        .await
        .expect("error creating user");

    let email_login = Login {
        email: Some("me@gmail.com".to_string()),
        username: None,
        password: "Str0ngPw!".to_string(),
    };
    let email_user = users.get_by_login(&email_login);

    let username_login = Login {
        email: None,
        username: Some("user".to_string()),
        password: "Str0ngPw!".to_string(),
    };
    let username_user = users.get_by_login(&username_login);

    assert_eq!(email_user.await.unwrap(), username_user.await.unwrap());
}
