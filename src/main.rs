use actix_web::web::Data;
use actix_web::{delete, get, post, put, web::Json, web::Path, App, HttpServer};
use error::TodoError;
use models::todo::Todo;
use uuid;
mod models;
use crate::db::{todo_data_trait::TodoDataTrait, Database};
mod db;
use crate::models::todo::{AddNewTodoItem, DeleteTodoItemURL, UpdateTodoItemURL};
use validator::Validate;
mod error;

#[get("/gettodoitems")]
async fn get_todo_items(db: Data<Database>) -> Result<Json<Vec<Todo>>, TodoError> {
    let todos = Database::get_all_todo_items(&db).await;
    match todos {
        Some(found_todos) => Ok(Json(found_todos)),
        None => Err(TodoError::NoTodosFound),
    }
}

#[post("/addnewtodoitem")]
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

#[put("/edittodoitem/{uuid}")]
async fn update_todo_item(
    update_todo_item_url: Path<UpdateTodoItemURL>,
    db: Data<Database>,
) -> Result<Json<Todo>, TodoError> {
    let uuid: String = update_todo_item_url.into_inner().uuid;
    let updated_result = Database::update_todo(&db, uuid).await;
    match updated_result {
        Some(updated_todo) => Ok(Json(updated_todo)),
        None => Err(TodoError::NoSuchTodoFound),
    }
}

#[delete("/deletetodoitem/{uuid}")]
async fn delete_todo_item(
    delete_todo_item_url: Path<DeleteTodoItemURL>,
    db: Data<Database>,
) -> Result<Json<Todo>, TodoError> {
    let uuid = delete_todo_item_url.into_inner().uuid;
    let deleted_todo = Database::delete_todo(&db, uuid.clone()).await;
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
            .service(update_todo_item)
            .service(delete_todo_item)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
