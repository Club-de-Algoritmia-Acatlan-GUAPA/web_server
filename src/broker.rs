use std::default::Default;

use anyhow::{bail, Result};
use axum::async_trait;
use bb8::{CustomizeConnection, Pool};
use bb8_lapin::prelude::*;
use lapin::{
    options::*,
    protocol::{AMQPError, AMQPErrorKind, AMQPHardError},
    publisher_confirm::PublisherConfirm,
    types::ShortString,
    BasicProperties, Connection, ConnectionProperties, ConnectionState,
};
use secrecy::{ExposeSecret, Secret};
use serde::Serialize;
use serde_json::ser::to_string;

use crate::configuration::RabbitMqSettings;

#[derive(Debug)]
struct Customizer;

#[derive(Clone)]
pub struct MessageBroker {
    pool: Pool<CustomLapinConnectionManager>,
    queue: Secret<String>,
}

impl MessageBroker {
    pub async fn new(settings: &RabbitMqSettings) -> Self {
        let lapin_manager = CustomLapinConnectionManager::new(
            settings.uri.expose_secret().clone(),
            ConnectionProperties::default(),
        );

        let pool = Pool::builder()
            .connection_customizer(Box::new(Customizer))
            .build(lapin_manager)
            .await
            .expect("build error");
        Self {
            pool,
            queue: settings.queue.clone(),
        }
    }

    pub async fn publish<T: Serialize>(&self, message: T) -> Result<PublisherConfirm> {
        let conn = self.pool.get().await?;

        match &conn.channel {
            Some(channel) => {
                let json = to_string(&message)?;
                dbg!("message to be sent", &json);
                Ok(channel
                    .basic_publish(
                        "",
                        self.queue.expose_secret(),
                        BasicPublishOptions::default(),
                        json.as_bytes(),
                        BasicProperties::default(),
                    )
                    .await?)
            },
            None => {
                bail!("Unable to acquire a new channel")
            },
        }
    }
}

pub struct CustomLapinConnection {
    inner: Connection,
    channel: Option<lapin::Channel>,
}
impl CustomLapinConnection {
    pub fn new(conn: lapin::Connection) -> Self {
        Self {
            inner: conn,
            channel: None,
        }
    }
}

pub struct CustomLapinConnectionManager {
    uri: String,
    conn_properties: ConnectionProperties,
}
impl CustomLapinConnectionManager {
    pub fn new(uri: String, conn_properties: ConnectionProperties) -> Self {
        Self {
            uri,
            conn_properties,
        }
    }
}
#[async_trait]
impl CustomizeConnection<CustomLapinConnection, lapin::Error> for Customizer {
    async fn on_acquire(&self, conn: &mut CustomLapinConnection) -> Result<(), lapin::Error> {
        let channel = conn.inner.create_channel().await?;
        // Confirm acknowledge of queue
        channel
            .confirm_select(ConfirmSelectOptions::default())
            .await?;

        conn.channel = Some(channel);
        Ok(())
    }
}

#[async_trait]
impl bb8::ManageConnection for CustomLapinConnectionManager {
    type Connection = CustomLapinConnection;
    type Error = lapin::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let conn = lapin::Connection::connect(&self.uri, self.conn_properties.clone()).await?;
        Ok(CustomLapinConnection::new(conn))
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let valid_states = vec![
            ConnectionState::Initial,
            ConnectionState::Connecting,
            ConnectionState::Connected,
        ];
        if valid_states.contains(&conn.inner.status().state()) {
            Ok(())
        } else {
            Err(lapin::Error::ProtocolError(AMQPError::new(
                AMQPErrorKind::Hard(AMQPHardError::CONNECTIONFORCED),
                ShortString::from("Invalid connection"),
            )))
        }
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        let broken_states = vec![ConnectionState::Closed, ConnectionState::Error];
        broken_states.contains(&conn.inner.status().state())
    }
}
