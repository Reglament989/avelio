use crate::proto::profile;
use futures_util::TryFutureExt;
use sqlx::{Error, PgPool};
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub login: String,
    pub username: String,
    pub password_hash: String,
    pub blacklist_tokens: Vec<String>
}

impl User {
    pub fn new(login: String, username: String, password_hash: String) -> Self {
        User {
            id: Uuid::new_v4(),
            login,
            username,
            password_hash,
            blacklist_tokens: vec![]
        }
    }

    pub async fn save(&self, pool: &PgPool) -> Result<(), Error> {
        let mut tx = pool.begin().await?;
        sqlx::query!(
            "insert into users (id, login, username, password_hash, blacklist_tokens) values ($1, $2, $3, $4, $5)",
            self.id,
            self.login,
            self.username,
            self.password_hash,
            &self.blacklist_tokens,
        )
        .execute(&mut tx)
        .await?;
        sqlx::query!(
            "insert into playlist (id, title, total_playback_display, playlist_owner) values ($1, $2, $3, $4)", 
            Uuid::new_v4(), 
            "Liked", 
            "Nothing to play", 
            self.id
        )
        .execute(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn sensetive(&mut self) -> &mut Self {
        self.password_hash = String::from("");
        self.login = String::from("");
        self
    }

    pub async fn find_by_login(login: &str, pool: &PgPool) -> Option<Self> {
        sqlx::query_as!(Self, "select * from users where login = $1", login).fetch_one(pool).await.ok()
    }

    pub async fn find_by_id(id: &str, pool: &PgPool) -> Option<Self> {
        sqlx::query_as!(Self, "select * from users where id = $1", Uuid::parse_str(id).unwrap_or_default()).fetch_one(pool).await.ok()
    }

    pub async fn add_token_to_blacklist(&self, token: String, pool: &PgPool) -> Result<(), Error> {
        sqlx::query!("update users set blacklist_tokens = array_append(blacklist_tokens, $1)", token).execute(pool).await?;
        Ok(())
    }
}
