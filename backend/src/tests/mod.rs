use crate::{hello, users, AppData, UserInfo};
use actix_web::{middleware::Logger, test, web, App};
use assert_json_diff::{assert_json_eq, assert_json_include};
use helpers::*;
use serde_json::json;

mod helpers;
mod testdb;

#[actix_web::test]
async fn insert_user() {
    let db = test_db().await;
    let server = test::init_service(
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppData { db_pool: db.pool() }))
            .service(hello)
            .service(users),
    )
    .await;

    let resp = get("/users").send(&server).await;
    assert_json_eq!(resp, json!([]));

    let user = UserInfo {
        username: "bob".to_string(),
    };

    let resp = post("/users", &user).send(&server).await;
    assert_json_eq!(resp, json!(user));

    let resp = get("/users").send(&server).await;
    assert_json_include!(actual: resp, expected: json!([user]));

    db.drop_db().await;
}

#[actix_web::test]
async fn insert_user_1() {
    let db = test_db().await;
    let server = test::init_service(
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppData { db_pool: db.pool() }))
            .service(hello)
            .service(users),
    )
    .await;

    let resp = get("/users").send(&server).await;
    assert_json_eq!(resp, json!([]));

    let user = UserInfo {
        username: "alice".to_string(),
    };

    let resp = post("/users", &user).send(&server).await;
    assert_json_eq!(resp, json!(user));

    let resp = get("/users").send(&server).await;
    assert_json_include!(actual: resp, expected: json!([user]));

    db.drop_db().await;
}
