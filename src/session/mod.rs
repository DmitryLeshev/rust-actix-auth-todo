use actix_identity::Identity;
use actix_web::{
    dev::Payload,
    error::{ErrorInternalServerError, ErrorUnauthorized},
    web, Error, FromRequest, HttpRequest,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{future::Future, pin::Pin};
use tracing::warn;

use crate::AppState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sessions {
    pub map: HashMap<String, SessionUser>,
}

impl Default for Sessions {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    Guest,
    User,
    Admin,
}

impl Default for Role {
    fn default() -> Self {
        Role::Guest
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionUser {
    pub id: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub authorities: Role,
}

impl FromRequest for SessionUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<SessionUser, Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let fut = Identity::from_request(req, pl);
        let app_state: Option<&web::Data<AppState>> = req.app_data();

        if app_state.is_none() {
            warn!("app_state is none!");
            return Box::pin(async { Err(ErrorInternalServerError("Server Error")) });
        }
        let app_state = app_state.unwrap().clone();
        Box::pin(async move {
            if let Some(identity) = fut.await?.identity() {
                if let Some(user) = app_state
                    .sessions
                    .read()
                    .unwrap()
                    .map
                    .get(&identity)
                    .map(|x| x.clone())
                {
                    return Ok(user);
                }
            };

            Err(ErrorUnauthorized("unauthorized"))
        })
    }
}
