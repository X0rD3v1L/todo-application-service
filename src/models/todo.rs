//todo.rs
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Deserialize, Serialize)]
pub struct AddNewTodoItem {
    #[validate(length(min = 1, message = "todo task name required"))]
    pub task_name: String,
}
#[derive(Validate, Deserialize, Serialize)]
pub struct DeleteTodoItemURL {
    pub uuid: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct UpdateTodoTaskItem {
    pub uuid: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct UpdateTodoTaskStatus {
    pub uuid: String,
    #[validate(length(min = 1, message = "todo task name required"))]
    pub task_name: String,
}

#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct Todo {
    pub uuid: String,
    pub task_name: String,
    pub is_completed: bool,
}

impl Todo {
    pub fn new(uuid: String, task_name: String, is_completed: bool) -> Todo {
        Todo {
            uuid,
            task_name,
            is_completed,
        }
    }
}
