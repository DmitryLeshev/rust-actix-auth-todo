use actix_web::{web::Data, FromRequest};
use std::{
    future::{ready, Ready},
    ops::Deref,
    sync::Arc,
};
use tracing::instrument;
use validator::Validate;

use crate::{
    api::v1::{
        account::models::AccountId,
        todo::{
            models::{QuerySearchTodoLists, DEFAULT_LIMIT_TODOLIST, DEFAULT_PAGE_TODOLIST},
            PATH,
        },
    },
    app::{error::AppError, state::AppState},
    common::models::Pagination,
};

use super::{
    models::{
        DTOCreateTodoItem, DTOCreateTodoList, DTODeleteTodoItem, DTODeleteTodoList,
        DTOGetTodoItems, DTOGetTodoLists, DTOJsonUpdateTodoItem, DTOJsonUpdateTodoList,
        DTOUpdateTodoItem, DTOUpdateTodoList, QuerySearchTodoItems, TodoItem, TodoItemId,
        TodoItems, TodoList, TodoListId, TodoLists, DEFAULT_LIMIT_TODOITEM, DEFAULT_PAGE_TODOITEM,
    },
    repository::TodoRepository,
};

pub struct TodoService {
    repository: Arc<TodoRepository>,
}

impl TodoService {
    pub fn new(repository: Arc<TodoRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_todolists(
        &self,
        query: QuerySearchTodoLists,
        account_id: i64,
        domain: String,
        version: u8,
    ) -> Result<(TodoLists, Option<Pagination>), AppError> {
        let limit = match query.limit {
            Some(limit) => limit,
            None => DEFAULT_LIMIT_TODOLIST,
        };
        let page = match query.page {
            Some(page) => page,
            None => DEFAULT_PAGE_TODOLIST,
        };
        let offset = page * &limit - &limit;
        let dto = DTOGetTodoLists {
            account_id,
            limit,
            offset,
            name: query.name.clone(),
        };
        let (todolists, number_items) = self.repository.get_todolists(dto).await?;
        let pagination = if todolists.len() > 0 {
            let base_link = format!("{}/api/v{}{}?", domain, version, PATH);
            Some(Pagination::new(number_items, limit, page).create_links(base_link))
        } else {
            None
        };

        Ok((todolists, pagination))
    }

    pub async fn create_todolist(&self, dto: DTOCreateTodoList) -> Result<TodoList, AppError> {
        match dto.validate() {
            Ok(_) => Ok(()),
            Err(errors) => {
                let error_map = errors.field_errors();
                let message = if error_map.contains_key("name") {
                    format!("Invalid name. \"{}\" is too short.", dto.name)
                } else {
                    "Invalid input.".to_string()
                };

                Err(AppError::BAD_REQUEST.message(message))
            }
        }?;
        if let None = dto.account_id {
            return Err(AppError::UNAUTHORIZED
                .message("Батя, надо авторизоваться, что бы создать тудулистик".to_string()));
        };
        let todolist = self.repository.create_todolist(dto).await?;
        Ok(todolist)
    }

    pub async fn update_todolist(
        &self,
        json: DTOJsonUpdateTodoList,
        account_id: AccountId,
        todolist_id: TodoListId,
    ) -> Result<TodoList, AppError> {
        let name = if let Some(name) = json.name {
            name
        } else {
            return Err(AppError::BAD_REQUEST.message("Где имя!".to_string()));
        };
        let dto = DTOUpdateTodoList {
            name,
            account_id,
            todolist_id,
        };
        let todolist = self.repository.update_todolist(dto).await?;
        Ok(todolist)
    }

    pub async fn delete_todolist(&self, dto: DTODeleteTodoList) -> Result<(), AppError> {
        Ok(self.repository.delete_todolist(dto).await?)
    }

    pub async fn get_todoitems(
        &self,
        query: QuerySearchTodoItems,
        account_id: i64,
        todolist_id: i64,
        domain: String,
        version: u8,
    ) -> Result<(TodoItems, Option<Pagination>), AppError> {
        let limit = match query.limit {
            Some(limit) => limit,
            None => DEFAULT_LIMIT_TODOITEM,
        };
        let page = match query.page {
            Some(page) => page,
            None => DEFAULT_PAGE_TODOITEM,
        };
        let offset = page * &limit - &limit;
        let dto = DTOGetTodoItems {
            account_id,
            todolist_id,
            limit,
            offset,
            name: query.name.clone(),
        };
        let (todoitems, number_items) = self.repository.get_todoitems(dto).await?;
        let pagination = if todoitems.len() > 0 {
            let base_link = format!(
                "{}/api/v{}{}/{}/todoitems?",
                domain, version, PATH, todolist_id
            );
            Some(Pagination::new(number_items, limit, page).create_links(base_link))
        } else {
            None
        };

        Ok((todoitems, pagination))
    }

    pub async fn create_todoitem(&self, dto: DTOCreateTodoItem) -> Result<TodoItem, AppError> {
        match dto.validate() {
            Ok(_) => Ok(()),
            Err(errors) => {
                let error_map = errors.field_errors();
                let message = if error_map.contains_key("name") {
                    format!("Invalid name. \"{}\" is too short.", dto.name)
                } else {
                    "Invalid input.".to_string()
                };

                Err(AppError::BAD_REQUEST.message(message))
            }
        }?;
        if let None = dto.todolist_id {
            return Err(AppError::BAD_REQUEST.message("Нэт того тудулистика".to_string()));
        };
        let todoitem = self.repository.create_todoitem(dto).await?;
        Ok(todoitem)
    }

    pub async fn update_todoitem(
        &self,
        json: DTOJsonUpdateTodoItem,
        todolist_id: TodoListId,
        todoitem_id: TodoItemId,
    ) -> Result<TodoItem, AppError> {
        json.one_field_is_required()?;
        match json.validate() {
            Ok(_) => Ok(()),
            Err(errors) => {
                let error_map = errors.field_errors();

                let message = if error_map.contains_key("name") {
                    format!(
                        "Invalid name. \"{}\" is too short.",
                        json.name.clone().unwrap()
                    )
                } else if error_map.contains_key("description") {
                    format!(
                        "Invalid description. \"{}\" is too short.",
                        json.description.clone().unwrap()
                    )
                } else {
                    "Invalid input.".to_string()
                };

                Err(AppError::BAD_REQUEST.message(message))
            }
        }?;
        let dto = DTOUpdateTodoItem::new(json, todolist_id, todoitem_id);
        let todoitem = self.repository.update_todoitem(dto).await?;
        Ok(todoitem)
    }

    pub async fn delete_todoitem(&self, dto: DTODeleteTodoItem) -> Result<(), AppError> {
        Ok(self.repository.delete_todoitem(dto).await?)
    }
}

impl FromRequest for TodoService {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    #[instrument(skip(req, payload))]
    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let app_state_result = Data::<AppState>::from_request(req, payload).into_inner();

        if let Ok(app_state) = app_state_result {
            let pool = app_state.deref().pool.clone();
            let repo = TodoRepository::new(Arc::new(pool.clone()));
            return ready(Ok(TodoService::new(Arc::new(repo))));
        } else {
            return ready(Err(AppError::SERVICE_ERROR
                .default()
                .with_cause("[TodoService] Initialization error".to_string())));
        }
    }
}
