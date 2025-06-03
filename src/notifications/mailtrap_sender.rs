use std::env;

use anyhow::{Context, Result, anyhow};
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::entities::Notifier;

pub struct MailtrapSender {
    user: String,
    password: String,
    recipient: Mailbox,
}

impl MailtrapSender {
    pub fn init() -> Result<Self> {
        let recipient = env::var("MAIL_RECIPIENT")
            .context("Mail recipient missing")?
            .parse()?;
        let user = env::var("SMTP_USER").context("SMTP user missing")?;
        let password = env::var("SMTP_PASSWORD").context("SMTP password missing")?;

        Ok(Self {
            user,
            password,
            recipient,
        })
    }
}

impl Notifier for MailtrapSender {
    fn message(&self, message: String) -> Result<()> {
        let email = Message::builder()
            .from("Detectrs <hello@demomailtrap.co>".parse()?)
            .to(self.recipient.clone())
            .subject("Version changed")
            .body(message)
            .unwrap();

        let creds = Credentials::new(self.user.clone(), self.password.clone());

        let mailer = SmtpTransport::relay("live.smtp.mailtrap.io")?
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => {
                println!("Email sent successfully!");
                Ok(())
            }
            Err(e) => {
                println!("{}", e);
                Err(anyhow!("Email could not be sent"))
            }
        }
    }
}

// pub fn send_mail() -> Result<()> {

//     let email = Message::builder()
//         .from("Detectrs <hello@demomailtrap.co>".parse()?)
//         .to(recipient.parse().unwrap())
//         .subject("Version changed")
//         .body(String::from("Hello, the version changed."))
//         .unwrap();

//     let creds = Credentials::new(user, password);

//     let mailer = SmtpTransport::relay("live.smtp.mailtrap.io")?
//         .credentials(creds)
//         .build();

//     match mailer.send(&email) {
//         Ok(_) => {
//             println!("Email sent successfully!");
//             Ok(())
//         },
//         Err(e) => {
//             println!("{}", e);
//             Err(anyhow!("Email could not be sent"))
//         }
//     }
// }
