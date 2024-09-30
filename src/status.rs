use axum::response::{IntoResponse, Response};

use crate::with_axum::{into_response, Template};
pub type ResultHTML = Result<ServerResponse, ServerResponse>;
pub enum ServerResponse {
    WrongPassword,
    InvalidUsername,
    AlreadyLoggedIn,
    GenericError(anyhow::Error),
    ProblemId(u32),
    ContestId(u32),
    SubmissionId(i128),
    SuccessfulLogin,
    SuccessfulSignup,
    SuccessfullySubscribedToContest,
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
    pub message: &'a str,
}

pub enum Messages<'a> {
    ErrorMessage(ErrorMessage<'a>),
    Message(Message<'a>),
    SuccessMessage(SuccessMessage<'a>),
}

impl std::fmt::Display for Messages<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Messages::ErrorMessage(e) => e.fmt(f),
            Messages::Message(m) => m.fmt(f),
            Messages::SuccessMessage(s) => s.fmt(f),
        }
    }
}

impl Template for Messages<'_> {
    const EXTENSION: Option<&'static str> = Some("txt");
    const MIME_TYPE: &'static str = "text/plain; charset=utf-8";
    const SIZE_HINT: usize = 0;

    fn render_into(
        &self,
        writer: &mut (impl std::fmt::Write + ?Sized),
    ) -> Result<(), askama::Error> {
        match self {
            Messages::ErrorMessage(e) => Ok(writer.write_str(e.render()?.as_str())?),
            Messages::Message(m) => Ok(writer.write_str(m.render()?.as_str())?),
            Messages::SuccessMessage(s) => Ok(writer.write_str(s.render()?.as_str())?),
        }
    }
}

#[derive(Template)]
#[template(path = "flash_message.html", escape = "none")]
pub struct FlashMessage<'a> {
    pub message: Messages<'a>,
    pub redirect: String,
    pub delay_in_secs: f32,
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
        ServerResponse::SuccessfulLogin => into_response(&FlashMessage {
            message: Messages::SuccessMessage(SuccessMessage {
                message: "Successfully logged in",
            }),
            redirect: "/problems".to_string(),
            delay_in_secs: 0.5,
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
        ServerResponse::SuccessfulSignup => into_response(&FlashMessage {
            message: Messages::SuccessMessage(SuccessMessage {
                message: "Successfully signed up, redirecting to login page.",
            }),
            redirect: "/login".to_string(),
            delay_in_secs: 1.0,
        }),
        ServerResponse::SuccessfullySubscribedToContest => into_response(&SuccessMessage {
            message: "Successfully subscribed to contest",
        }),
    }
}
