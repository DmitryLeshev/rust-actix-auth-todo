use serde::Deserialize;

pub const DEFAULT_LIMIT_TODOITEM: i64 = 10;
pub const DEFAULT_PAGE_TODOITEM: i64 = 1;

#[derive(Debug, Deserialize, Clone)]
pub struct QuerySearchTodoItems {
    pub name: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}
