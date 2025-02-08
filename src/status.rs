use axum::{
    body::HttpBody,
    response::{IntoResponse, Redirect, Response},
};
use http::HeaderValue;
use primitypes::{problem::ContestId, status::Status};
use uuid::Uuid;

use crate::with_axum::{into_response, Template};
pub type ResultHTML = Result<ServerResponse, ServerResponse>;
pub enum ServerResponse {
    WrongPassword,
    InvalidUsername,
    AlreadyLoggedIn,
    GenericError(anyhow::Error),
    ProblemId(u32),
    ContestId(u32),
    SubmissionId(u128),
    SubmissionStatus(Status),
    SuccessfulLogin,
    SuccessfulSignup,
    SuccessfullySubscribedToContest(ContestId),
    SuccessfulTestCaseCreation(Uuid),
    SuccessfulTestCaseAdded(String),
    SuccessfulTestCaseOrderUpdate,
    NotFound,
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

#[derive(Template)]
#[template(path = "warning_message.html")]
pub struct WarningMessage<'a> {
    pub message: &'a str,
}
pub enum Messages<'a> {
    ErrorMessage(ErrorMessage<'a>),
    Message(Message<'a>),
    SuccessMessage(SuccessMessage<'a>),
    WarningMessage(WarningMessage<'a>),
}

impl std::fmt::Display for Messages<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Messages::ErrorMessage(e) => e.fmt(f),
            Messages::Message(m) => m.fmt(f),
            Messages::SuccessMessage(s) => s.fmt(f),
            Messages::WarningMessage(w) => w.fmt(f),
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
            Messages::WarningMessage(w) => Ok(writer.write_str(w.render()?.as_str())?),
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

impl IntoResponse for FlashMessage<'_> {
    fn into_response(self) -> Response {
        into_response(&self)
    }
}

fn match_response(response: ServerResponse) -> Response {
    match response {
        ServerResponse::WrongPassword => into_response(&Messages::ErrorMessage(ErrorMessage {
            message: "Wrong password",
        })),
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
        ServerResponse::SubmissionId(id) => {
            let mut res = into_response(&Message {
                message: &format!("Submission ID: {}", id),
            });
            res.headers_mut().insert(
                "Submission-ID",
                HeaderValue::from_str(id.to_string().as_str()).unwrap(),
            );
            res
        },
        ServerResponse::SuccessfulSignup => into_response(&FlashMessage {
            message: Messages::SuccessMessage(SuccessMessage {
                message: "Successfully signed up, redirecting to login page.",
            }),
            redirect: "/login".to_string(),
            delay_in_secs: 1.0,
        }),
        ServerResponse::SuccessfullySubscribedToContest(contest_id) => {
            into_response(&FlashMessage {
                message: Messages::SuccessMessage(SuccessMessage {
                    message: "Successfully subscribed to contest",
                }),
                redirect: format!("/contest/{}", contest_id.as_u32()),
                delay_in_secs: 1.4,
            })
        },
        ServerResponse::SuccessfulTestCaseCreation(filename) => {
            let mut response = into_response(&SuccessMessage {
                message: format!("Successfully Test case created, ID: {}", filename).as_str(),
            });
            let headers = response.headers_mut();
            headers.insert("HX-Trigger", "SuccessfulTestCaseCreation".parse().unwrap());
            response
        },
        ServerResponse::SuccessfulTestCaseAdded(uuid) => {
            let mut response = into_response(&SuccessMessage {
                message: format!(
                    "Successfully Test case added, ID: {}",
                    uuid.to_string().as_str()
                )
                .as_str(),
            });
            let headers = response.headers_mut();
            headers.insert("HX-Trigger", "SuccessfulTestCaseCreation".parse().unwrap());
            response
        },
        ServerResponse::SubmissionStatus(status) => {
            let message = &status.to_string();
            let mut res = match status {
                Status::Accepted => into_response(&SuccessMessage { message }),
                Status::WrongAnswer | Status::TimeLimitExceeded => {
                    into_response(&ErrorMessage { message })
                },
                Status::UnknownError(e) => into_response(&ErrorMessage {
                    message: &format!("Unknown Error: {}", e),
                }),
                Status::RuntimeError | Status::CompilationError => {
                    into_response(&WarningMessage { message })
                },
                Status::Pending | Status::PartialPoints => into_response(&Message { message }),
            };
            res.headers_mut()
                .insert("Submission-Status", HeaderValue::from_str(message).unwrap());
            res
        },
        ServerResponse::SuccessfulTestCaseOrderUpdate => into_response(&SuccessMessage {
            message: "Successfully updated test case order",
        }),
        ServerResponse::NotFound => Redirect::to("/404").into_response(),
    }
}
