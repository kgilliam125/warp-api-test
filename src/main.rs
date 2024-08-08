
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

    // let db = model::init_todo_db();

    let health_route = warp::path!("api" / "health")
        .and(warp::get())
        .and_then(handler::health_check_handler);

    let routes = health_route.with(warp::log("api"));

    println!("Starting server");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
