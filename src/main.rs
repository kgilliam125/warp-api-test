
mod handler;
mod model;
mod response;
mod schema;

use model::DB;
use warp::{Filter, Rejection, http::Method};

type WebResult<T> = Result<T, Rejection>;


#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();

    let db = model::init_todo_db();

    let todo_router = warp::path!("api" / "todos");

    let cors = warp::cors()
    .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    .allow_origins(vec!["http://localhost:3000/", "http://localhost:8000/"])
    .allow_headers(vec!["content-type"])
    .allow_credentials(true);

    let health_route = warp::path!("api" / "health")
        .and(warp::get())
        .and_then(handler::health_check_handler);

    let todo_routes = todo_router
        .and(warp::get())
        .and(warp::query::<schema::QueryOptions>())
        .and(with_db(db.clone()))
        .and_then(handler::todo_list_handler);

    let routes = health_route
        .with(cors)
        .with(warp::log("api"))
        .or(todo_routes);

    println!("Starting server");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
