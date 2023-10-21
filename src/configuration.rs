use std::time::Duration;

use anyhow::Result;
use config::{Config, FileFormat};
use dotenvy::dotenv;
use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::{PgConnectOptions, PgSslMode};

use crate::{domain::email::Email, email_client::EmailClient};

const CONFIGURATION_DIRECTORY: &str = "CONFIGURATION_DIRECTORY";
const CONFIGURATION_FILE: &str = "CONFIGURATION_FILE";

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub app: AppSettings,
    pub redis: RedisSettings,
    pub database: DatabaseSettings,
    pub email_client: EmailClientSettings,
    pub rabbitmq: RabbitMqSettings,
    pub cookies: CookiesSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

#[derive(serde::Deserialize, Clone)]
pub struct RedisSettings {
    pub uri: Secret<String>,
    pub secret: Secret<String>,
}

#[derive(serde::Deserialize, Clone)]
pub struct RabbitMqSettings {
    pub uri: Secret<String>,
    pub queue: Secret<String>,
}

#[derive(serde::Deserialize, Clone)]
pub struct EmailClientSettings {
    pub host: String,
    pub sender: String,
    pub authorization_token: Secret<String>,
    pub timeout_milliseconds: u64,
}

#[derive(serde::Deserialize, Clone)]
pub struct AppSettings {
    pub base_url: String,
    pub application_port: u16,
    pub domain: String,
    pub is_prod: bool,
}

#[derive(serde::Deserialize, Clone)]
pub struct CookiesSettings {
    pub name: String,
    pub domain: String,
    pub path: String,
    pub http_only: bool,
    pub same_site_policy: String,
    pub persistence_policy: String,
    pub secure: bool,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret().as_str())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}
impl EmailClientSettings {
    pub fn client(&self) -> Result<EmailClient> {
        let sender = self.sender()?;

        EmailClient::new(
            self.host.clone(),
            sender,
            self.authorization_token.clone(),
            self.timeout(),
        )
    }

    fn sender(&self) -> Result<Email> {
        self.sender.parse::<Email>()
    }

    fn timeout(&self) -> Duration {
        Duration::from_millis(self.timeout_milliseconds)
    }
}
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let (is_prod, config_dir, config_file);
    match dotenv() {
        Ok(_) => {
            is_prod = dotenvy::var("IS_PROD")
                .expect("IS_PROD is not set")
                .parse::<bool>()
                .unwrap();
            config_dir =
                dotenvy::var(CONFIGURATION_DIRECTORY).expect("CONFIGURATION_DIRECTORY is not set");
            config_file = dotenvy::var(CONFIGURATION_FILE).expect("CONFIGURATION_FILE is not set");
        },
        Err(_) => {
            is_prod = std::env::var("IS_PROD")
                .expect("IS_PROD is not set")
                .parse::<bool>()
                .unwrap();
            config_dir =
                std::env::var(CONFIGURATION_DIRECTORY).expect("CONFIGURATION_DIRECTORY is not set");
            config_file = std::env::var(CONFIGURATION_FILE).expect("CONFIGURATION_FILE is not set");
        },
    }

    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join(config_dir);

    let settings = Config::builder()
        .add_source(
            config::File::from(configuration_directory.join(config_file)).format(FileFormat::Yaml),
        )
        .build()?;

    settings.try_deserialize::<Settings>().map(|s| Settings {
        app: AppSettings { is_prod, ..s.app },
        ..s
    })
}
