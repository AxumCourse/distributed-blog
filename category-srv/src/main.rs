use blog_proto::category_service_server::CategoryServiceServer;

mod server;
#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:19527";
    println!("category-srv run at: {}", addr);

    let category_srv = server::Category::new();
    tonic::transport::Server::builder()
        .add_service(CategoryServiceServer::new(category_srv))
        .serve(addr.parse().unwrap())
        .await
        .unwrap();
}
