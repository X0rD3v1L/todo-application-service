# Todo App Service API Documentation

This document provides an overview of the Todo App Service API endpoints and their functionality.

### Creating SurrealDB Instance

To create a SurrealDB instance named `todoapp.db` with the username `root` and password `root`, run the following command:

`surreal start file:todoapp.db --user root --password root`

Replace `root` with your desired username and password for accessing the database.

## Database Credentials
Database used: SurrealDB

Please create a `.env` file in the root directory of the project and add the following variables:

DB_USERNAME=your_username

DB_PASSWORD=your_password

## Endpoints

### GET /api/v1/todos

- **Description:** Retrieve all todo items.
- **Method:** `GET`
- **Request Body:** None
- **Response Body:** JSON array of todo items.
  ```json
  [
    {
        "uuid": "3b2cb991a5c64b54a2b7196e8c104073",
        "task_name": "Finish reading articles",
        "is_completed": true
    },
    {
        "uuid": "518cc1436d2342f2952a37e5525ff6d1",
        "task_name": "Learn Async Rust",
        "is_completed": false
    },
    {
        "uuid": "77aff0a66de047e59a9475f21412d92c",
        "task_name": "Complete Rustlings",
        "is_completed": false
    },
    {
        "uuid": "7dc55978c82145f397e9373df2deaa1c",
        "task_name": "Learn Actix Web",
        "is_completed": true
    }
  ]
- **Response Codes:**
  - `200 OK`: Successful request.
  - `404 Not Found`: No todo items found.

### POST /api/v1/todos/new

- **Description:** Add a new todo item.
- **Method:** `POST`
- **Request Body:**
  - `task_name` (string, required): Name of the new todo task.
  ```json
  {
    "task_name": "Finish reading articles"
  }
- **Response Body:** None
- **Response Codes:**
  - `201 Created`: Todo item created successfully.
  - `500 Internal Server Error`: Todo item creation failed.

### PUT /api/v1/todos/edit/status

- **Description:** Update the status of a todo item.
- **Method:** `PUT`
- **Request Body:**
  - `uuid` (string, required): UUID of the todo item to update.
  - `is_completed` (boolean, required): New completion status of the todo item.
  ```json
  {
    "uuid": "3b2cb991a5c64b54a2b7196e8c104073",
    "is_completed": true
  }
- **Response Body:** None
- **Response Codes:**
  - `200 OK`: Todo item status updated successfully.
  - `404 Not Found`: Todo item not found.

### PUT /api/v1/todos/edit/task

- **Description:** Update the task name of a todo item.
- **Method:** `PUT`
- **Request Body:**
  - `uuid` (string, required): UUID of the todo item to update.
  - `task_name` (string, required): New task name for the todo item.
  ```json
  {
    "uuid": "3b2cb991a5c64b54a2b7196e8c104073",
    "task_name": "Finish reading all pending articles"
  }
- **Response Body:** None
- **Response Codes:**
  - `200 OK`: Todo item task name updated successfully.
  - `404 Not Found`: Todo item not found.

### DELETE /api/v1/todos/delete/{uuid}

- **Description:** Delete a todo item.
- **Method:** `DELETE`
- **Request Params:**
  - `uuid` (string, required): UUID of the todo item to delete.
- **Response Body:** None
- **Response Codes:**
  - `204 No Content`: Todo item deleted successfully.
  - `404 Not Found`: Todo item not found.

## Error Handling

- If an error occurs during any request, the API will return an appropriate HTTP status code along with a response message containing an error message.
- Possible error codes include `400 Bad Request`, `404 Not Found`, `500 Internal Server Error`, etc.

## Models

### Todo Item

- **Fields:**
  - `uuid` (string): Unique identifier of the todo item.
  - `task_name` (string): Name of the todo task.
  - `is_completed` (boolean): Completion status of the todo task.