use std::net::TcpListener;

use anyhow::Result;
use axum::{
    extract::DefaultBodyLimit,
    middleware::from_fn,
    routing::{delete, get, post},
    Router,
};
use fred::{clients::RedisPool, prelude::*};
use futures::future::Future;
use http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use primitypes::consts::{MAX_SUBMISSION_FILE_SIZE_IN_BYTES, MAX_TESCASE_FILE_SIZE_IN_BYTES};
use sqlx::{pool, PgPool};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};

use crate::{
    broker::MessageBroker,
    configuration::{AppSettings, CookiesSettings, RedisSettings, Settings},
    database::{get_postgres_pool, get_redis_pool},
    email_client::EmailClient,
    ftp::FTPClient,
    pubsub::pubsub_connection,
    rendering::render,
    routes::{
        confirm::confirm,
        contest::{post_subscribe_contest, post_update_or_create_contest},
        health::health,
        login::{login_get, login_post},
        logout::logout,
        new_problem::{
            add_new_test_case, download_test_case, get_test_cases, new_problem_get, new_problem_post, new_test_case, remove_single_test_case, remove_whole_test_case, update_problem_get, update_problem_post
        },
        notify::event_stream,
        problem::{problem_get, problem_static, problems_get},
        redirect::htmx_redirect,
        signup::{signup_get, signup_post},
        spa,
        submission::{submission_get, submission_get_id},
        submit::submit_post,
    },
    session::{needs_auth, render_navbar, session_middleware},
    telemetry::trace_headers,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub email_client: EmailClient,
    pub message_broker: MessageBroker,
    pub base_url: String,
    pub ftp: FTPClient,
}
pub async fn build(
    configuration: Settings,
) -> Result<impl Future<Output = Result<(), std::io::Error>>> {
    let email_client = configuration.email_client.client()?;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", configuration.app.application_port))?;

    let pg_pool = get_postgres_pool(&configuration.database).await;
    let redis_pool = get_redis_pool(&configuration.redis).await;

    let message_broker = MessageBroker::new(&configuration.rabbitmq).await;
    let ftp = FTPClient::new(configuration.upstream.uri.clone());

    Ok(run(
        listener,
        email_client,
        configuration.redis,
        configuration.app,
        configuration.cookies,
        pg_pool,
        redis_pool,
        message_broker,
        ftp,
    ))
}
pub fn run(
    listener: TcpListener,
    email_client: EmailClient,
    redis_config: RedisSettings,
    app_config: AppSettings,
    cookies_config: CookiesSettings,
    pool: PgPool,
    redis_pool: RedisPool,
    message_broker: MessageBroker,
    ftp: FTPClient,
) -> impl Future<Output = Result<(), std::io::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        //with(tracing_subscriber::fmt::layer())
        .init();

    let _cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap());
    //.allow_origin(Any);

    let serve_dir = ServeDir::new("static").not_found_service(ServeFile::new("static/404.html"));

    let session = session_middleware(redis_pool, &redis_config, &cookies_config, &app_config);

    let submissions = Router::new()
        .route("/", post(submit_post))
        .route("/get/:submission_id", get(submission_get_id))
        .route("/submissions/get", get(submission_get))
        .layer(from_fn(needs_auth))
        // max size of body 70kb
        .layer(DefaultBodyLimit::max(MAX_SUBMISSION_FILE_SIZE_IN_BYTES))
        .layer(_cors.clone());

    let problem = Router::new()
        .route("/new", post(new_problem_post))
        .route("/update/:problem_id", post(update_problem_post))
        .route("/testcases/:problem_id", get(get_test_cases))
        .layer(from_fn(needs_auth))
        .route("/get/:id", get(problem_get))
        .route("/all", get(problems_get));

    //let problem_registration = Router::new()
    //    .route("/newproblem", post(new_problem))
    //    .route("/newtestcase/:problem_id", post(new_test_case))
    // TODO necesita ser problem setter
    //.layer(from_fn(needs_auth));

    let testcase = Router::new()
        .route(
            "/get/:problem_id/:testcase_id/:filetype",
            get(download_test_case),
        )
        .route("/new/:problem_id", post(new_test_case))
        .route("/add/:problem_id/:testcase_id/:file_type", post(add_new_test_case))
        // TODO necesita ser problem setter
        .route(
            "/remove/:problem_id/:testcase_id/:filetype",
            delete(remove_single_test_case),
        )
        .route(
            "/remove/:problem_id/:testcase_id",
            delete(remove_whole_test_case),
        )
        .route(
            "/all/:problem_id",
            get(get_test_cases),
        )
        .layer(DefaultBodyLimit::max(MAX_TESCASE_FILE_SIZE_IN_BYTES))
        .layer(from_fn(needs_auth));

    let contest = Router::new()
        .route("/create", post(post_update_or_create_contest))
        .route("/subscribe/:contest_id", post(post_subscribe_contest))
        .layer(from_fn(needs_auth));

    let auth = Router::new()
        .route("/login", post(login_post))
        .route("/logout", get(logout))
        .route("/signup", post(signup_post).get(signup_get))
        .route("/confirm", get(confirm));

    let notif = Router::new().route("/submissions", get(event_stream));
    //    .layer(_cors.clone());
    let frontend = Router::new()
        .route("/newproblem", get(new_problem_get))
        .route("/editproblem/:problem_id", get(update_problem_get))
        .layer(from_fn(needs_auth))
        .route("/login", get(login_get))
        .route("/signup", get(signup_get))
        .route("/problems", get(problems_get))
        .route("/logout", get(logout))
        .route("/htmx/redirect/*path", get(htmx_redirect));

    let app = Router::new()
        .nest("/", frontend)
        .nest(
            "/api",
            Router::new()
                .route("/health", get(health))
                .nest("/submit", submissions)
                .nest("/problem", problem)
                .nest("/testcase", testcase)
                .nest("/auth", auth)
                .nest("/notify", notif)
                .nest("/contest", contest),
        )
        .with_state(AppState {
            pool,
            email_client,
            base_url: app_config.base_url,
            message_broker,
            ftp,
        })
        .layer(from_fn(trace_headers))
        .fallback_service(serve_dir)
        .layer(CompressionLayer::new())
        .layer(from_fn(render_navbar))
        .layer(session)
        .layer(from_fn(render))
        .layer(_cors);

    axum_server::from_tcp(listener).serve(app.into_make_service())
}
