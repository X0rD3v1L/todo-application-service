use actix_web::web::Data;
use actix_web::{delete, get, post, put, web::Json, App, HttpServer};
use error::TodoError;
use models::todo::Todo;
use uuid;
mod models;
use crate::db::{todo_data_trait::TodoDataTrait, Database};
mod db;
use crate::models::todo::{AddNewTodoItem, UpdateTodoTaskItem, UpdateTodoTaskStatus};
use validator::Validate;
mod error;

#[get("/api/todos")]
async fn get_todo_items(db: Data<Database>) -> Result<Json<Vec<Todo>>, TodoError> {
    let todos = Database::get_all_todo_items(&db).await;
    match todos {
        Some(found_todos) => Ok(Json(found_todos)),
        None => Err(TodoError::NoTodosFound),
    }
}

#[post("/api/todo/new")]
async fn add_new_todo_item(
    body: Json<AddNewTodoItem>,
    db: Data<Database>,
) -> Result<Json<Todo>, TodoError> {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let task_name = body.task_name.clone();

            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_todo =
                Database::add_new_todo(&&db, Todo::new(String::from(new_uuid), task_name, false))
                    .await;

            match new_todo {
                Some(created) => Ok(Json(created)),
                None => Err(TodoError::TodoCreationFailure),
            }
        }
        Err(_) => Err(TodoError::TodoCreationFailure),
    }
}

#[put("/api/todo/editstatus")]
async fn update_todo_task_item(
    body: Json<UpdateTodoTaskItem>,
    db: Data<Database>,
) -> Result<Json<Todo>, TodoError> {
    let uuid: String = body.uuid.clone();
    let updated_result = Database::update_todo_task(&db, uuid).await;
    match updated_result {
        Some(updated_todo) => Ok(Json(updated_todo)),
        None => Err(TodoError::NoSuchTodoFound),
    }
}

#[put("/api/todo/edittask")]
async fn update_todo_task_status(
    body: Json<UpdateTodoTaskStatus>,
    db: Data<Database>,
) -> Result<Json<Todo>, TodoError> {
    let uuid: String = body.uuid.clone();
    let edited_task_name = body.task_name.clone();
    let updated_result = Database::update_todo_status(&db, uuid, edited_task_name).await;
    match updated_result {
        Some(updated_todo) => Ok(Json(updated_todo)),
        None => Err(TodoError::NoSuchTodoFound),
    }
}

#[delete("/api/todo/delete/{uuid}")]
async fn delete_todo_item(
    body: Json<UpdateTodoTaskStatus>,
    db: Data<Database>,
) -> Result<Json<Todo>, TodoError> {
    let uuid: String = body.uuid.clone();
    let deleted_todo = Database::delete_todo(&db, uuid).await;
    match deleted_todo {
        Some(deleted) => Ok(Json(deleted)),
        None => Err(TodoError::NoSuchTodoFound),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
        .await
        .expect("Error connecting to database");

    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_todo_items)
            .service(add_new_todo_item)
            .service(update_todo_task_item)
            .service(update_todo_task_status)
            .service(delete_todo_item)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
