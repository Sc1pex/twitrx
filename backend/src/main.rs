use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    middleware::Logger,
    post, web, App, HttpResponse, HttpServer, Responder,
};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{query, query_as, PgPool};
use thiserror::Error;
use uuid::Uuid;

#[post("/users")]
async fn hello(
    data: web::Data<AppData>,
    info: web::Json<UserInfo>,
) -> Result<impl Responder, Error> {
    let q = query!(
        "insert into users(id, username) values ($1, $2)",
        Uuid::new_v4(),
        info.username,
    )
    .execute(&data.db_pool)
    .await?;

    info!("{:?}", q);

    Ok(info)
}

#[derive(Deserialize, Serialize)]
struct UserInfo {
    username: String,
}

#[get("/users")]
async fn users(data: web::Data<AppData>) -> Result<impl Responder, Error> {
    let q = query_as!(User, "select * from users")
        .fetch_all(&data.db_pool)
        .await?;

    Ok(web::Json(json!(q)))
}

struct AppData {
    db_pool: PgPool,
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let db_url = std::env::var("DATABASE_URL").expect("No database url in .env");
    let db_pool = PgPool::connect(&db_url).await?;
    sqlx::migrate!().run(&db_pool).await?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppData {
                db_pool: db_pool.clone(),
            }))
            .service(hello)
            .service(users)
    })
    .bind(("127.0.0.1", 42069))?
    .run()
    .await?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct User {
    id: Uuid,
    username: String,
}

#[derive(Error, Debug)]
enum Error {
    #[error("Io error {0}")]
    Io(#[from] std::io::Error),

    #[error("Actix error {0}")]
    Actix(#[from] actix_web::Error),

    #[error("Sqlx error {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Sqlx migration error {0}")]
    SqlxMigration(#[from] sqlx::migrate::MigrateError),
}

impl actix_web::error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(test)]
mod tests;
