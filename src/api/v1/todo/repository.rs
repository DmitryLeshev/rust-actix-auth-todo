use actix_web::{web::Data, FromRequest};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use std::{
    collections::HashMap,
    future::{ready, Ready},
    ops::Deref,
    sync::Arc,
};
use tracing::instrument;

use crate::{
    api::v1::todo::models::TodoList,
    app::{error::AppError, state::AppState},
    common::models::RowCount,
};

use super::models::{
    DTOCreateTodoItem, DTOCreateTodoList, DTODeleteTodoItem, DTODeleteTodoList, DTOGetTodoItems,
    DTOGetTodoLists, DTOUpdateTodoItem, DTOUpdateTodoList, TodoItem, TodoItems, TodoLists,
};

pub struct TodoRepository {
    pool: Arc<Pool<Postgres>>,
}

impl TodoRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool: pool }
    }

    pub async fn get_todolists(&self, dto: DTOGetTodoLists) -> Result<(TodoLists, i64), AppError> {
        sqlx::query("drop table if exists tmp_todolists cascade")
            .execute(&*self.pool)
            .await?;
        let like = if let Some(name) = dto.name {
            name.to_lowercase()
        } else {
            "".to_string()
        };
        let sql = format!("select * into tmp_todolists from todolist where todolist.account_id = {} and lower(todolist.name) like '%{}%'", dto.account_id, like);
        sqlx::query(&sql).execute(&*self.pool).await?;
        let sql = format!(
            "select * from tmp_todolists offset {} limit {}",
            dto.offset, dto.limit
        );
        let todolists = sqlx::query_as::<_, TodoList>(&sql)
            .fetch_all(&*self.pool)
            .await?;

        let row_count = sqlx::query_as::<_, RowCount>("select count(*) from tmp_todolists")
            .fetch_one(&*self.pool)
            .await?;

        sqlx::query("drop table if exists tmp_todolists cascade")
            .execute(&*self.pool)
            .await?;

        Ok((todolists, row_count.count))
    }

    pub async fn get_todoitems(&self, dto: DTOGetTodoItems) -> Result<(TodoItems, i64), AppError> {
        sqlx::query("drop table if exists tmp_todoitems cascade")
            .execute(&*self.pool)
            .await?;
        let like = if let Some(name) = dto.name {
            name.to_lowercase()
        } else {
            "".to_string()
        };
        let sql = format!("select * into tmp_todoitems from todoitem as t where t.todolist_id = {} and lower(t.name) like '%{}%'", dto.todolist_id, like);
        sqlx::query(&sql).execute(&*self.pool).await?;
        let sql = format!(
            "select * from tmp_todoitems offset {} limit {}",
            dto.offset, dto.limit
        );
        let todoitems = sqlx::query_as::<_, TodoItem>(&sql)
            .fetch_all(&*self.pool)
            .await?;

        let row_count = sqlx::query_as::<_, RowCount>("select count(*) from tmp_todoitems")
            .fetch_one(&*self.pool)
            .await?;

        sqlx::query("drop table if exists tmp_todoitems cascade")
            .execute(&*self.pool)
            .await?;

        Ok((todoitems, row_count.count))
    }

    pub async fn create_todolist(&self, dto: DTOCreateTodoList) -> Result<TodoList, AppError> {
        let sql = format!(
            "insert into todolist (account_id, name) values ('{}', '{}') returning *",
            dto.account_id.unwrap(),
            dto.name
        );
        let todolist = sqlx::query_as::<_, TodoList>(&sql)
            .fetch_one(&*self.pool)
            .await?;
        Ok(todolist)
    }

    pub async fn create_todoitem(&self, dto: DTOCreateTodoItem) -> Result<TodoItem, AppError> {
        let description = if let Some(description) = dto.description {
            description
        } else {
            "null".to_string()
        };
        let sql = format!(
            "insert into todoitem (todolist_id, name, description) values ('{}', '{}', '{}') returning *",
            dto.todolist_id.unwrap(),
            dto.name,
            description
        );
        let todoitem = sqlx::query_as::<_, TodoItem>(&sql)
            .fetch_one(&*self.pool)
            .await?;
        Ok(todoitem)
    }

    pub async fn delete_todolist(&self, dto: DTODeleteTodoList) -> Result<(), AppError> {
        let sql = format!(
            "delete from todolist where account_id = {} and todolist_id = {}",
            dto.account_id, dto.todolist_id
        );
        sqlx::query(&sql).execute(&*self.pool).await?;
        Ok(())
    }

    pub async fn delete_todoitem(&self, dto: DTODeleteTodoItem) -> Result<(), AppError> {
        let sql = format!(
            "delete from todoitem where todolist_id = {} and todoitem_id = {}",
            dto.todolist_id, dto.todoitem_id
        );
        sqlx::query(&sql).execute(&*self.pool).await?;
        Ok(())
    }

    pub async fn update_todolist(&self, dto: DTOUpdateTodoList) -> Result<TodoList, AppError> {
        let now = Utc::now().naive_utc();
        let sql = format!("update todolist set name = '{}', updated_at = '{}' where account_id = {} and todolist_id = {} returning *", dto.name, now, dto.account_id, dto.todolist_id);
        let todolist = sqlx::query_as::<_, TodoList>(&sql)
            .fetch_one(&*self.pool)
            .await?;
        Ok(todolist)
    }

    pub async fn update_todoitem(&self, dto: DTOUpdateTodoItem) -> Result<TodoItem, AppError> {
        let now = Utc::now().naive_utc();
        let str_true = String::from("true");
        let str_false = String::from("false");
        let sql = {
            let mut hash_map = HashMap::new();
            if let Some(name) = dto.name {
                hash_map.insert("name", name);
            };
            if let Some(description) = dto.description {
                hash_map.insert("description", description);
            };
            if let Some(active) = dto.active {
                hash_map.insert(
                    "active",
                    if active {
                        str_true.clone()
                    } else {
                        str_false.clone()
                    },
                );
            };
            if let Some(completed) = dto.completed {
                hash_map.insert(
                    "completed",
                    if completed {
                        str_true.clone()
                    } else {
                        str_false.clone()
                    },
                );
            };
            if let Some(deleted) = dto.deleted {
                hash_map.insert(
                    "deleted",
                    if deleted {
                        str_true.clone()
                    } else {
                        str_false.clone()
                    },
                );
            };
            hash_map.insert("updated_at", now.to_string());
            hash_map
                .iter()
                .map(|(key, value)| format!("{} = '{}'", key, value))
                .collect::<Vec<String>>()
                .join(", ")
        };
        let sql = format!(
            "update todoitem set {} where todolist_id = {} and todoitem_id = {} returning *",
            sql, dto.todolist_id, dto.todoitem_id
        );
        let todoitem = sqlx::query_as::<_, TodoItem>(&sql)
            .fetch_one(&*self.pool)
            .await?;
        Ok(todoitem)
    }
}

impl FromRequest for TodoRepository {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    #[instrument(skip(req, payload))]
    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let app_state_result = Data::<AppState>::from_request(req, payload).into_inner();

        match app_state_result {
            Ok(app_state) => {
                let pool = app_state.deref().pool.clone();
                ready(Ok(TodoRepository::new(Arc::new(pool))))
            }
            _ => ready(Err(AppError::DB_ERROR.default())),
        }
    }
}
