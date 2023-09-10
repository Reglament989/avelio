pub mod hashing;
pub mod proto;
use thiserror::Error as TError;

#[derive(TError, Debug)]
pub enum Error {
    #[error("Song with id `{0}` exists in playlist")]
    SongExistsInPlaylist(String),
    #[error("While operation in database whats happend")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Given id is incorrect")]
    IncorrectID(#[from] uuid::Error),
}
