use blog_proto::{
    admin_service_client::AdminServiceClient,
    get_admin_request::{ByAuth, ById, Condition},
    EditAdminRequest, GetAdminRequest, ListAdminRequest, ToggleAdminRequest,
};

#[tokio::test]
async fn test_create_admin() {
    let mut client = AdminServiceClient::connect("http://127.0.0.1:49527")
        .await
        .unwrap();
    let resp = client
        .create_admin(tonic::Request::new(blog_proto::CreateAdminRequest {
            email: "team@axum.rs".into(),
            password: "axum.rs".into(),
        }))
        .await
        .unwrap();
    let reply = resp.into_inner();
    assert!(reply.id > 0);
}

#[tokio::test]
async fn test_edit_admin() {
    let mut client = AdminServiceClient::connect("http://127.0.0.1:49527")
        .await
        .unwrap();
    let resp = client
        .edit_admin(tonic::Request::new(EditAdminRequest {
            id: 1,
            email: "team@axum.rs".into(),
            password: "axum.rs".into(),
            new_password: Some("axum.rs".into()),
        }))
        .await
        .unwrap();
    let reply = resp.into_inner();
    println!("{:?}", reply)
}
#[tokio::test]
async fn test_toggle_admin() {
    let mut client = AdminServiceClient::connect("http://127.0.0.1:49527")
        .await
        .unwrap();
    let resp = client
        .toggle_admin(tonic::Request::new(ToggleAdminRequest { id: 1 }))
        .await
        .unwrap();
    let reply = resp.into_inner();
    println!("{:?}", reply)
}
#[tokio::test]
async fn test_byid_get_admin() {
    let mut client = AdminServiceClient::connect("http://127.0.0.1:49527")
        .await
        .unwrap();
    let condition = Condition::ById(ById {
        id: 1,
        is_del: None,
    });
    let resp = client
        .get_admin(tonic::Request::new(GetAdminRequest {
            condition: Some(condition),
        }))
        .await
        .unwrap();
    let reply = resp.into_inner();
    assert!(reply.admin.is_some());
}

#[tokio::test]
async fn test_byauth_get_admin_as_login() {
    let mut client = AdminServiceClient::connect("http://127.0.0.1:49527")
        .await
        .unwrap();
    let condition = Condition::ByAuth(ByAuth {
        email: "team@axum.rs".into(),
        password: "axum.rs".into(),
    });
    let resp = client
        .get_admin(tonic::Request::new(GetAdminRequest {
            condition: Some(condition),
        }))
        .await
        .unwrap();
    let reply = resp.into_inner();
    assert!(reply.admin.is_some());
}

#[tokio::test]
async fn test_list_admin() {
    let mut client = AdminServiceClient::connect("http://127.0.0.1:49527")
        .await
        .unwrap();
    let resp = client
        .list_admin(tonic::Request::new(ListAdminRequest {
            email: Some("@axum.rs".into()),
            is_del: Some(false),
        }))
        .await
        .unwrap();
    let reply = resp.into_inner();
    assert!(reply.admins.len() > 0)
}
