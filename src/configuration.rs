#[derive(serde::Deserialize)]
pub struct Settings {
    pub port: u16,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut builder = config::Config::builder().add_source(config::File::new(
        "configuration.yml",
        config::FileFormat::Yaml,
    ));

    if let Ok(db_user) = std::env::var("APP_DB_USER") {
        builder = builder.set_override("database.user", db_user)?;
    }

    if let Ok(db_password) = std::env::var("APP_DB_PASSWORD") {
        builder = builder.set_override("database.password", db_password)?;
    }
    builder.build()?.try_deserialize::<Settings>()
}
