use blog_proto::{category_service_client::CategoryServiceClient, CreateCategoryRequest};

#[tokio::test]
async fn test_create_category() {
    let mut client = CategoryServiceClient::connect("http://127.0.0.1:19527")
        .await
        .unwrap();
    let request = tonic::Request::new(CreateCategoryRequest {
        name: "分类1".into(),
    });
    let reply = client.create_category(request).await.unwrap();
    let reply = reply.into_inner();
    assert!(reply.id > 0);
}
