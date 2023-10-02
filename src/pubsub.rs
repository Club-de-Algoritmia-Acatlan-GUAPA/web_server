use axum::async_trait;
use bb8::{CustomizeConnection, Pool};
use bb8_redis::RedisMultiplexedConnectionManager;
use redis::{
    aio::{Connection, PubSub},
    Client, ErrorKind, RedisError,
};
use secrecy::ExposeSecret;

use crate::configuration::RedisSettings;

pub type RedisPool = bb8::Pool<RedisMultiplexedConnectionManager>;
pub struct RedisPubSub(pub redis::aio::PubSub);

pub struct CustomRedisConnection {
    inner: Connection,
    pub_sub: Option<PubSub>,
}

impl CustomRedisConnection {
    pub fn new(conn: Connection) -> Self {
        Self {
            inner: conn,
            pub_sub: None,
        }
    }
}
pub struct CustomRedisConnectionManager {
    uri: String,
    //conn_properties: ConnectionProperties,
}

#[derive(Debug)]
struct Customizer;

#[async_trait]
impl CustomizeConnection<CustomRedisConnection, RedisError> for Customizer {
    async fn on_acquire(&self, conn: &mut CustomRedisConnection) -> Result<(), RedisError> {
        //let channel = conn.inner.into_pubsub();
        // Confirm acknowledge of queue
        //channel
        //    .confirm_select(ConfirmSelectOptions::default())
        //    .await?;

        //conn.channel = Some(channel);
        Ok(())
    }
}
#[async_trait]
impl bb8::ManageConnection for CustomRedisConnectionManager {
    type Connection = CustomRedisConnection;
    type Error = redis::RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let client = Client::open("redis://localhost:6379")?;
        let conn = client.get_async_connection().await?;
        Ok(CustomRedisConnection::new(conn))
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let pong: String = redis::cmd("PING").query_async(&mut conn.inner).await?;

        match pong.as_str() {
            "PONG" => Ok(()),
            _ => Err((ErrorKind::ResponseError, "ping request").into()),
        }
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        //let broken_states = vec![ConnectionState::Closed, ConnectionState::Error];
        //broken_states.contains(&conn.inner.status().state())
        false
    }
}
pub fn pubsub_connection(config: &RedisSettings) -> RedisPubSub {
    use async_global_executor::block_on;

    block_on(async {
        let c = Client::open(config.uri.expose_secret().clone()).unwrap();

        let mut h = c.get_async_connection().await.unwrap().into_pubsub();

        h.subscribe("channel_1").await.unwrap();

        RedisPubSub(h)
    })
}
