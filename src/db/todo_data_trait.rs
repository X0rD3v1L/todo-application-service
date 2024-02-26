use crate::db::Database;
use crate::Todo;
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::Error;

#[async_trait]
pub trait TodoDataTrait {
    async fn get_all_todo_items(db: &Data<Database>) -> Option<Vec<Todo>>;
    async fn add_new_todo(db: &Data<Database>, new_todo_item: Todo) -> Option<Todo>;
    async fn update_todo_task(
        db: &Data<Database>,
        uuid: String,
        edited_task_name: Option<String>,
        is_completed: Option<bool>,
    ) -> Option<Todo>;
    async fn delete_todo(db: &Data<Database>, uuid: String) -> Option<Todo>;
}

#[async_trait]
impl TodoDataTrait for Database {

    // Method to get all todo items from the database
    async fn get_all_todo_items(db: &Data<Database>) -> Option<Vec<Todo>> {
        let result = db.client.select("todo").await;
        match result {
            Ok(all_to_items) => Some(all_to_items),
            Err(_) => None,
        }
    }

    // Method to add a new todo item to the database
    async fn add_new_todo(db: &Data<Database>, new_todo_item: Todo) -> Option<Todo> {
        let created_todo = db
            .client
            .create(("todo", new_todo_item.uuid.clone()))
            .content(new_todo_item)
            .await;

        match created_todo {
            Ok(created) => created,
            Err(_) => None,
        }
    }

     // Method to update a todo item in the database
    async fn update_todo_task(
        db: &Data<Database>,
        uuid: String,
        edited_task_name: Option<String>,
        is_completed: Option<bool>,
    ) -> Option<Todo> {
        let find_todo_item: Result<Option<Todo>, Error> = db.client.select(("todo", &uuid)).await;
        match find_todo_item {
            Ok(found) => match found {
                Some(mut todo_item) => {
                    if let Some(edited_task_name) = edited_task_name {
                        todo_item.task_name = edited_task_name;
                    }
                    if let Some(is_completed) = is_completed {
                        todo_item.is_completed = is_completed;
                    }
                    let updated_todo: Result<Option<Todo>, Error> =
                        db.client.update(("todo", &uuid)).merge(todo_item).await;
                    match updated_todo {
                        Ok(updated) => updated,
                        Err(_) => None,
                    }
                }
                None => None,
            },
            Err(_) => None,
        }
    }

    // Method to delete a todo item from the database
    async fn delete_todo(db: &Data<Database>, uuid: String) -> Option<Todo> {
        let deleted_todo: Result<Option<Todo>, Error> = db.client.delete(("todo", &uuid)).await;

        match deleted_todo {
            Ok(deleted) => deleted,
            Err(_) => None,
        }
    }
}
