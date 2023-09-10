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
pub struct AvelioRequest<T: Archive> {
    pub errors: Vec<String>,
    pub success: bool,
    pub data: T,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive(compare(PartialEq), check_bytes)]
#[archive_attr(derive(Debug))]
pub struct Song {
    pub id: i64,
    pub title: String,
    pub artist_name: String,
    pub description: String,
    pub header_image: String,
    pub genius_id: String,
    pub recording_location: String,
    pub release_date: String,
    pub image: String,
    pub album_cover: String,
    pub album_name: String,
}
