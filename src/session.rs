use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
use async_session::{Session,Result,chrono::Utc, SessionStore};
use async_trait::async_trait;
use tracing::info;
use sqlx::{SqlitePool, FromRow};

#[derive(Debug, Clone)]
pub struct SqliteStore{
    pool: SqlitePool,
    table_name: String,
}
#[derive(Debug, Clone, FromRow)]
pub struct SessionRow{
    id: String,
    expiry: Option<i64>,
    session: String,
}
impl SqliteStore {
    pub fn from_pool(pool: SqlitePool) -> Self {
    Self{
    pool,
    table_name: "async_sessions".to_string(),
    }
    }

    pub fn from_pool_with_table_name(pool: SqlitePool, table_name: impl AsRef<str>) -> Self {
    Self{
    pool,
    table_name: table_name.as_ref().to_string(),
    }
    }

    pub fn with_table_name(mut self, table_name: impl AsRef<str>)-> Self{
        let table_name = table_name.as_ref();
        if table_name.is_empty()
            || !table_name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            panic!(
                "table name must be [a-zA-Z0-9_-]+, but {} was not",
                table_name
            );
        }

        self.table_name = table_name.to_owned();
        self
    }
    pub async fn migrate(&self) -> Result<()> {
        info!("migrating sessions on `{}`", self.table_name);

        let _ = sqlx::query(&self.substitute_table_name(
            r#"
            CREATE TABLE IF NOT EXISTS %%TABLE_NAME%% (
                id TEXT PRIMARY KEY NOT NULL,
                expiry INTEGER NULL,
                session TEXT NOT NULL
            )
            "#,
        )).execute(&self.pool).await?;
        Ok(())
    }
    // private utility function because sqlite does not support
    // parametrized table names
    fn substitute_table_name(&self, query: &str) -> String {
        query.replace("%%TABLE_NAME%%", &self.table_name)
    }


 /// Performs a one-time cleanup task that clears out stale
    /// (expired) sessions. You may want to call this from cron.
    pub async fn cleanup(&self) -> Result<()> {
        let _ = sqlx::query(&self.substitute_table_name(
            r#"
            DELETE FROM %%TABLE_NAME%%
            WHERE expiry < ?
            "#
        )).execute(&self.pool).await?;

        Ok(())
    }

    /// retrieves the number of sessions currently stored, including
    /// expired sessions

    pub async fn count(&self) -> Result<i32> {
//        let query = &self.subsVtitute_table_name("SELECT COUNT(*) as count from %%TABLE_NAME%%");
        //let count = sqlx::query!"SELECT COUNT(*) as count FROM %%TABLE_NAME%%")).await?;
        
        //TODO: MAKE THIS DO SO SOMETHING USEFUL
        Ok(1)
    }
}
#[async_trait]
impl SessionStore for SqliteStore{
 async fn load_session(&self, cookie_value: String) -> Result<Option<Session>> {
        let id = Session::id_from_cookie_value(&cookie_value)?;
        
        let session_row = sqlx::query_as::<_, SessionRow>(&self.substitute_table_name(
            r#"
            SELECT * FROM %%TABLE_NAME%%
              WHERE id = ? AND (expiry IS NULL OR expiry > ?)
            "#,
        )).bind(id.to_string()).bind(Utc::now().timestamp()).bind(id.to_string()).fetch_one(&self.pool).await;
        
        let session: Option<String> = match session_row{
        Ok(s) => Some(s.session),
        Err(_) => None
        };

        Ok(session
            .map(|session| serde_json::from_str(&session))
            .transpose()?)
    }

    async fn store_session(&self, session: Session) -> Result<Option<String>> {
        let id = session.id();
        let string = serde_json::to_string(&session)?;
        let expiry = session.expiry().map(|expiry| expiry.timestamp());
        let _ = sqlx::query(&self.substitute_table_name(
            r#"
            INSERT INTO %%TABLE_NAME%%
              (id, session, expiry) VALUES (?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
              expiry = excluded.expiry,
              session = excluded.session
            "#,
        )).bind((&id).to_string()).bind(string).bind(expiry).execute(&self.pool).await?;

        Ok(session.into_cookie_value())
    }

    async fn destroy_session(&self, session: Session) -> Result {
        let id = session.id();
        println!("ID: {id}");
        let _ = sqlx::query(&self.substitute_table_name(
            r#"
            DELETE FROM %%TABLE_NAME%% WHERE id = ?
            "#,
        )).bind(id.to_string()).execute(&self.pool).await?;

        Ok(())
    }

    async fn clear_store(&self) -> Result {
        let _ = sqlx::query(&self.substitute_table_name(
            r#"
            DELETE FROM %%TABLE_NAME%%
            "#,
        )).execute(&self.pool)
        .await?;

        Ok(())
    }
}
}}
