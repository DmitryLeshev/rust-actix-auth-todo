use actix_web::{http::StatusCode, web};

use crate::app::{
    response::{AppResponse, ClientResponse},
    state::AppState,
};

use super::{
    models::{
        AccountId, DTOCreateAccount, DTOGetAccounts, QuerySearchAccounts, ResponseAccount,
        ResponseGetAccountById, ResponseGetAccounts,
    },
    service::AccountService,
    PATH,
};

pub async fn get_accounts(
    app_state: web::Data<AppState>,
    query: web::Query<QuerySearchAccounts>,
    service: AccountService,
) -> AppResponse {
    let dto = DTOGetAccounts {
        query: QuerySearchAccounts {
            email: query.email.clone(),
            page: query.page.clone(),
            limit: query.limit.clone(),
        },
        base_path: PATH.to_string(),
        domain: app_state.domain.clone(),
        version: app_state.version.clone(),
    };
    let data = service.get_accounts(dto).await?;
    if data.items.len() > 0 {
        Ok(ClientResponse::<ResponseGetAccounts>::build()
            .with_status(StatusCode::OK)
            .with_message("Что-то есть".to_string())
            .with_data(data)
            .send())
    } else {
        Ok(ClientResponse::<ResponseGetAccounts>::build()
            .with_status(StatusCode::NOT_FOUND)
            .with_message("Ни-ни, тута ни чего нэту".to_string())
            .with_data(data)
            .send())
    }
}
pub async fn get_account_by_id(
    account_id: web::Path<AccountId>,
    service: AccountService,
) -> AppResponse {
    let data = service.get_account_by_id(*account_id).await?;
    Ok(ClientResponse::<ResponseGetAccountById>::build()
        .with_status(StatusCode::OK)
        .with_message("Вот тот самый аккаунт".to_string())
        .with_data(data)
        .send())
}
pub async fn delete_account(account_id: web::Path<AccountId>) -> AppResponse {
    println!("{}", account_id);
    Ok(ClientResponse::<ResponseAccount>::default())
}
pub async fn create_account(dto: web::Json<DTOCreateAccount>) -> AppResponse {
    println!("{:?}", dto);
    Ok(ClientResponse::<ResponseAccount>::default())
}
pub async fn update_account(account_id: web::Path<AccountId>) -> AppResponse {
    println!("{}", account_id);
    Ok(ClientResponse::<ResponseAccount>::default())
}
pub async fn ban_account(account_id: web::Path<AccountId>) -> AppResponse {
    println!("{}", account_id);
    Ok(ClientResponse::<ResponseAccount>::default())
}
