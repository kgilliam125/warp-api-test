
mod handler;
mod model;
mod response;
mod schema;

use model::DB;
use warp::{Filter, Rejection};

type WebResult<T> = Result<T, Rejection>;


#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();

    let db = model::init_todo_db();

    // let todo_router = warp::path!("api" / "todos");
    let todo_router = filters::todos(db);

    // let cors = warp::cors()
    // .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    // .allow_origins(vec!["http://localhost:3000/", "http://localhost:8000/"])
    // .allow_headers(vec!["content-type"])
    // .allow_credentials(true);

    let health_router = warp::path!("api" / "health")
        .and(warp::get())
        .and_then(handler::health_check_handler);

    let routes = todo_router
        // .with(cors)
        .with(warp::log("api"))
        .or(health_router);

    println!("Starting server");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}



mod filters {
    use super::handler;
    use super::model::DB;
    use super::schema;
    use warp::Filter;

    pub fn todos(db: DB) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        list_todos(db.clone())
            .or(create_todo(db.clone()))
            // .or(update_todo(db))
            // .or(delete_todo(db))
    }

    pub fn list_todos(db: DB) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("api" / "todos")
            .and(warp::get())
            .and(warp::query::<schema::QueryOptions>())
            .and(with_db(db))
            .and_then(handler::todo_list_handler)
    }

    pub fn create_todo(db: DB) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("api" / "todos")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db))
            .and_then(handler::create_todo_handler)
    }

    fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}