use axum::{
    extract::{Query, Path}, 
    response::{Html, IntoResponse}, 
    routing::{get, post, get_service}, 
    Router, 
    http::StatusCode, 
    Json
};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_http::services::ServeFile;
use std::net::SocketAddr;


#[derive(Deserialize)]
struct ParameterList {
    start: usize,
    end: usize
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, Hash, PartialEq)] 
struct User { 
    id: u64, 
    username: String, 
} 



#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // Route all requests on "/" endpoint to anonymous handler

    // a handler is a async function which returns something that implements axum::response::IntoResponse


    let app = Router::new().route("/", get(handler))
    .route("/json/:name", get(json_hello))
    .route("/static", get_service(ServeFile::new("static/index.html"))
    .handle_error(|err| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
    }))
    .route("/user", post(create_user))
    .fallback(root);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // hyper::server::Server
    tracing::info!("App running on {}", addr);
    println!("try:  http://localhost:3000/?start=50&end=100");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn handler(Query(range): Query<ParameterList>) -> Html<String>{
    let random_number = thread_rng().gen_range(range.start..range.end);
    Html(format!("<h1>Random Number: {}</h1>", random_number))
}

async fn json_hello(Path(fname): Path<String>) -> impl IntoResponse {
    let name = fname.as_str();
    let hello = String::from("Hello ");

    (
        StatusCode::OK, 
        Json(json!(
            {
                "message" : format!("{} {} world", name, hello)
            }
    )))
}

async fn root() -> (StatusCode ,Html<String>){
    (StatusCode::OK ,Html(format!("<h1>Home page</h1>")))
}

async fn create_user(Json(payload): Json<User>) -> impl IntoResponse {
    let user = User {
        id: 133332,
        username: payload.username
    };

    (StatusCode::CREATED, Json(user))
}