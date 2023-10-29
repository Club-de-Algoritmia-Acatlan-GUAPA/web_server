use std::net::TcpListener;

use anyhow::Result;
use axum::{
    extract::DefaultBodyLimit,
    middleware::from_fn,
    routing::{get, post},
    Router,
};
use futures::future::Future;
use http::Method;
use primitypes::consts::MAX_SUBMISSION_FILE_SIZE_IN_BYTES;
use sqlx::PgPool;
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};

use crate::{
    broker::MessageBroker,
    configuration::{AppSettings, CookiesSettings, RedisSettings, Settings},
    database::get_pool,
    email_client::EmailClient,
    pubsub::pubsub_connection,
    routes::{
        confirm::confirm,
        health::health,
        login::{login_get, login_post},
        logout::logout,
        notify::event_stream,
        problem::{problem_get, problem_static, problems_get},
        signup::{signup_get, signup_post},
        submission::submission_get,
        submit::submit_post,
    },
    session::{needs_auth, session_middleware},
    telemetry::trace_headers,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub email_client: EmailClient,
    pub message_broker: MessageBroker,
    pub base_url: String,
}
pub async fn build(
    configuration: Settings,
) -> Result<impl Future<Output = Result<(), std::io::Error>>> {
    let email_client = configuration.email_client.client()?;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", configuration.app.application_port))?;

    let pool = get_pool(&configuration.database).await;

    let message_broker = MessageBroker::new(&configuration.rabbitmq).await;

    Ok(run(
        listener,
        email_client,
        configuration.redis,
        configuration.app,
        configuration.cookies,
        pool,
        message_broker,
    ))
}
pub fn run(
    listener: TcpListener,
    email_client: EmailClient,
    redis_config: RedisSettings,
    app_config: AppSettings,
    cookies_config: CookiesSettings,
    pool: PgPool,
    message_broker: MessageBroker,
) -> impl Future<Output = Result<(), std::io::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        //with(tracing_subscriber::fmt::layer())
        .init();

    let _cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let serve_dir = ServeDir::new("static").not_found_service(ServeFile::new("static/404.html"));

    let session = session_middleware(&redis_config, &cookies_config, &app_config);

    let submissions = Router::new()
        .route("/submit", post(submit_post))
        .route("/problem", get(problem_static))
        .route("/problems", get(problems_get))
        .route("/problem-get", get(problem_get))
        .route("/submission-get", get(submission_get))
        .layer(from_fn(needs_auth))
        // max size of body 70kb
        .layer(DefaultBodyLimit::max(MAX_SUBMISSION_FILE_SIZE_IN_BYTES));

    //let _pubsub = Arc::new(pubsub_connection(&redis_config));
    //let notif = Router::new()
    //    .route("/notify", get(event_stream))
    //    .with_state(Arc::clone(&pubsub))
    //    .layer(_cors.clone());

    let app = Router::new()
        .route("/health", get(health))
        .nest("/", submissions)
        //.nest("/", notif)
        .nest(
            "/",
            Router::new()
                .route("/login", post(login_post).get(login_get))
                .route("/logout", get(logout))
                .route("/signup", post(signup_post).get(signup_get))
                .route("/confirm", get(confirm)),
        )
        .with_state(AppState {
            pool,
            email_client,
            base_url: app_config.base_url,
            message_broker,
        })
        .layer(from_fn(trace_headers))
        .fallback_service(serve_dir)
        .layer(session);

    axum_server::from_tcp(listener).serve(app.into_make_service())
}
