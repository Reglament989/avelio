use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive(
    // This will generate a PartialEq impl between our unarchived and archived
    // types:
    compare(PartialEq),
    // bytecheck can be used to validate your data if you want. To use the safe
    // API, you have to derive CheckBytes for the archived type:
    check_bytes,
)]
pub struct Profile {
    pub id: i64,
    pub username: i64,
    pub followers: i64,
    pub subscriptions: i64,
    pub playlists: Vec<super::playlists::Playlist>,
}
