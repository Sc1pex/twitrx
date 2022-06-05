use crate::{hello, users, AppData, UserInfo};
use actix_web::{middleware::Logger, test, web, App};
use assert_json_diff::{assert_json_eq, assert_json_include};
use helpers::*;
use serde_json::{json, Value};

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

    let req = test::TestRequest::get().uri("/users").to_request();
    let resp: Value = test::call_and_read_body_json(&server, req).await;
    assert_json_eq!(resp, json!([]));

    let user = UserInfo {
        username: "bob".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(json!(user))
        .to_request();
    let resp: Value = test::call_and_read_body_json(&server, req).await;
    assert_json_eq!(resp, json!(user));

    let req = test::TestRequest::get().uri("/users").to_request();
    let resp: Value = test::call_and_read_body_json(&server, req).await;
    assert_json_include!(actual: resp, expected: json!([user]));

    db.drop_db().await;
}
