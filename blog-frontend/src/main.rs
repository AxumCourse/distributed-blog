use std::sync::Arc;

use axum::{extract::Extension, routing::get, Router};
use blog_proto::{
    category_service_client::CategoryServiceClient, topic_service_client::TopicServiceClient,
};
use tera::Tera;

mod handler;
mod model;
#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:39527";

    let cate = CategoryServiceClient::connect("http://127.0.0.1:19527")
        .await
        .unwrap();
    let topic = TopicServiceClient::connect("http://127.0.0.1:29527")
        .await
        .unwrap();

    let tera = Tera::new("blog-frontend/templates/**/*.html").unwrap();

    let app = Router::new()
        .route("/", get(handler::index))
        .route("/detail/:id", get(handler::detail))
        .layer(Extension(Arc::new(model::AppState::new(cate, topic, tera))));
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
