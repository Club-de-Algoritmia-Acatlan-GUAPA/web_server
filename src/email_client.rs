use anyhow::Result;
use lettre::{
    message::MultiPart, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};
use secrecy::{ExposeSecret, Secret};

use crate::domain::email::Email;

#[derive(Clone)]
pub struct EmailClient {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    sender: Email,
}

impl EmailClient {
    pub fn new(
        host: String,
        sender: Email,
        authorization_token: Secret<String>,
        timeout: std::time::Duration,
    ) -> Result<Self> {
        let creds = Credentials::new(
            sender.as_ref().to_owned(),
            authorization_token.expose_secret().into(),
        );

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&host)?
            .credentials(creds)
            .timeout(Some(timeout))
            .build();

        Ok(Self { mailer, sender })
    }

    pub async fn send_email(
        &self,
        recipient: &Email,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<()> {
        let email = Message::builder()
            .from(format!("Do Not Reply <{}>", self.sender.as_ref()).parse()?)
            .to(recipient.as_ref().parse()?)
            .subject(subject)
            .multipart(MultiPart::alternative_plain_html(
                text_content.to_string(),
                html_content.to_string(),
            ))?;

        self.mailer.send(email).await?;
        Ok(())
    }
}
