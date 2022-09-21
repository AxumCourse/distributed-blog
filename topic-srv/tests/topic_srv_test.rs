use blog_proto::{
    topic_service_client::TopicServiceClient, CreateTopicRequest, EditTopicRequest,
    GetTopicRequest, ToggleTopicRequest,
};

#[tokio::test]
async fn test_create_topic() {
    let mut client = TopicServiceClient::connect("http://127.0.0.1:29527")
        .await
        .unwrap();
    let request = tonic::Request::new(CreateTopicRequest {
        title: "Hello,世界".into(),
        category_id: 1.into(),
        content: "欢迎来到axum.rs".into(),
        summary: None,
    });
    let resp = client.create_topic(request).await.unwrap();
    let reply = resp.into_inner();
    assert!(reply.id == 1);
}

#[tokio::test]
async fn test_edit_topic() {
    let mut client = TopicServiceClient::connect("http://127.0.0.1:29527")
        .await
        .unwrap();
    let request = tonic::Request::new(EditTopicRequest {
        id: 1.into(),
        title: "Hello,世界!".into(),
        category_id: 1.into(),
        content: "欢迎来到axum.rs!".into(),
        summary: Some("axum.rs yyds".to_string()),
    });
    let resp = client.edit_topic(request).await.unwrap();
    let reply = resp.into_inner();
    assert!(reply.id == 1);
}

#[tokio::test]
async fn test_toggle_topic() {
    let mut client = TopicServiceClient::connect("http://127.0.0.1:29527")
        .await
        .unwrap();
    let request = tonic::Request::new(ToggleTopicRequest { id: 1.into() });
    let resp = client.toggle_topic(request).await.unwrap();
    let reply = resp.into_inner();
    assert!(reply.id == 1);
}

#[tokio::test]
async fn test_get_topic() {
    let mut client = TopicServiceClient::connect("http://127.0.0.1:29527")
        .await
        .unwrap();
    let request = tonic::Request::new(GetTopicRequest {
        id: 1.into(),
        is_del: None,
    });
    let resp = client.get_topic(request).await.unwrap();
    let reply = resp.into_inner();
    assert!(reply.topic.is_some());
    println!("{:?}", reply.topic.unwrap().dateline);
}
