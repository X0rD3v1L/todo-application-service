use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::Display;

// Custom error types for todo-related errors
#[derive(Debug, Display)]
pub enum TodoError {
    NoTodosFound,
    TodoCreationFailure,
    NoSuchTodoFound,
}

impl ResponseError for TodoError {

    // Generates an HTTP response for the error
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    // Gets the HTTP status code for the error
    fn status_code(&self) -> StatusCode {
        match self {
            TodoError::NoTodosFound => StatusCode::NOT_FOUND,
            TodoError::TodoCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
            TodoError::NoSuchTodoFound => StatusCode::NOT_FOUND,
        }
    }
}
