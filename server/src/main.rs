use axum::{
    body::Body,
    response::{Response, IntoResponse},
    extract::{Request, DefaultBodyLimit},
    routing::{get, post},
    Router,
    http::StatusCode,
};
/*
#[macro_use]
extern crate diesel;
*/
#[macro_use]
extern crate diesel_derives;


use tower::util::ServiceExt;

use tower_http::services::{ServeDir, ServeFile};

pub mod schema;
pub mod database;
pub mod image_dir;
pub mod file;
pub mod api;

#[tokio::main]
async fn main() {
    image_dir::generate_image_dir();
    let api = {
        use api::*;
        Router::new()
            .route("/secret", get(secret))
            .route("/secret_json", get(json_response))
            .route("/get_image_dir", get(image_dir::get_image_dir))
            .route("/reload_image_dir", get(reload_image_dir))
            .route("/save_image",
                   post(save_image)
                   .layer(DefaultBodyLimit::max(10 * 1000 * 1000))
                   )
            .route("/set_background",
                   post(set_background_handler)
                   .layer(DefaultBodyLimit::max(10 * 1000 * 1000))
                   )
            .route("/user/:username/profile", get(user_profile))
            .route("/create_user", post(create_user))
            .route("/get_users", get(get_users))
            
            .route("/current_background", post(get_current_background))
    };
    let app = Router::new()
        .nest("/api", api)
        .nest_service(
            "/public",
            get(static_root)
            )
        .fallback_service(get(root));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:64009").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn static_root(req: Request<Body>) -> Response {
    let res = ServeDir::new("../storage/public").oneshot(req).await.unwrap();
    let status = res.status();
    match status {
        StatusCode::NOT_FOUND => {
            (StatusCode::NOT_FOUND, "404 public file not found")
                .into_response()
        }
        _ => {
            res.into_response()
        }
    }
}

async fn root(req: Request<Body>) -> Response {
    let res = ServeDir::new("../dist")
        .fallback(ServeFile::new("../dist/index.html"))
        .oneshot(req).await.unwrap();
    res.into_response()
}
