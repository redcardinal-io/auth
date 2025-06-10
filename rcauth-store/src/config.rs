pub struct Config {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub pool_size: u32,
}

impl Config {
    pub fn new(
        host: String,
        port: u16,
        user: String,
        password: String,
        database: String,
        pool_size: u32,
    ) -> Self {
        Self {
            host,
            port,
            user,
            password,
            database,
            pool_size,
        }
    }

    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}?sslmode=disable",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}
