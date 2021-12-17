use actix_session::Session;

use crate::{
    app::response::{AppResponse, ClientResponse},
    common::models::SessionAccount,
};

pub async fn create_session(session: Session) -> AppResponse {
    // access session data
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        session.insert("counter", count + 1)?;
    } else {
        session.insert("counter", 1)?;
    }
    let data = SessionAccount::default();
    let session_id = &data.session_id.clone();
    if let Some(session_user) = session.get::<SessionAccount>(session_id)? {
        println!("SESSION get USER: {:?}", session_user);
    } else {
        println!("SESSION insert USER: {:?}", data);
        session.insert(session_id, data.clone())?;
    }

    Ok(ClientResponse::<SessionAccount>::build()
        .with_data(data)
        .send())
}
