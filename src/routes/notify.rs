use std::convert::Infallible;

use async_stream::try_stream;
use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
};
use futures::Stream;
use serde_json::{from_str, Value};
use sqlx::postgres::PgListener;

use crate::startup::AppState;

pub async fn event_stream(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut listener = PgListener::connect_with(&state.pool)
        .await
        .expect("Failed to connect to pg listener");
    listener
        .listen_all(vec!["submission_channel"])
        .await
        .expect("Failed to listen to channels");

    Sse::new(try_stream! {
        loop {
            match listener.recv().await {
                Ok(notif) => {
                    let payload : Value= from_str(notif.payload()).unwrap();

                    let event = Event::default()
                        .data(payload["status"].as_str().unwrap())
                        .event(payload["submission_id"].as_str().unwrap());
                    yield event
                }
                Err(e) => {dbg!(e);}
            }
        }

    })
    .keep_alive(KeepAlive::default())
}
