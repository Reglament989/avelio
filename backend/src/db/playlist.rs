use crate::{proto::playlists::Playlist as ProtoPlaylist, utils};
use chrono::NaiveDate;
use sqlx::{Error, Executor, PgPool};
use uuid::Uuid;

use crate::proto::general::Song;

pub struct PlaylistSong {
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
    pub song_id: Uuid,
    pub playlist_id: Uuid,
    pub upload_date: NaiveDate,
}

impl From<PlaylistSong> for Song {
    fn from(s: PlaylistSong) -> Self {
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

#[derive(Debug)]
pub struct Playlist {
    pub id: Uuid,
    pub title: String,
    pub total_playback_display: String,
    pub playlist_owner: Uuid,
}

impl From<Playlist> for ProtoPlaylist {
    fn from(p: Playlist) -> Self {
        Self {
            id: p.id.to_string(),
            title: p.title,
            total_playback_display: p.total_playback_display,
            playlist_owner_id: p.playlist_owner.to_string(),
        }
    }
}

impl Playlist {
    pub fn new(title: String, total_playback_display: String, playlist_owner: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            total_playback_display,
            playlist_owner,
        }
    }

    pub async fn save(&self, pool: &PgPool) -> Result<(), Error> {
        sqlx::query!("insert into playlist (id, title, total_playback_display, playlist_owner) values ($1, $2, $3, $4)", self.id, self.title, self.total_playback_display, self.playlist_owner).execute(pool).await?;
        Ok(())
    }

    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Self, Error> {
        Ok(
            sqlx::query_as!(Self, "select * from playlist where id=$1", id)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn add_songs(&self, songs: Vec<String>, pool: &PgPool) -> Result<(), Error> {
        let mut tx = pool.begin().await?;
        for song in songs {
            let id = Uuid::parse_str(&song);
            match id {
                Ok(id) => {
                    let exists = sqlx::query!("select exists(select 1 from songs where id=$1)", id)
                        .fetch_one(pool)
                        .await?
                        .exists
                        .unwrap();
                    let exists_in_playlist = sqlx::query!("select exists(select 1 from playlist_record where song_id=$1 and playlist_id=$2)", id, self.id)
                        .fetch_one(pool)
                        .await?.exists.unwrap();
                    if exists {
                        if !exists_in_playlist {
                            sqlx::query!(
                            "insert into playlist_record (song_id, playlist_id) values ($1, $2)",
                            id,
                            self.id
                        )
                            .execute(&mut tx)
                            .await?;
                        } else {
                            return Err(Error::Decode(Box::new(
                                utils::Error::SongExistsInPlaylist(id.to_string()),
                            )));
                        }
                    } else {
                        return Err(Error::RowNotFound);
                    }
                }
                Err(err) => return Err(Error::Decode(Box::new(err))),
            }
        }
        tx.commit().await?;
        Ok(())
    }

    pub async fn songs(&self, pool: &PgPool) -> Result<Vec<PlaylistSong>, Error> {
        Ok(sqlx::query_as!(PlaylistSong,
            "select * from playlist_record p inner join songs s on p.song_id = s.id where p.playlist_id=$1",
            self.id
        ).fetch_all(pool).await?)
    }
}
