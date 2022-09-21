use std::env;

use blog_proto::category_service_server::CategoryServiceServer;
use sqlx::PgPool;

mod server;
#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:19527";
    println!("category-srv run at: {}", addr);

    let dsn = env::var("PG_DSN").unwrap_or("postgres://axum.rs@pg.axum.rs/axum_rs".to_string());
    let pool = PgPool::connect(&dsn).await.unwrap();
    let category_srv = server::Category::new(pool);
    tonic::transport::Server::builder()
        .add_service(CategoryServiceServer::new(category_srv))
        .serve(addr.parse().unwrap())
        .await
        .unwrap();
}
