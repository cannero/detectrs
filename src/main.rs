use anyhow::{Result, anyhow};
use entities::Notifier;
use libsql_access::LibSqlAccess;
use notifications::mailtrap_sender::MailtrapSender;
use version_check::check_mysql_version;

mod comparer;
mod downloader;
mod entities;
mod libsql_access;
mod notifications;
mod parser;
mod version_check;

#[tokio::main]
async fn main() -> Result<()> {
    match dotenvy::dotenv() {
        Ok(_) => println!("loading .env"),
        Err(_) => println!("no .env found"),
    }

    let mailer = MailtrapSender::init()?;
    //let mailer = notifications::null_sender::NullSender{};

    let access = match LibSqlAccess::init().await {
        Ok(access) => access,
        Err(err) => {
            mailer.message(format!("Db access could not be created, {}", err))?;
            return Err(anyhow!("Db access init failed"));
        }
    };

    check_mysql_version(access, mailer).await?;
    Ok(())
}
