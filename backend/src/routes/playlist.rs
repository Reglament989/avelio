use crate::{
    db::playlist::Playlist,
    proto::{
        general::{Request, Song as ProtoSong},
        playlists::{
            AddSongsRequest, GetPlaylistResponse, NewPlaylistResponse, Playlist as ProtoPlaylist,
        },
    },
    utils::proto::proto_to_vec,
};
use actix_protobuf::*;
use actix_web::{
    get, post,
    web::{self, Data},
    Error, HttpResponse,
};
use uuid::Uuid;

use crate::State;

use super::auth_middleware::AuthInfo;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(save)
        .service(extend)
        .service(get_songs)
        .service(get_playlist);
}

#[post("/new/{title}")]
async fn save(
    path: web::Path<String>,
    auth_info: AuthInfo,
    state: Data<State>,
) -> Result<HttpResponse, Error> {
    let title = path.into_inner();
    let playlist = Playlist::new(
        title,
        "Nothing to play".to_string(),
        Uuid::parse_str(&auth_info.user_id).unwrap(),
    );
    match playlist.save(&state.pool).await {
        Ok(_) => HttpResponse::Created().protobuf(Request {
            errors: vec![],
            success: true,
            data: proto_to_vec(NewPlaylistResponse {
                playlist: Some(ProtoPlaylist::from(playlist)),
            }),
        }),
        Err(err) => HttpResponse::Created().protobuf(Request {
            errors: vec![format!("Error while save {}", err)],
            success: false,
            data: vec![],
        }),
    }
}

#[post("/{id}/extend")]
async fn extend(
    path: web::Path<String>,
    req: ProtoBuf<AddSongsRequest>,
    state: Data<State>,
) -> Result<HttpResponse, Error> {
    let playlist_id = path.into_inner();
    let playlist = Playlist::find_by_id(
        Uuid::parse_str(&playlist_id).unwrap_or_default(),
        &state.pool,
    )
    .await;
    match playlist {
        Ok(p) => match p.add_songs(req.0.songs, &state.pool).await {
            Ok(_) => HttpResponse::Created().protobuf(Request {
                errors: vec![],
                success: true,
                data: vec![],
            }),
            Err(err) => HttpResponse::BadRequest().protobuf(Request {
                errors: vec![format!("{:#?}", err)],
                success: false,
                data: vec![],
            }),
        },
        Err(err) => HttpResponse::BadRequest().protobuf(Request {
            errors: vec![format!("{:#?}", err)],
            success: false,
            data: vec![],
        }),
    }
}

#[get("/{id}/songs")]
async fn get_songs(path: web::Path<String>, state: Data<State>) -> Result<HttpResponse, Error> {
    let playlist_id = path.into_inner();
    let playlist = Playlist::find_by_id(
        Uuid::parse_str(&playlist_id).unwrap_or_default(),
        &state.pool,
    )
    .await;
    match playlist {
        Ok(p) => {
            let songs = p
                .songs(&state.pool)
                .await
                .unwrap()
                .into_iter()
                .map(|s| ProtoSong::from(s))
                .collect();
            HttpResponse::Ok().protobuf(Request {
                errors: vec![],
                success: true,
                data: proto_to_vec(GetPlaylistResponse { songs }),
            })
        }
        Err(err) => HttpResponse::BadRequest().protobuf(Request {
            errors: vec![format!("{:#?}", err)],
            success: false,
            data: vec![],
        }),
    }
}

#[get("/{id}")]
async fn get_playlist(path: web::Path<String>, state: Data<State>) -> Result<HttpResponse, Error> {
    let playlist_id = path.into_inner();
    let playlist = Playlist::find_by_id(
        Uuid::parse_str(&playlist_id).unwrap_or_default(),
        &state.pool,
    )
    .await;
    match playlist {
        Ok(p) => HttpResponse::Ok().protobuf(Request {
            errors: vec![],
            success: true,
            data: proto_to_vec(ProtoPlaylist::from(p)),
        }),
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
    async fn test_playlist_extend() -> Result<(), Box<dyn std::error::Error>> {
        let c = reqwest::Client::new();
        let request = AddSongsRequest {
            songs: vec!["e0185674-3647-4317-906c-93045b9f21d5".to_string()],
        };
        let response = c
            .post("http://127.0.0.1:56750/playlist/82d6efad-bb2b-4df1-aed3-264e704d95db/extend")
            .header("Authorization", "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2NDYwNWM2ZS0zN2VjLTQ5ZTItOGExNC1lOGZhMzdlMTU1ODgiLCJleHAiOjE2MzcwNzM4NzJ9.EKnVc0lxcUWZkETfkokT9uBR9h_MDV54QJKJKJmrvwo")
            .header("Content-Type", "application/protobuf")
            .body(proto_to_vec(request))
            .send()
            .await?;

        let raw_request = Request::decode(response.bytes().await?)?;

        println!("{:#?}", raw_request);

        assert_eq!(raw_request.success, true);
        Ok(())
    }
}
