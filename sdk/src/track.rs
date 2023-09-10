use crate::proto::upload::{SaveMusicRequest, SaveMusicResponse, UploadFileResponse};
use crate::proto::{
    general::Song,
    track::{GetSongInfoResponse, SongRepository},
};
use crate::utils::{proto_to_vec, ResponseToProto};
use crate::Avelio;
use anyhow::Result;
use bytes::Bytes;
use reqwest::{multipart, StatusCode};

impl Avelio {
    pub async fn upload_track(&self, bytes: Bytes, song: Song) -> Result<String> {
        assert!(self.token.len() > 0);
        let response_upload = self
            .c
            .post(format!("{}/track/upload", self.base_url))
            .bearer_auth(&self.token)
            .multipart(
                multipart::Form::new().part("upload", multipart::Part::bytes(bytes.to_vec())),
            )
            .send()
            .await?;

        match response_upload.status() {
            StatusCode::CREATED => {
                let mut body = response_upload.bytes().await?;
                let file = body.proto::<UploadFileResponse>()?;
                let resp = self
                    .c
                    .post(format!("{}/track/save", self.base_url))
                    .bearer_auth(&self.token)
                    .body(proto_to_vec(SaveMusicRequest {
                        file_id: file.id,
                        song: Some(song),
                    }))
                    .send()
                    .await?;
                match resp.status() {
                    StatusCode::CREATED => {
                        let mut body = resp.bytes().await?;
                        let music = body.proto::<SaveMusicResponse>()?;
                        Ok(music.id)
                    }
                    _ => Err(anyhow::anyhow!(format!(
                        "Response upload_track/save with status {} expected 201",
                        resp.status()
                    ))),
                }
            }
            _ => Err(anyhow::anyhow!(format!(
                "Response upload_track/upload with status {} expected 201",
                response_upload.status()
            ))),
        }
    }

    pub async fn tracks(&self, limit: Option<i64>, offset: Option<i64>) -> Result<SongRepository> {
        assert!(self.token.len() > 0);
        let response = self
            .c
            .get(format!("{}/track/m", self.base_url))
            .bearer_auth(&self.token)
            .query(&[("limit", limit), ("offset", offset)])
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let mut body = response.bytes().await?;
                let songs = body.proto::<SongRepository>()?;
                Ok(songs)
            }
            _ => Err(anyhow::anyhow!(format!(
                "Response tracks with status {} expected 200",
                response.status()
            ))),
        }
    }

    pub async fn track_by_id(&self, id: String) -> Result<Song> {
        assert!(self.token.len() > 0);
        let response = self
            .c
            .get(format!("{}/track/{}", self.base_url, id))
            .bearer_auth(&self.token)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let mut body = response.bytes().await?;
                let song = body.proto::<GetSongInfoResponse>()?;
                Ok(song.song.unwrap())
            }
            _ => Err(anyhow::anyhow!(format!(
                "Response track_by_id with status {} expected 200",
                response.status()
            ))),
        }
    }

    pub async fn get_bytes_of_track(&self, id: String) -> Result<Bytes> {
        assert!(self.token.len() > 0);
        let response = self
            .c
            .get(format!("{}/track/{}/listen", self.base_url, id))
            .bearer_auth(&self.token)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let body = response.bytes().await?;
                Ok(body)
            }
            _ => Err(anyhow::anyhow!(format!(
                "Response get_bytes_of_track with status {} expected 200",
                response.status()
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tracks() -> Result<()> {
        let mut c = Avelio::default();
        let tokens = c.sign_in("test".to_string(), "test".to_string()).await?;
        c.token = tokens.token;
        println!("{:#?}", c.tracks(None, None).await);
        Ok(())
    }
}
