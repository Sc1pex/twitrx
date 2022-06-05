use sqlx::{query, Connection, PgConnection, PgPool};

pub struct TestDB {
    url: String,
    db_pool: PgPool,
}

impl TestDB {
    pub async fn new() -> Self {
        let url = gen_url();
        let (pg_url, db_name) = url.rsplit_once("/").unwrap();

        let mut pg_conn = PgConnection::connect(&format!("{}/postgres", pg_url))
            .await
            .unwrap();
        query(&format!(r#"CREATE DATABASE "{}""#, &db_name))
            .execute(&mut pg_conn)
            .await
            .unwrap();

        let db_pool = PgPool::connect(&url).await.unwrap();
        sqlx::migrate!().run(&db_pool).await.unwrap();

        Self { url, db_pool }
    }

    pub fn pool(&self) -> PgPool {
        self.db_pool.clone()
    }

    pub async fn drop_db(&self) {
        let (pg_url, db_name) = self.url.rsplit_once("/").unwrap();

        let mut pg_conn = PgConnection::connect(&format!("{}/postgres", pg_url))
            .await
            .unwrap();
        query(&format!(
            r#"SELECT pg_terminate_backend(pg_stat_activity.pid)
        FROM pg_stat_activity
        WHERE pg_stat_activity.datname = '{}'
        AND pid <> pg_backend_pid()"#,
            db_name
        ))
        .execute(&mut pg_conn)
        .await
        .unwrap();

        query(&format!(r#"DROP DATABASE "{}""#, db_name))
            .execute(&mut pg_conn)
            .await
            .unwrap();
    }
}

fn gen_url() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let rng = thread_rng();
    let suffix = rng.sample_iter(&Alphanumeric).take(16).collect();
    let suffix = String::from_utf8(suffix).unwrap();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL missing from environment.");
    format!("{}_{}", db_url, suffix)
}
