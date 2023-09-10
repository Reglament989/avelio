use super::auth_middleware::verify_token;
use crate::{
    db::user::User,
    proto::{
        auth::{AuthorizateRequest, AuthorizateResponse, RefreshTokenRequest},
        general::Request,
    },
    routes::auth_middleware::generate_pair_tokens,
    utils::proto::proto_to_vec,
};
use crate::{utils, State};
use actix_protobuf::*;
use actix_web::{
    delete, get, patch, post,
    web::{self, Data},
    Error, HttpResponse, Responder,
};
use chrono::Duration;
use prost::Message;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(register).service(refresh);
}

#[post("/login")]
async fn login(
    req: ProtoBuf<AuthorizateRequest>,
    state: Data<State>,
) -> Result<HttpResponse, Error> {
    match User::find_by_login(&req.login, &state.pool).await {
        Some(candidate) => {
            match utils::hashing::validate(&candidate.password_hash, req.password.as_bytes()) {
                true => {
                    let (token, refresh_token) = generate_pair_tokens(
                        state.config.server.jwt_key.as_bytes(),
                        Duration::days(1),
                        candidate.id.to_string(),
                    );
                    let data = AuthorizateResponse {
                        token,
                        refresh_token,
                    };
                    HttpResponse::Ok().protobuf(Request {
                        errors: vec![],
                        success: true,
                        data: proto_to_vec(data),
                    })
                }
                false => HttpResponse::BadRequest().protobuf(Request {
                    errors: vec!["User not found or password incorect".to_string()],
                    success: false,
                    data: vec![],
                }),
            }
        }
        None => HttpResponse::BadRequest().protobuf(Request {
            errors: vec!["User not found or password incorect".to_string()],
            success: false,
            data: vec![],
        }),
    }
}

#[post("/refresh")]
async fn refresh(
    req: ProtoBuf<RefreshTokenRequest>,
    state: Data<State>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let token = req.0.token;
    let user_id = verify_token(&token, &state.config.server.jwt_key)?;
    match User::find_by_id(&user_id, &state.pool).await {
        Some(user) => {
            if user.blacklist_tokens.contains(&token) {
                return Ok(HttpResponse::BadRequest().protobuf(Request {
                    errors: vec!["Token invalid or user does not exists".to_string()],
                    success: false,
                    data: vec![],
                })?);
            } else {
                user.add_token_to_blacklist(token, &state.pool).await?;
                let (token, refresh_token) = generate_pair_tokens(
                    state.config.server.jwt_key.as_bytes(),
                    Duration::days(1),
                    user_id,
                );
                let data = AuthorizateResponse {
                    token,
                    refresh_token,
                };
                Ok(HttpResponse::Ok().protobuf(Request {
                    errors: vec![],
                    success: true,
                    data: proto_to_vec(data),
                })?)
            }
        }
        None => {
            return Ok(HttpResponse::BadRequest().protobuf(Request {
                errors: vec!["Token invalid or user does not exists".to_string()],
                success: false,
                data: vec![],
            })?);
        }
    }
}

#[post("/register")]
async fn register(
    req: ProtoBuf<AuthorizateRequest>,
    state: Data<State>,
) -> Result<HttpResponse, Error> {
    if state.config.server.registration {
        match User::find_by_login(&req.login, &state.pool).await {
            Some(_) => {
                return HttpResponse::BadRequest().protobuf(Request {
                    errors: vec!["This login was not available".to_string()],
                    success: false,
                    data: vec![],
                })
            }
            None => {
                let user = User::new(
                    req.login.to_string(),
                    req.login.to_string(),
                    utils::hashing::hash(req.password.as_bytes()),
                );
                return match user.save(&state.pool).await {
                    Ok(_) => {
                        let (token, refresh_token) = generate_pair_tokens(
                            state.config.server.jwt_key.as_bytes(),
                            Duration::days(1),
                            user.id.to_string(),
                        );
                        let data = AuthorizateResponse {
                            token,
                            refresh_token,
                        };
                        HttpResponse::Created().protobuf(Request {
                            errors: vec![],
                            success: true,
                            data: proto_to_vec(data),
                        })
                    }
                    Err(err) => HttpResponse::InternalServerError().protobuf(Request {
                        errors: vec![format!("Error while save user {}", err)],
                        success: false,
                        data: vec![],
                    }),
                };
            }
        }
    }
    Ok(HttpResponse::Gone().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[actix_web::test]
    async fn test_register() -> Result<(), Box<dyn std::error::Error>> {
        let c = reqwest::Client::new();
        let request = AuthorizateRequest {
            login: "test".to_string(),
            password: "password".to_string(),
        };
        let response = c
            .post("http://127.0.0.1:56750/auth/register")
            .header("Content-Type", "application/protobuf")
            .body(proto_to_vec(request))
            .send()
            .await?;

        assert_eq!(response.status(), 201);

        let raw_request = Request::decode(response.bytes().await?)?;

        assert_eq!(raw_request.success, true);

        let tokens = AuthorizateResponse::decode(&*raw_request.data)?;

        println!("{}", tokens.token);
        Ok(())
    }

    #[actix_web::test]
    async fn test_login() -> Result<(), Box<dyn std::error::Error>> {
        let c = reqwest::Client::new();
        let request = AuthorizateRequest {
            login: "test".to_string(),
            password: "password".to_string(),
        };
        let response = c
            .post("http://127.0.0.1:56750/auth/login")
            .header("Content-Type", "application/protobuf")
            .body(proto_to_vec(request))
            .send()
            .await?;

        assert_eq!(response.status(), 200);

        let raw_request = Request::decode(response.bytes().await?)?;

        assert_eq!(raw_request.success, true);

        let tokens = AuthorizateResponse::decode(&*raw_request.data)?;

        println!("{}", tokens.token);
        Ok(())
    }
}
