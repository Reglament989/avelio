use crate::db::song::Song;
use crate::proto::{
    general::{Request, Song as ProtoSong},
    track::{GetSongInfoResponse, SongRepository},
    upload::{SaveMusicRequest, SaveMusicResponse, UploadFileResponse},
};
use crate::utils::proto::proto_to_vec;
use crate::{routes::auth_middleware::AuthInfo, State};
use actix_multipart::Multipart;
use actix_protobuf::*;
use actix_web::{
    get,
    http::{HeaderName, HeaderValue},
    post,
    web::{self, Data},
    Error, HttpResponse,
};
use futures_util::TryStreamExt as _;
use minio_rs::minio::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(upload)
        .service(save)
        .service(get_many)
        .service(get)
        .service(listen);
}

#[post("/upload")]
async fn upload(
    mut payload: Multipart,
    state: Data<State>,
    _: AuthInfo,
) -> Result<HttpResponse, Error> {
    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field
            .content_disposition()
            .ok_or_else(|| HttpResponse::BadRequest().finish());
        match content_disposition {
            Ok(_) => {
                let mut buffer = web::BytesMut::new();

                // Field in turn is stream of *Bytes* object
                while let Some(chunk) = field.try_next().await? {
                    buffer.extend_from_slice(&chunk);
                }

                let file_id = uuid::Uuid::new_v4();
                let status = state
                    .s3
                    .client
                    .put_object_req(
                        "songs",
                        &format!("{}.mp3", file_id),
                        vec![(
                            HeaderName::from_static("content-type"),
                            HeaderValue::from_static("audio/mpeg"),
                        )],
                        buffer.to_vec(),
                    )
                    .await
                    .unwrap()
                    .resp
                    .status();
                match status {
                    StatusCode::OK => {
                        return HttpResponse::Created().protobuf(Request {
                            errors: vec![],
                            success: true,
                            data: proto_to_vec(UploadFileResponse {
                                id: file_id.to_string(),
                            }),
                        });
                    }
                    _ => {
                        return HttpResponse::InternalServerError().protobuf(Request {
                            errors: vec![format!("S3 bucket return code {}", status)],
                            success: false,
                            data: vec![],
                        })
                    }
                }
            }
            Err(response) => return Ok(response),
        }
    }
    Ok(HttpResponse::BadRequest().finish())
}

#[post("/save")]
async fn save(
    req: ProtoBuf<SaveMusicRequest>,
    state: Data<State>,
    _: AuthInfo,
) -> Result<HttpResponse, Error> {
    let file = state
        .s3
        .client
        .get_object_req("songs", &format!("{}.mp3", req.file_id), vec![])
        .await;
    match file {
        Ok(_) => {
            let info = req.song.clone().unwrap();
            let song = Song::new(
                Uuid::parse_str(&req.file_id).unwrap(),
                info.title,
                info.artist_name,
                info.description,
                info.header_image_thumbnail_url,
                info.header_image_url,
                Some(info.genius_id),
                Some(info.recording_location),
                info.release_date_for_display,
                info.song_art_image_thumbnail_url,
                info.album_cover_art_url,
                info.album_name,
            );
            match song.save(&state.pool).await {
                Ok(_) => HttpResponse::Created().protobuf(Request {
                    errors: vec![],
                    success: true,
                    data: proto_to_vec(SaveMusicResponse {
                        id: song.id.to_string(),
                    }),
                }),
                Err(err) => HttpResponse::BadRequest().protobuf(Request {
                    errors: vec![format!("{:#?}", err)],
                    success: false,
                    data: vec![],
                }),
            }
        }
        Err(err) => HttpResponse::BadRequest().protobuf(Request {
            errors: vec![format!("{:#?}", err)],
            success: false,
            data: vec![],
        }),
    }
}

#[derive(Deserialize)]
struct GetManyTracksQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[get("/m")]
async fn get_many(
    query: web::Query<GetManyTracksQuery>,
    state: Data<State>,
    _: AuthInfo,
) -> Result<HttpResponse, Error> {
    match Song::find(query.limit, query.offset, &state.pool).await {
        Ok(tracks) => {
            let songs = {
                let mut songs = Vec::<ProtoSong>::new();
                for song in tracks {
                    songs.push(ProtoSong::from(song));
                }
                songs
            };
            HttpResponse::Ok().protobuf(Request {
                errors: vec![],
                success: true,
                data: proto_to_vec(SongRepository { songs }),
            })
        }
        Err(err) => HttpResponse::InternalServerError().protobuf(Request {
            errors: vec![format!("{}", err)],
            success: false,
            data: vec![],
        }),
    }
}

#[get("/{id}")]
async fn get(
    path: web::Path<String>,
    state: Data<State>,
    _: AuthInfo,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let song = Song::find_by_id(Uuid::parse_str(&id).unwrap_or_default(), &state.pool).await;
    match song {
        Ok(song) => {
            return HttpResponse::Ok().protobuf(Request {
                errors: vec![],
                success: true,
                data: proto_to_vec(GetSongInfoResponse {
                    song: Some(ProtoSong::from(song)),
                    id,
                }),
            })
        }
        Err(err) => HttpResponse::BadRequest().protobuf(Request {
            errors: vec![format!("{}", err)],
            success: false,
            data: vec![],
        }),
    }
}

#[get("/{id}/listen")]
async fn listen(
    path: web::Path<String>,
    state: Data<State>,
    _: AuthInfo,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let song = state
        .s3
        .client
        .get_object_req("songs", &format!("{}.mp3", id), vec![])
        .await;
    match song {
        Ok(mut song) => match song.bytes().await {
            Ok(b) => Ok(HttpResponse::Ok().body(b)),
            Err(err) => Ok(HttpResponse::BadRequest()
                .protobuf(Request {
                    errors: vec![format!("{:#?}", err)],
                    success: false,
                    data: vec![],
                })
                .unwrap()),
        },
        Err(err) => HttpResponse::BadRequest().protobuf(Request {
            errors: vec![format!("{:#?}", err)],
            success: false,
            data: vec![],
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::settings::Settings;

    use super::*;
    use bytes::Bytes;
    use pretty_assertions::{assert_eq, assert_ne};
    use prost::Message;
    use reqwest::Client;

    #[actix_web::test]
    async fn test_save() -> Result<(), Box<dyn std::error::Error>> {
        let c = reqwest::Client::new();
        let request = SaveMusicRequest {
            file_id: "e0185674-3647-4317-906c-93045b9f21d5".to_string(),
            song: Some(ProtoSong {
                id: "".to_string(),
                title: "Test".to_string(),
                artist_name: "No name".to_string(),
                description: "WHAT?".to_string(),
                header_image_thumbnail_url: "".to_string(),
                header_image_url: "".to_string(),
                genius_id: "".to_string(),
                recording_location: "".to_string(),
                release_date_for_display: "".to_string(),
                song_art_image_thumbnail_url: "".to_string(),
                album_cover_art_url: "".to_string(),
                album_name: "".to_string(),
            }),
        };
        let response = c
            .post("http://127.0.0.1:56750/track/save")
            .header("Content-Type", "application/protobuf")
            .header("Authorization", "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2NDYwNWM2ZS0zN2VjLTQ5ZTItOGExNC1lOGZhMzdlMTU1ODgiLCJleHAiOjE2MzcwNzM4NzJ9.EKnVc0lxcUWZkETfkokT9uBR9h_MDV54QJKJKJmrvwo")
            .body(proto_to_vec(request))
            .send()
            .await?;

        assert_eq!(response.status(), 201);

        let raw_request = Request::decode(response.bytes().await?)?;

        assert_eq!(raw_request.success, true);

        let resp = SaveMusicResponse::decode(&*raw_request.data)?;

        println!("{:#?}", resp);
        Ok(())
    }
}
