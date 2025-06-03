use anyhow::{Context, Result};
use libsql::{Builder, Connection, de};

use crate::entities::{Version, VersionAccess, VersionInfo};

pub struct LibSqlAccess {
    url: String,
    token: String,
}

impl LibSqlAccess {
    pub async fn init() -> Result<Self> {
        let url = std::env::var("TURSO_DATABASE_URL").context("TURSO_DATABASE_URL must be set")?;
        let token = std::env::var("TURSO_AUTH_TOKEN").context("TURSO_AUTH_TOKEN must be set")?;

        let access = Self { url, token };

        let conn = access.connect().await?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS version (
                 versionId INTEGER PRIMARY KEY,
                 versionText TEXT
             )",
            (),
        )
        .await?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS versionCheck (
                 checkId INTEGER PRIMARY KEY,
                 state TEXT,
                 versionId INTEGER,
                 runtime TEXT,
                 FOREIGN KEY(versionId) REFERENCES version(versionId)
              )",
            (),
        )
        .await?;

        Ok(access)
    }

    async fn connect(&self) -> Result<Connection> {
        // let db = Builder::new_local("local.db").build().await?;
        let db = Builder::new_remote(self.url.clone(), self.token.clone())
            .build()
            .await?;
        let conn = db.connect()?;
        Ok(conn)
    }
}

impl VersionAccess for LibSqlAccess {
    async fn get_latest_version(&self) -> Result<Option<Version>> {
        let conn = self.connect().await?;
        let maybe_row = conn
            .query(
                "SELECT versionId, versionText
                      FROM version
                      ORDER BY versionId DESC
                      LIMIT 1;",
                (),
            )
            .await?
            .next()
            .await?;

        match maybe_row {
            None => Ok(None),
            Some(row) => {
                let version = de::from_row::<Version>(&row)?;
                Ok(Some(version))
            }
        }
    }

    async fn insert_version_text(&self, text: String) -> Result<i64> {
        let conn = self.connect().await?;
        let mut stmt = conn
            .prepare(
                "INSERT INTO version (versionText) VALUES (?1)
                      RETURNING rowid",
            )
            .await?;

        let row = stmt.query_row([text]).await?;

        let rowid = row.get::<i64>(0)?;
        Ok(rowid)
    }

    async fn insert_info(&self, version_info: VersionInfo) -> Result<()> {
        let conn = self.connect().await?;
        let mut stmt = conn
            .prepare(
                "INSERT INTO versionCheck (state, versionId, runtime) VALUES (?1, ?2, datetime())",
            )
            .await?;

        stmt.execute((version_info.state.to_string(), version_info.version_id))
            .await?;

        Ok(())
    }
}
