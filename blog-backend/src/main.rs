use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use blog_proto::{
    admin_service_client::AdminServiceClient, category_service_client::CategoryServiceClient,
    topic_service_client::TopicServiceClient,
};
use tera::Tera;

mod form;
mod handler;
mod model;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:59527";

    let cate = CategoryServiceClient::connect("http://127.0.0.1:19527")
        .await
        .unwrap();
    let topic = TopicServiceClient::connect("http://127.0.0.1:29527")
        .await
        .unwrap();
    let admin = AdminServiceClient::connect("http://127.0.0.1:49527")
        .await
        .unwrap();
    let tera = Tera::new("blog-backend/templates/**/*.html").unwrap();

    let m_router = Router::new().route("/cate", get(handler::list_cate)).route(
        "/cate/add",
        get(handler::add_cate_ui).post(handler::add_cate),
    );

    let app = Router::new()
        .nest("/m", m_router)
        .route("/", get(handler::index))
        .route("/login", get(handler::login_ui).post(handler::login))
        .route("/logout", get(handler::logout))
        .layer(Extension(Arc::new(model::AppState {
            tera,
            admin,
            cate,
            topic,
        })));

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
