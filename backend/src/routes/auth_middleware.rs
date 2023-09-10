use crate::State;
use actix::fut::Ready;
use actix_web::{dev::ServiceRequest, web::Data, FromRequest, HttpMessage, HttpRequest};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};
use chrono::Duration;
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Clone)]
pub struct AuthInfo {
    pub user_id: String,
}

impl FromRequest for AuthInfo {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        match req.extensions().get::<AuthInfo>() {
            Some(user) => return actix::fut::ok(user.clone()),
            None => return actix::fut::err(actix_web::error::ErrorBadRequest("ups...")),
        };
    }

    fn extract(req: &actix_web::HttpRequest) -> Self::Future {
        Self::from_request(req, &mut actix_web::dev::Payload::None)
    }
}

pub async fn ok_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    let state = &req
        .app_data::<Data<State>>()
        .map(|data| data.clone())
        .unwrap();

    let token = decode::<Claims>(
        credentials.token(),
        &DecodingKey::from_secret(state.config.server.jwt_key.as_bytes()),
        &Validation::default(),
    );
    match token {
        Ok(token) => {
            let id = token.claims.sub;
            req.extensions_mut().insert(AuthInfo { user_id: id });
            Ok(req)
        }
        Err(_) => {
            let config = req
                .app_data::<Config>()
                .map(|data| data.clone())
                .unwrap_or_else(Default::default);
            Err(AuthenticationError::from(config).into())
        }
    }
}

pub fn create_token(key: &[u8], exp: Duration, id: String) -> String {
    let date = (chrono::Utc::now() + exp).timestamp();
    let claims = Claims {
        exp: date as usize,
        sub: id,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(key)).unwrap()
}

// First access, second refresh
pub fn generate_pair_tokens(key: &[u8], exp: Duration, id: String) -> (String, String) {
    (
        create_token(key, exp, id.clone()),
        create_token(key, Duration::days(7), id),
    )
}

pub fn verify_token(token: &str, jwt_key: &str) -> Result<String, Error> {
    let token = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_key.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token.claims.sub)
}
