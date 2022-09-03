use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use serde::Deserialize;
use serde_json::Value;

use crate::{jwt::sign, AppError::LoginWrongCredentialsErr, AppState};

use super::{respond_internal_server_error, respond_unauthorized, UserAuthnOutputDTO, UserInfoDTO};

#[derive(Debug, Deserialize)]
pub struct LoginUserInput {
    pub user: LoginUserInputUserKey,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserInputUserKey {
    pub email: String,
    pub password: String,
}

pub async fn login_user(
    Json(input): Json<LoginUserInput>,
    Extension(state): Extension<Arc<AppState>>,
) -> (StatusCode, Json<Value>) {
    match state
        .auth_mgr
        .login_user(input.user.email, input.user.password)
        .await
    {
        Ok(user) => match sign(user.id) {
            Ok(token) => {
                let out = UserAuthnOutputDTO {
                    user: UserInfoDTO {
                        email: user.email,
                        token: Some(token),
                        username: user.username,
                        bio: user.bio,
                        image: user.image,
                    },
                };
                (StatusCode::OK, Json(serde_json::to_value(out).unwrap()))
            }
            Err(err) => {
                log::error!("Failed to create jwt: {err}");
                respond_internal_server_error(err)
            }
        },
        Err(err) => match err {
            LoginWrongCredentialsErr => respond_unauthorized(err),
            _ => respond_internal_server_error(err),
        },
    }
}
