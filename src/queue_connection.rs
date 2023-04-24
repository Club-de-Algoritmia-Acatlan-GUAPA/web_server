use actix_web::{post, web, HttpResponse};
use async_global_executor;
use futures_lite::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::future::Future;
use std::sync::Mutex;

use lapin::{options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties};
use serde_json::ser::to_string;

pub struct SubmitQueue(pub lapin::Channel);

#[derive(Default, Serialize, Deserialize, Debug)]
struct Submission {
    problem: String,
    user: String,
    contest_id: String,
    code_type: String,
}

#[post("/submit")]
async fn submit_job(
    submission: web::Json<Submission>,
    queue_mutex: web::Data<Mutex<SubmitQueue>>,
) -> String {
    let queue = queue_mutex.lock().unwrap();
    println!("F");
    let _confirm = queue
        .0
        .basic_publish(
            "",
            "hello",
            BasicPublishOptions::default(),
            to_string(&submission.into_inner()).unwrap().as_bytes(),
            BasicProperties::default(),
        )
        .await
        .expect("basic_publish")
        .await // Wait for this specific ack/nack
        .expect("publisher-confirms");

    "Hello, world!".to_string()
}

pub fn create_channel() -> Result<SubmitQueue, lapin::Error> {
    async_global_executor::block_on(async {
        let conn =
            Connection::connect("amqp://localhost:5672", ConnectionProperties::default()).await?;

        let channel = conn.create_channel().await?;

        let queue = channel
            .queue_declare(
                "hello",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(SubmitQueue(channel))
    })
}

pub fn config(config: &mut web::ServiceConfig) {
    let queue = web::Data::new(Mutex::new(create_channel().unwrap()));

    config.app_data(queue.clone()).service(submit_job);
}
