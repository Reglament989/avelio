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
pub struct Playlist {
    pub id: String,
    pub title: String,
    pub total_playback: String,
    pub owner: i64,

    pub tracks: Vec<super::general::Song>,
}
