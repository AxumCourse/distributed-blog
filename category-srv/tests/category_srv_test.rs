use blog_proto::{
    category_service_client::CategoryServiceClient, CreateCategoryRequest, EditCategoryRequest,
    GetCategoryRequest,
};

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
#[tokio::test]
async fn test_edit_category() {
    let mut client = CategoryServiceClient::connect("http://127.0.0.1:19527")
        .await
        .unwrap();
    let request = tonic::Request::new(EditCategoryRequest {
        id: 1,
        name: "axum.rs".into(),
    });
    let reply = client.edit_category(request).await.unwrap();
    let reply = reply.into_inner();
    assert!(reply.id > 0);
    assert!(reply.ok);
}
#[tokio::test]
async fn test_get_category() {
    let mut client = CategoryServiceClient::connect("http://127.0.0.1:19527")
        .await
        .unwrap();
    let request = tonic::Request::new(GetCategoryRequest {
        id: 1,
        is_del: None,
    });
    let reply = client.get_category(request).await.unwrap();
    let reply = reply.into_inner();
    assert!(reply.category.is_some());
    assert!(reply.category.unwrap().id == 1);
}
#[tokio::test]
async fn test_get_notexists_category() {
    let mut client = CategoryServiceClient::connect("http://127.0.0.1:19527")
        .await
        .unwrap();
    let request = tonic::Request::new(GetCategoryRequest {
        id: 100,
        is_del: Some(true),
    });
    let reply = client.get_category(request).await.unwrap();
    let reply = reply.into_inner();
    assert!(reply.category.is_none());
}
