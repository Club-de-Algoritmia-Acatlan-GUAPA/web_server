use std::convert::Infallible;

use async_stream::try_stream;
use axum::{
    extract::{Path, State},
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

#[derive(serde::Deserialize, Debug)]
struct ContestEvent {
    contest_id: u32,
    status: String,
}
pub async fn contest_event_stream(
    Path(contest_id): Path<u32>,
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {

    let mut listener = PgListener::connect_with(&state.pool)
        .await
        .expect("Failed to connect to pg listener");
    listener
        .listen_all(vec!["contest_channel"])
        .await
        .expect("Failed to listen to channels");

    Sse::new(try_stream! {
        loop {
            match listener.recv().await {
                Ok(notif) => {
                    let payload : ContestEvent = from_str(notif.payload()).unwrap();
                    let event = Event::default()
                        .data(payload.status)
                        .event(format!("contest_{}", contest_id.to_string()));
                    yield event
                }
                Err(e) => {dbg!(e);}
            }
        }

    })
    .keep_alive(KeepAlive::default())
}
