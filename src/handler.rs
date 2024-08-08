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

