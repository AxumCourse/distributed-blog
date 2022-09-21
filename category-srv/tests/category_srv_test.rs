use blog_proto::{
    category_service_client::CategoryServiceClient, CreateCategoryRequest, EditCategoryRequest,
    GetCategoryRequest, ListCategoryRequest, ToggleCategoryRequest,
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

#[tokio::test]
async fn test_delete_category() {
    let mut client = CategoryServiceClient::connect("http://127.0.0.1:19527")
        .await
        .unwrap();
    let request = tonic::Request::new(ToggleCategoryRequest { id: 1 });
    let reply = client.toggle_category(request).await.unwrap();
    let reply = reply.into_inner();
    assert!(reply.id == 1);
    assert!(reply.is_del);
}
#[tokio::test]
async fn test_undelete_category() {
    let mut client = CategoryServiceClient::connect("http://127.0.0.1:19527")
        .await
        .unwrap();
    let request = tonic::Request::new(ToggleCategoryRequest { id: 1 });
    let reply = client.toggle_category(request).await.unwrap();
    let reply = reply.into_inner();
    assert!(reply.id == 1);
    assert!(reply.is_del == false);
}

#[tokio::test]
async fn test_delete_notexists_category() {
    let mut client = CategoryServiceClient::connect("http://127.0.0.1:19527")
        .await
        .unwrap();
    let request = tonic::Request::new(ToggleCategoryRequest { id: 100 });
    let reply = client.toggle_category(request).await;
    assert!(reply.is_err());
}

#[tokio::test]
async fn test_list_category() {
    let mut client = CategoryServiceClient::connect("http://127.0.0.1:19527")
        .await
        .unwrap();
    let request = tonic::Request::new(ListCategoryRequest {
        name: Some("AXUM.RS".to_string()),
        is_del: Some(false),
    });
    let reply = client.list_category(request).await.unwrap();
    let reply = reply.into_inner();
    assert!(reply.categories.len() > 0);
}
