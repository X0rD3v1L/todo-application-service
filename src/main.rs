use actix_web::web::{Data, Path};
use actix_web::{delete, get, post, put, web::Json, App, HttpServer};
use actix_web::{HttpResponse, Responder};
use error::TodoError;
use models::todo::{Todo, UpdateTodoTaskStatus};
use uuid;
mod models;
use crate::db::{todo_data_trait::TodoDataTrait, Database};
mod db;
use crate::models::todo::{AddNewTodoItem, DeleteTodoItemURL, UpdateTodoTaskItem};
use validator::Validate;
mod error;

// Handler to get all todo items
#[get("api/v1/todos")]
async fn get_todo_items(db: Data<Database>) -> Result<Json<Vec<Todo>>, TodoError> {
    let todos = Database::get_all_todo_items(&db).await;
    match todos {
        Some(found_todos) => Ok(Json(found_todos)),
        None => Err(TodoError::NoTodosFound),
    }
}

// Handler to add a new todo item
#[post("api/v1/todos/new")]
async fn add_new_todo_item(body: Json<AddNewTodoItem>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let task_name = body.task_name.clone();

            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_todo =
                Database::add_new_todo(&db, Todo::new(String::from(new_uuid), task_name, false))
                    .await;

            match new_todo {
                Some(_) => Ok(HttpResponse::Created().finish()),
                None => Err(TodoError::TodoCreationFailure),
            }
        }
        Err(_) => Err(TodoError::TodoCreationFailure),
    }
}

// Handler to update the status of a todo item
#[put("api/v1/todos/edit/status")]
async fn update_todo_task_item(
    body: Json<UpdateTodoTaskStatus>,
    db: Data<Database>,
) -> impl Responder {
    let uuid: String = body.uuid.clone();
    let completion_status = body.is_completed.clone();
    let updated_result = Database::update_todo_task(&db, uuid, None, Some(completion_status)).await;
    match updated_result {
        Some(_) => Ok(HttpResponse::Ok().finish()),
        None => Err(TodoError::NoSuchTodoFound),
    }
}

// Handler to update the task name of a todo item
#[put("api/v1/todos/edit/task")]
async fn update_todo_task_status(
    body: Json<UpdateTodoTaskItem>,
    db: Data<Database>,
) -> impl Responder {
    let uuid: String = body.uuid.clone();
    let edited_task_name = body.task_name.clone();
    let updated_result =
        Database::update_todo_task(&db, uuid, Some(edited_task_name.to_string()), None).await;
    match updated_result {
        Some(_) => Ok(HttpResponse::Ok().finish()),
        None => Err(TodoError::NoSuchTodoFound),
    }
}

// Handler to delete a todo item
#[delete("api/v1/todos/delete/{uuid}")]
async fn delete_todo_item(
    delete_todo_item_url: Path<DeleteTodoItemURL>,
    db: Data<Database>,
) -> impl Responder {
    let uuid: String = delete_todo_item_url.into_inner().uuid;
    let deleted_todo = Database::delete_todo(&db, uuid).await;
    match deleted_todo {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(TodoError::NoSuchTodoFound),
    }
}

// Main function to start the HTTP server
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
