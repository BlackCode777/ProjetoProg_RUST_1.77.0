use std::env;

#[derive(Clone, Debug)]
pub struct Settings {
    pub host: String,
    pub port: u16,
    pub db_url: String,
    pub db_max_conns: u32,
}

impl Settings {
    pub fn from_env() -> Self {
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(8080);
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL não definido no .env");
        let db_max_conns = env::var("DB_MAX_CONNS").ok().and_then(|v| v.parse().ok()).unwrap_or(10);

        Self { host, port, db_url, db_max_conns }
    }
}
