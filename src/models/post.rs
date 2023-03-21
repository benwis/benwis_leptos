use crate::models::User;
use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

cfg_if! {
if #[cfg(feature = "ssr")] {
    use sqlx::SqlitePool;
    use femark::HTMLOutput;

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlPost{
     id: u32,
     user_id: i64,
     title: String,
     excerpt: Option<String>,
     content: String,
     created_at: String,
     updated_at: String,
     published: bool,
     preview: bool,
     links: Option<String>,
     hero: Option<String>,
     tags: String,
    }

    impl SqlPost {
        pub async fn into_post(self, pool: &SqlitePool) -> Post {

            let HTMLOutput{content, toc} = femark::process_markdown_to_html(self.content).unwrap_or_default();
            Post {
                id: self.id,
                user: User::get(self.user_id, pool).await,
                title: self.title,
                created_at: self.created_at,
                published: self.published,
                excerpt: self.excerpt,
                content,
                toc,
                updated_at: self.updated_at,
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
    pub user: Option<User>,
    pub title: String,
    pub excerpt: Option<String>,
    pub content: String,
    pub toc: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub published: bool,
    pub preview: bool,
    pub links: Option<String>,
    pub hero: Option<String>,
    pub tags: String,
}
