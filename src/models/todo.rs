use cfg_if::cfg_if;
use serde::{Serialize, Deserialize};
use crate::models::User;

cfg_if! {
    if #[cfg(feature = "ssr")] {
    
        use sqlx::SqlitePool;
    
        #[derive(sqlx::FromRow, Clone)]
        pub struct SqlTodo {
            id: u32,
            user_id: i64,
            title: String,
            created_at: String,
            completed: bool,
        }
    
        impl SqlTodo {
            pub async fn into_todo(self, pool: &SqlitePool) -> Todo {
                Todo {
                    id: self.id,
                    user: User::get(self.user_id, pool).await,
                    title: self.title,
                    created_at: self.created_at,
                    completed: self.completed,
                }
            }
        }
    }
    }

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub user: Option<User>,
    pub title: String,
    pub created_at: String,
    pub completed: bool,
}