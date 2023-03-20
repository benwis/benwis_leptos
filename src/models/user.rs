use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use cfg_if::cfg_if;


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub permissions: HashSet<String>,
}

impl Default for User {
    fn default() -> Self {
        let permissions = HashSet::new();

        Self {
            id: -1,
            username: "Guest".into(),
            password: "".into(),
            permissions,
        }
    }
}
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::SqlitePool;
        use crate::functions::auth::SqlPermissionTokens;


    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlUser {
        pub id: i64,
        pub username: String,
        pub password: String,
    }

    impl SqlUser {
        pub fn into_user(self, sql_user_perms: Option<Vec<SqlPermissionTokens>>) -> User {
            User {
                id: self.id,
                username: self.username,
                password: self.password,
                permissions: if let Some(user_perms) = sql_user_perms {
                    user_perms
                        .into_iter()
                        .map(|x| x.token)
                        .collect::<HashSet<String>>()
                } else {
                    HashSet::<String>::new()
                },
            }
        }
    }


        impl User {
            pub async fn get(id: i64, pool: &SqlitePool) -> Option<Self> {
                let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = ?")
                    .bind(id)
                    .fetch_one(pool)
                    .await
                    .ok()?;
    
                //lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
                let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
                    "SELECT token FROM user_permissions WHERE user_id = ?;",
                )
                .bind(id)
                .fetch_all(pool)
                .await
                .ok()?;
    
                Some(sqluser.into_user(Some(sql_user_perms)))
            }
    
            pub async fn get_from_username(name: String, pool: &SqlitePool) -> Option<Self> {
                let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE username = ?")
                    .bind(name)
                    .fetch_one(pool)
                    .await
                    .ok()?;
    
                //lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
                let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
                    "SELECT token FROM user_permissions WHERE user_id = ?;",
                )
                .bind(sqluser.id)
                .fetch_all(pool)
                .await
                .ok()?;
    
                Some(sqluser.into_user(Some(sql_user_perms)))
            }
        }}}