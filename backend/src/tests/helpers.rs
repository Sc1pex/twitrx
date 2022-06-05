use super::testdb::TestDB;

pub async fn test_db() -> TestDB {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    TestDB::new().await
}
