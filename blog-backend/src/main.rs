use std::{env, sync::Arc};

use axum::{routing::get, Extension, Router};
use blog_auth::Jwt;
use blog_proto::{
    admin_service_client::AdminServiceClient, category_service_client::CategoryServiceClient,
    topic_service_client::TopicServiceClient,
};
use tera::Tera;

mod form;
mod handler;
mod middleware;
mod model;

#[tokio::main]
async fn main() {
    let addr = env::var("ADDR").unwrap_or("0.0.0.0:59527".to_string());
    let jwt_secret =
        env::var("JWT_SECRET").unwrap_or("PRFw6DQuWfFSQZjuUCnCeLhLXfWetA3r".to_string());
    let jwt_iss = env::var("JWT_ISS").unwrap_or("axum.rs".to_string());
    let jwt_exp = env::var("JWT_EXP").unwrap_or("120".to_string());
    let jwt_exp = jwt_exp.parse().unwrap_or(120);

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
    let jwt = Jwt::new(jwt_secret, jwt_exp, jwt_iss);

    let m_router = Router::new()
        .route("/cate", get(handler::list_cate))
        .route(
            "/cate/add",
            get(handler::add_cate_ui).post(handler::add_cate),
        )
        .layer(axum::middleware::from_extractor::<middleware::Auth>());

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
            jwt,
        })));

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
