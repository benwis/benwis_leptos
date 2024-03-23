use crate::models::SafeUser;
use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

cfg_if! {
if #[cfg(feature = "ssr")] {
    use sqlx::SqlitePool;
    use femark::HTMLOutput;
    use chrono::naive::NaiveDateTime;

    #[derive(sqlx::FromRow, Debug, Clone)]
    pub struct SqlPost{
     id: u32,
     user_id: i64,
     title: String,
     slug: String,
     excerpt: Option<String>,
     content: String,
     created_at: i64,
     updated_at: i64,
     published: bool,
     preview: bool,
     links: Option<String>,
     hero: Option<String>,
     tags: String,
    }

    impl SqlPost {
        #[tracing::instrument(level = "info", fields(error))]
        pub async fn into_post(self, pool: &SqlitePool) -> Post {
            let HTMLOutput{content, toc,..} = femark::process_markdown_to_html(&self.content).unwrap_or_default();
                println!("Content: {:#?}",content);
            Post {
                id: self.id,
                user: SafeUser::get(self.user_id, pool).await,
                title: self.title,
                slug: self.slug,
                created_at: self.created_at,
                created_at_pretty: NaiveDateTime::from_timestamp_opt(self.created_at, 0).unwrap_or_default().to_string(),
                published: self.published,
                excerpt: self.excerpt,
                content: self.content,
                html: content,
                toc,
                updated_at: self.updated_at,
                updated_at_pretty: NaiveDateTime::from_timestamp_opt(self.updated_at, 0).unwrap_or_default().to_string(),
                preview: self.preview,
                hero: self.hero,
                links: self.links,
                tags: self.tags,
            }
        }
        #[tracing::instrument(level = "info",fields(error))]
        pub async fn into_post_meta(self, pool: &SqlitePool) -> PostMeta {
            PostMeta {
                id: self.id,
                user_id: self.user_id,
                title: self.title,
                slug: self.slug,
                created_at: self.created_at,
                created_at_pretty: NaiveDateTime::from_timestamp_opt(self.created_at, 0).unwrap_or_default().to_string(),
                published: self.published,
                excerpt: self.excerpt,
                updated_at: self.updated_at,
                updated_at_pretty: NaiveDateTime::from_timestamp_opt(self.updated_at, 0).unwrap_or_default().to_string(),
                preview: self.preview,
                hero: self.hero,
                links: self.links,
                tags: self.tags,
            }
        }
    }
    #[derive(sqlx::FromRow, Debug, Clone)]
    pub struct SqlPostMeta{
     id: u32,
     user_id: i64,
     title: String,
     slug: String,
     excerpt: Option<String>,
     created_at: i64,
     updated_at: i64,
     published: bool,
     preview: bool,
     links: Option<String>,
     hero: Option<String>,
     tags: String,
    }
    impl SqlPostMeta{
        #[tracing::instrument(level = "info",fields(error))]
        pub async fn into_post_meta(self, pool: &SqlitePool) -> PostMeta {
            PostMeta {
                id: self.id,
                user_id: self.user_id,
                title: self.title,
                slug: self.slug,
                created_at: self.created_at,
                created_at_pretty: NaiveDateTime::from_timestamp_opt(self.created_at, 0).unwrap_or_default().to_string(),
                published: self.published,
                excerpt: self.excerpt,
                updated_at: self.updated_at,
                updated_at_pretty: NaiveDateTime::from_timestamp_opt(self.updated_at, 0).unwrap_or_default().to_string(),
                preview: self.preview,
                hero: self.hero,
                links: self.links,
                tags: self.tags,
            }
        }
    }
}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub id: u32,
    pub user: Option<SafeUser>,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content: String,
    pub html: String,
    pub toc: Option<String>,
    pub created_at: i64,
    pub created_at_pretty: String,
    pub updated_at: i64,
    pub updated_at_pretty: String,
    pub published: bool,
    pub preview: bool,
    pub links: Option<String>,
    pub hero: Option<String>,
    pub tags: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostMeta {
    pub id: u32,
    pub user_id: i64,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub created_at: i64,
    pub created_at_pretty: String,
    pub updated_at: i64,
    pub updated_at_pretty: String,
    pub published: bool,
    pub preview: bool,
    pub links: Option<String>,
    pub hero: Option<String>,
    pub tags: String,
}
