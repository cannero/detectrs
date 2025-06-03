use anyhow::{Result, anyhow};

use crate::{
    comparer::are_same,
    downloader::download_site,
    entities::{Notifier, ReadState, VersionAccess, VersionInfo},
    parser::extract_sidebar,
};

pub async fn check_mysql_version<A: VersionAccess, N: Notifier>(
    db_access: A,
    notifier: N,
) -> Result<()> {
    let mysql_url = "https://dev.mysql.com/doc/relnotes/mysql/8.4/en";
    let html_text = match download_site(mysql_url).await {
        Ok(html) => html,
        Err(err) => {
            notifier.message(format!("Site could not be read, {}", err))?;
            return Err(anyhow!("site reading failed"));
        }
    };

    let new_version = match extract_sidebar(&html_text) {
        Ok(text) => text,
        Err(err) => {
            notifier.message(format!("Version could not be parsed, {}", err))?;
            return Err(anyhow!("parsing failed"));
        }
    };

    if new_version.is_empty() {
        notifier.message("Only empty version found".to_string())?;
    }

    let maybe_latest_version = match db_access.get_latest_version().await {
        Ok(version) => version,
        Err(err) => {
            notifier.message(format!("Latest version could not be read, {}", err))?;
            return Err(anyhow!("reading latest version failed"));
        }
    };

    match maybe_latest_version {
        None => {
            add_new_version(&db_access, &notifier, new_version).await?;
            send_message(&notifier, "No old version found, initializing.")?;
        }
        Some(old_version) => {
            if are_same(&old_version.version_text, &new_version) {
                insert_info(
                    &db_access,
                    &notifier,
                    old_version.version_id,
                    ReadState::NoChange,
                )
                .await?;
                println!("versions match, nothing to do");
            } else {
                add_new_version(&db_access, &notifier, new_version).await?;
                send_message(&notifier, "NEW VERSION")?
            }
        }
    }

    Ok(())
}

fn send_message<N: Notifier>(notifier: &N, message: &str) -> Result<()> {
    notifier.message(message.to_string())
}

async fn add_new_version<A: VersionAccess, N: Notifier>(
    db_access: &A,
    notifier: &N,
    version: String,
) -> Result<()> {
    let id = match db_access.insert_version_text(version).await {
        Ok(id) => id,
        Err(err) => {
            notifier.message(format!("Insert version text failed, {}", err))?;
            return Err(anyhow!("insert version text failed"));
        }
    };

    insert_info(db_access, notifier, id, ReadState::Change).await
}

async fn insert_info<A: VersionAccess, N: Notifier>(
    db_access: &A,
    notifier: &N,
    id: i64,
    state: ReadState,
) -> Result<()> {
    let version_info = VersionInfo {
        version_id: id,
        state,
    };

    match db_access.insert_info(version_info).await {
        Ok(_) => Ok(()),
        Err(err) => {
            notifier.message(format!("Insert info failed, {}", err))?;
            Err(anyhow!("insert info failed"))
        }
    }
}
