use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    domain::model::User,
    handlers::{respond_bad_request, respond_internal_server_error},
    AppError::UserRepoSaveEmailAlreadyExistsErr,
    AppState,
};

use super::{UserAuthnOutputDTO, UserInfoDTO};

#[derive(Debug, Deserialize)]
pub struct RegisterUserInput {
    pub user: RegisterUserInputUserKey,
}

impl Into<User> for RegisterUserInput {
    fn into(self) -> User {
        User {
            email: self.user.email,
            username: self.user.username,
            bio: "".to_string(),
            image: "".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserInputUserKey {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register_user(
    Json(input): Json<RegisterUserInput>,
    Extension(state): Extension<Arc<AppState>>,
) -> (StatusCode, Json<Value>) {
    let pwd = input.user.password.clone();
    let user: User = input.into();
    match &state.auth_mgr.register_user(&user, pwd).await {
        Ok(()) => {
            let out = UserAuthnOutputDTO {
                user: UserInfoDTO {
                    email: user.email,
                    token: "TODO".to_string(),
                    username: user.username,
                    bio: "".to_string(),
                    image: "".to_string(),
                },
            };
            (StatusCode::OK, Json(serde_json::to_value(out).unwrap()))
        }
        Err(err) => match err {
            UserRepoSaveEmailAlreadyExistsErr => respond_bad_request(err),
            _ => respond_internal_server_error(err),
        },
    }
}
