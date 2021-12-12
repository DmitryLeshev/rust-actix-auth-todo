use actix_web::{http::StatusCode, web};

use crate::app::{
    response::{AppResponse, ClientResponse},
    state::AppState,
};

use super::{
    models::{
        AccountId, DTOCreateAccount, DTOGetAccounts, DTOUpdateAccount, QuerySearchAccounts,
        ResponseAccount, ResponseCreateAccount, ResponseGetAccountById, ResponseGetAccounts,
        ResponseUpdateAccount,
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
pub async fn delete_account(
    account_id: web::Path<AccountId>,
    service: AccountService,
) -> AppResponse {
    service.delete_account(*account_id).await?;
    Ok(ClientResponse::<ResponseAccount>::build()
        .with_status(StatusCode::OK)
        .with_message(format!("Аккаунт с айди = {} был удалён", account_id))
        .send())
}
pub async fn create_account(
    dto: web::Json<DTOCreateAccount>,
    service: AccountService,
) -> AppResponse {
    let new_account = service.create_account(dto.clone()).await?;
    // + Header -> Location = uri new account
    Ok(ClientResponse::<ResponseCreateAccount>::build()
        .with_status(StatusCode::CREATED)
        .with_message(format!("Аккаунт успешно создан"))
        .with_data(new_account)
        .send())
}
pub async fn update_account(
    dto: web::Json<DTOUpdateAccount>,
    service: AccountService,
) -> AppResponse {
    let update_account = service.update_account(dto.clone()).await?;
    // + Header -> Location = uri new account
    Ok(ClientResponse::<ResponseUpdateAccount>::build()
        .with_status(StatusCode::CREATED)
        .with_message(format!("Аккаунт успешно изменён"))
        .with_data(update_account)
        .send())
}
pub async fn ban_account(account_id: web::Path<AccountId>, service: AccountService) -> AppResponse {
    service.ban_account(*account_id).await?;
    Ok(ClientResponse::<ResponseAccount>::build()
        .with_status(StatusCode::OK)
        .with_message(format!("Аккаунт с айди = {} был забанен", account_id))
        .send())
}
