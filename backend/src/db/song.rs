use crate::proto::general::Song as ProtoSong;
use chrono::NaiveDate;
use sqlx::{error::Error, PgPool};
use uuid::Uuid;

impl From<Song> for ProtoSong {
    fn from(s: Song) -> Self {
        Self {
            id: s.id.to_string(),
            title: s.title,
            artist_name: s.artist_name,
            description: s.description.unwrap_or_default(),
            header_image_thumbnail_url: s.header_image_thumbnail_url,
            header_image_url: s.header_image_url,
            genius_id: s.genius_id.unwrap_or_default(),
            recording_location: s.recording_location.unwrap_or_default(),
            release_date_for_display: s.release_date_for_display,
            song_art_image_thumbnail_url: s.song_art_image_thumbnail_url,
            album_cover_art_url: s.album_cover_art_url,
            album_name: s.album_name,
        }
    }
}

pub struct Song {
    pub id: Uuid,
    pub title: String,
    pub artist_name: String,
    pub description: Option<String>,
    pub header_image_thumbnail_url: String,
    pub header_image_url: String,
    pub genius_id: Option<String>,
    pub recording_location: Option<String>,
    pub release_date_for_display: String,
    pub song_art_image_thumbnail_url: String,
    pub album_cover_art_url: String,
    pub album_name: String,
    pub upload_date: NaiveDate,
}

impl Song {
    pub fn new(
        id: Uuid,
        title: String,
        artist_name: String,
        description: String,
        header_image_thumbnail_url: String,
        header_image_url: String,
        genius_id: Option<String>,
        recording_location: Option<String>,
        release_date_for_display: String,
        song_art_image_thumbnail_url: String,
        album_cover_art_url: String,
        album_name: String,
    ) -> Self {
        Song {
            id,
            title,
            artist_name,
            description: Some(description),
            header_image_thumbnail_url,
            header_image_url,
            genius_id,
            recording_location,
            release_date_for_display,
            song_art_image_thumbnail_url,
            album_cover_art_url,
            album_name,
            upload_date: chrono::Utc::now().date().naive_utc(),
        }
    }

    pub async fn save(&self, pool: &PgPool) -> Result<(), Error> {
        sqlx::query!(
            r#"insert into songs 
            (id, title, artist_name, description, header_image_thumbnail_url, header_image_url, genius_id, recording_location,
                release_date_for_display, song_art_image_thumbnail_url, album_cover_art_url, album_name)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"#,
            self.id,
            self.title,
            self.artist_name,
            self.description,
            self.header_image_thumbnail_url,
            self.header_image_url,
            self.genius_id,
            self.recording_location,
            self.release_date_for_display,
            self.song_art_image_thumbnail_url,
            self.album_cover_art_url,
            self.album_name
        ).execute(pool).await?;
        Ok(())
    }

    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Self, Error> {
        Ok(sqlx::query_as!(Self, "select * from songs where id=$1", id)
            .fetch_one(pool)
            .await?)
    }

    pub async fn find(
        limit: Option<i64>,
        offset: Option<i64>,
        pool: &PgPool,
    ) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(
            Self,
            "select * from songs order by upload_date offset $1 limit $2 ",
            offset.unwrap_or_default(),
            limit.unwrap_or(100)
        )
        .fetch_all(pool)
        .await?)
    }
}
