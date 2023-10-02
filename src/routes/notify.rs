use std::convert::Infallible;

use async_stream::try_stream;
use axum::response::sse::{Event, KeepAlive, Sse};
use futures::Stream;
use futures_util::StreamExt as _;
use redis::Client;

pub async fn event_stream() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let c = Client::open("redis://localhost:6379").unwrap();

    let mut pubsub = c.get_async_connection().await.unwrap().into_pubsub();

    pubsub.subscribe("channel_1").await.unwrap();

    let mut pubsub_stream = pubsub.into_on_message();

    Sse::new(try_stream! {
        loop {
            match pubsub_stream.next().await {
                Some(notif) => {
                    let payload : String = notif.get_payload().unwrap();
                    let p = payload.clone();
                    let parts : Vec<_> = p.split(':').collect();

                    let event = Event::default()
                        .data(payload)
                        .event(parts[1]);
                    yield event
                }
                None => {dbg!("F");}
            }
        }

    })
    .keep_alive(KeepAlive::default())

    //Sse::new(try_stream! {
    //    loop {
    //        match pubsub.0.on_message() {
    //            Ok(msg) => {
    //                let payload : String = msg.get_payload().unwrap();
    //                let event = Event::default()
    //                    .data(payload);

    //                yield event;
    //            },
    //            Err(e) => {dbg!(e);}
    //        };
    //    }
    //})
    //.keep_alive(KeepAlive::default())
}
