use crate::{
    model::Todo, response::{GenericResponse, SingleTodoResponse, TodoData, TodoListResponse}, schema::QueryOptions, WebResult, DB
};
use chrono::prelude::*;
use uuid::Uuid;
use warp::{
    http::StatusCode,
    reply::{json, with_status},
    Reply,
};

pub async fn health_check_handler() -> WebResult<impl Reply> {
    const MESSAGE: &str = "Alive and kicking.";

    let response = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };

    Ok(json(response))
}

pub async fn todo_list_handler(opts: QueryOptions, db: DB) -> WebResult<impl Reply> {
    let todos = db.lock().await;

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) -1) * limit;

    let todos: Vec<Todo> = todos.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = TodoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos
    };

    Ok(json(&json_response))
}

pub async fn create_todo_handler(mut body: Todo, db: DB) -> WebResult<impl Reply> {
    if body.title.is_empty() || body.title.is_empty() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: "Title and Body cannot be empty".to_string()
        };

        return Ok(with_status(json(&error_response), StatusCode::BAD_REQUEST))
    }

    let mut vec = db.lock().await;

    for todo in vec.iter() {
        if todo.title == body.title {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("Todo with title: '{}' already exists", todo.title)
            };

            return Ok(with_status(json(&error_response), StatusCode::CONFLICT))
        }
    }

    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    body.id = Some(id);
    body.completed = Some(false);
    body.created_at = Some(now);
    body.updated_at = Some(now);

    let todo = body.to_owned();
    vec.push(todo);

    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData { todo: body },
    };

    Ok(with_status(json(&json_response), StatusCode::CREATED))
}

pub async fn find_single_todo_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let todos = db.lock().await;

    let todo = todos.iter().find(|t| t.id.as_ref().unwrap() == &id);

    match todo {
        Some(todo) => {
            let json_response = SingleTodoResponse {
                status: "success".to_string(),
                data: TodoData { todo: todo.clone() },
            };

            Ok(with_status(json(&json_response), StatusCode::OK))
        }
        None => {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("Todo with id: '{}' not found", id),
            };

            Ok(with_status(json(&error_response), StatusCode::NOT_FOUND))
        }
    }
}

pub async fn update_todo_handler(id: String, body: Todo, db: DB) -> WebResult<impl Reply> {
    let mut vec = db.lock().await;

    let todo = vec.iter_mut().find(|t| t.id.as_ref().unwrap() == &id);

    match todo {
        Some(todo) => {
            todo.title = body.title;
            todo.content = body.content;
            todo.completed = body.completed;
            todo.updated_at = Some(Utc::now());

            let json_response = SingleTodoResponse {
                status: "success".to_string(),
                data: TodoData { todo: todo.clone() },
            };

            Ok(with_status(json(&json_response), StatusCode::OK))
        }
        None => {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("Todo with id: '{}' not found", id),
            };

            Ok(with_status(json(&error_response), StatusCode::NOT_FOUND))
        }
    }
}
