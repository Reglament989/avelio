use crate::proto::auth::*;
use crate::utils::{proto_to_vec, ResponseToProto};
use crate::Avelio;
use anyhow::Result;
use reqwest::StatusCode;

impl Avelio {
    /// First bearer token, second refresh token, mutable for assign token into instance
    pub async fn sign_in(&self, login: String, password: String) -> Result<AuthorizateResponse> {
        let request = AuthorizateRequest { login, password };
        let raw_response = self
            .c
            .post(format!("{}/auth/login", self.base_url))
            .body(proto_to_vec(request))
            .send()
            .await?;
        match raw_response.status() {
            StatusCode::OK => {
                let mut body = raw_response.bytes().await?;
                Ok(body.proto::<AuthorizateResponse>()?)
            }
            _ => Err(anyhow::anyhow!(format!(
                "Response sign_in with status {} expected 200",
                raw_response.status()
            ))),
        }
    }

    pub async fn sign_up(&self, login: String, password: String) -> Result<AuthorizateResponse> {
        let request = AuthorizateRequest { login, password };
        let raw_response = self
            .c
            .post(format!("{}/auth/register", self.base_url))
            .body(proto_to_vec(request))
            .send()
            .await?;
        match raw_response.status() {
            StatusCode::CREATED => {
                let mut body = raw_response.bytes().await?;
                Ok(body.proto::<AuthorizateResponse>()?)
            },
            StatusCode::GONE => Err(anyhow::anyhow!("This server does not accept registration, if this is an error contact with administrator")),
            _ => Err(anyhow::anyhow!(format!(
                "Response sign_in with status {} expected 201",
                raw_response.status()
            ))),
        }
    }

    pub async fn refresh_token(&self, token: String) -> Result<AuthorizateResponse> {
        let request = RefreshTokenRequest { token };
        let raw_response = self
            .c
            .post(format!("{}/auth/refresh", self.base_url))
            .body(proto_to_vec(request))
            .send()
            .await?;
        match raw_response.status() {
            _ => {
                let mut body = raw_response.bytes().await?;
                Ok(body.proto::<AuthorizateResponse>()?)
            }
        }
    }
}
