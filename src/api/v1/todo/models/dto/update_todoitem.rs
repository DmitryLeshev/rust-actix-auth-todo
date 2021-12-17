use serde::{Deserialize, Serialize};
use validator_derive::Validate;

use crate::{
    api::v1::todo::models::{TodoItemId, TodoListId},
    app::error::AppError,
};

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DTOJsonUpdateTodoItem {
    #[validate(length(min = 3))]
    pub name: Option<String>,
    #[validate(length(min = 3))]
    pub description: Option<String>,
    pub active: Option<bool>,
    pub completed: Option<bool>,
    pub deleted: Option<bool>,
}

impl DTOJsonUpdateTodoItem {
    pub fn one_field_is_required(&self) -> Result<(), AppError> {
        if let (None, None, None, None, None) = (
            self.name.clone(),
            self.description.clone(),
            self.active,
            self.completed,
            self.deleted,
        ) {
            return Err(AppError::BAD_REQUEST.message("Должно быть хотя бы одно поле!".to_string()));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DTOUpdateTodoItem {
    pub todolist_id: i64,
    pub todoitem_id: i64,
    #[validate(length(min = 3))]
    pub name: Option<String>,
    #[validate(length(min = 3))]
    pub description: Option<String>,
    pub active: Option<bool>,
    pub completed: Option<bool>,
    pub deleted: Option<bool>,
}

impl DTOUpdateTodoItem {
    pub fn new(
        json: DTOJsonUpdateTodoItem,
        todolist_id: TodoListId,
        todoitem_id: TodoItemId,
    ) -> Self {
        Self {
            todolist_id,
            todoitem_id,
            name: json.name,
            description: json.description,
            active: json.active,
            completed: json.completed,
            deleted: json.deleted,
        }
    }
}
