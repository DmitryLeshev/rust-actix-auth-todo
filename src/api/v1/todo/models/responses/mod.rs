mod get_todoitems;
mod get_todolists;

use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ResponseDefaultTodolist;

pub use get_todoitems::*;
pub use get_todolists::*;
