use crate::{
    model::Todo,
    response::{GenericResponse, SingleTodoResponse, TodoData, TodoListResponse},
    WebResult, DB,
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

