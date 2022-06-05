use super::testdb::TestDB;
use actix_web::{
    body::MessageBody,
    dev::{Service, ServiceResponse},
    test::{call_and_read_body_json, TestRequest},
};
use serde::Serialize;
use serde_json::Value;

pub async fn test_db() -> TestDB {
    dotenv::dotenv().ok();
    pretty_env_logger::try_init().ok();

    TestDB::new().await
}

pub fn get(url: &str) -> Request {
    Request::GET {
        url: url.to_string(),
    }
}

pub fn post<T: Serialize>(url: &str, body: T) -> Request {
    Request::POST {
        url: url.to_string(),
        json: serde_json::to_value(body).unwrap(),
    }
}

pub enum Request {
    GET { url: String },
    POST { url: String, json: Value },
}

impl Request {
    pub async fn send<S, B>(&self, server: &S) -> Value
    where
        S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
        B: MessageBody,
    {
        match self {
            Request::GET { url } => {
                let req = TestRequest::get().uri(&url).to_request();
                return call_and_read_body_json(server, req).await;
            }
            Request::POST { url, json } => {
                let req = TestRequest::post().uri(&url).set_json(json).to_request();
                return call_and_read_body_json(server, req).await;
            }
        }
    }
}
