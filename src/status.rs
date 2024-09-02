//use askama_axum::Template;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::with_axum::{into_response, Template};
pub enum ServerResponse {
    WrongPassword,
    InvalidUsername,
    AlreadyLoggedIn,
    GenericError(anyhow::Error),
    ProblemId(u32),
    ContestId(u32),
    SubmissionId(i128),
    SuccessfulLogin,
}

#[derive(Template)]
#[template(path = "error_message.html")]
pub struct ErrorMessage<'a> {
    message: &'a str,
}

#[derive(Template)]
#[template(path = "message.html")]
pub struct Message<'a> {
    message: &'a str,
}

#[derive(Template)]
#[template(path = "success_message.html")]
pub struct SuccessMessage<'a> {
    message: &'a str,
}

impl<E> From<E> for ServerResponse
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::GenericError(err.into())
    }
}

impl IntoResponse for ServerResponse {
    fn into_response(self) -> Response {
        match_response(self)
    }
}

fn match_response(response: ServerResponse) -> Response {
    match response {
        ServerResponse::WrongPassword => into_response(&ErrorMessage {
            message: "Wrong password",
        }),
        ServerResponse::InvalidUsername => into_response(&ErrorMessage {
            message: "Invalid username",
        }),
        ServerResponse::AlreadyLoggedIn => into_response(&Message {
            message: "Already logged in",
        }),
        ServerResponse::GenericError(err) => into_response(&ErrorMessage {
            message: &err.to_string(),
        }),
        ServerResponse::SuccessfulLogin => into_response(&SuccessMessage {
            message: "Successfully logged in",
        }),
        ServerResponse::ProblemId(id) => into_response(&SuccessMessage {
            message: &format!("Problem ID: {}", id),
        }),
        ServerResponse::ContestId(id) => into_response(&SuccessMessage {
            message: &format!("Contest ID: {}", id),
        }),
        ServerResponse::SubmissionId(id) => into_response(&Message {
            message: &format!("Submission ID: {}", id),
        }),
    }
}
