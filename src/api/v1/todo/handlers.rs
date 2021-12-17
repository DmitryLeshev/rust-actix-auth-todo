use actix_web::web;

use crate::{
    api::v1::todo::models::ResponseGetTodoItems,
    app::{
        response::{AppResponse, ClientResponse},
        state::AppState,
    },
    common::services::SessionService,
};

use super::{
    models::{
        DTOCreateTodoItem, DTOCreateTodoList, DTODeleteTodoItem, DTODeleteTodoList,
        DTOJsonUpdateTodoItem, DTOJsonUpdateTodoList, DTOUpdateTodoList, QuerySearchTodoItems,
        QuerySearchTodoLists, ResponseDefaultTodolist, ResponseGetTodoLists, TodoItem, TodoItemId,
        TodoList, TodoListId,
    },
    service::TodoService,
};

pub async fn get_todolists(
    app_state: web::Data<AppState>,
    query: web::Query<QuerySearchTodoLists>,
    service_todo: TodoService,
    session_service: SessionService,
) -> AppResponse {
    let account_id = session_service.get_session_account_id().await?;
    let query = QuerySearchTodoLists {
        name: query.name.clone(),
        page: query.page.clone(),
        limit: query.limit.clone(),
    };
    let domain = app_state.domain.clone();
    let version = app_state.version.clone();

    let (items, pagination) = service_todo
        .get_todolists(query, account_id, domain, version)
        .await?;

    let data = ResponseGetTodoLists { items, pagination };

    Ok(ClientResponse::<ResponseGetTodoLists>::build()
        .with_message("Получай свои Тудулистки".to_string())
        .with_data(data)
        .send())
}

pub async fn create_todolist(
    dto: web::Json<DTOCreateTodoList>,
    service: TodoService,
    session_service: SessionService,
) -> AppResponse {
    let account_id = session_service.get_session_account_id().await?;
    let dto = DTOCreateTodoList {
        account_id: Some(account_id),
        name: dto.name.clone(),
    };
    let todolist = service.create_todolist(dto).await?;

    Ok(ClientResponse::<TodoList>::build()
        .with_data(todolist)
        .send())
}

pub async fn update_todolist(
    todolist_id: web::Path<TodoListId>,
    json: web::Json<DTOJsonUpdateTodoList>,
    service: TodoService,
    session_service: SessionService,
) -> AppResponse {
    let account_id = session_service.get_session_account_id().await?;
    let todolist = service
        .update_todolist(json.clone(), account_id, *todolist_id)
        .await?;
    Ok(ClientResponse::<TodoList>::build()
        .with_data(todolist)
        .with_message("EEEEE boy! ещё один метод!".to_string())
        .send())
}

pub async fn delete_todolist(
    todolist_id: web::Path<TodoListId>,
    service: TodoService,
    session_service: SessionService,
) -> AppResponse {
    let account_id = session_service.get_session_account_id().await?;
    let dto = DTODeleteTodoList {
        todolist_id: *todolist_id,
        account_id: account_id,
    };
    service.delete_todolist(dto).await?;
    Ok(ClientResponse::<ResponseDefaultTodolist>::build()
        .with_message(format!("Список с айди {} был удален", todolist_id))
        .send())
}

pub async fn get_todoitems(
    app_state: web::Data<AppState>,
    session_service: SessionService,
    todolist_id: web::Path<TodoListId>,
    query: web::Query<QuerySearchTodoItems>,
    service: TodoService,
    _session_service: SessionService,
) -> AppResponse {
    let account_id = session_service.get_session_account_id().await?;
    let query = QuerySearchTodoItems {
        name: query.name.clone(),
        page: query.page.clone(),
        limit: query.limit.clone(),
    };
    let domain = app_state.domain.clone();
    let version = app_state.version.clone();
    let (items, pagination) = service
        .get_todoitems(query, account_id, *todolist_id, domain, version)
        .await?;
    let data = ResponseGetTodoItems { items, pagination };
    Ok(ClientResponse::<ResponseGetTodoItems>::build()
        .with_message("Получай свои Тудушки".to_string())
        .with_data(data)
        .send())
}

pub async fn create_todoitem(
    todolist_id: web::Path<TodoListId>,
    dto: web::Json<DTOCreateTodoItem>,
    service: TodoService,
    session_service: SessionService,
) -> AppResponse {
    session_service.get_session_account_id().await?;

    let dto = DTOCreateTodoItem {
        todolist_id: Some(*todolist_id),
        name: dto.name.clone(),
        description: dto.description.clone(),
    };
    let todoitem = service.create_todoitem(dto).await?;

    Ok(ClientResponse::<TodoItem>::build()
        .with_data(todoitem)
        .send())
}

pub async fn update_todoitem(
    path: web::Path<(TodoListId, TodoItemId)>,
    json: web::Json<DTOJsonUpdateTodoItem>,
    service: TodoService,
    session_service: SessionService,
) -> AppResponse {
    session_service.get_session_account_id().await?;
    let todoitem = service
        .update_todoitem(json.clone(), path.0, path.1)
        .await?;
    Ok(ClientResponse::<TodoItem>::build()
        .with_data(todoitem)
        .with_message("Yes! Yes! Yes! Я сделал это!!!!".to_string())
        .send())
}

pub async fn delete_todoitem(
    path: web::Path<(TodoListId, TodoItemId)>,
    service: TodoService,
    session_service: SessionService,
) -> AppResponse {
    session_service.get_session_account_id().await?;
    let dto = DTODeleteTodoItem {
        todolist_id: path.0,
        todoitem_id: path.1,
    };
    service.delete_todoitem(dto).await?;
    Ok(ClientResponse::<ResponseDefaultTodolist>::build()
        .with_message(format!("ТУДУ с айди {} был удален", path.1))
        .send())
}
