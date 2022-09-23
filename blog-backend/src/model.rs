use blog_proto::{
    admin_service_client::AdminServiceClient, category_service_client::CategoryServiceClient,
    topic_service_client::TopicServiceClient,
};
use tera::Tera;

pub struct AppState {
    pub tera: Tera,
    pub cate: CategoryServiceClient<tonic::transport::Channel>,
    pub topic: TopicServiceClient<tonic::transport::Channel>,
    pub admin: AdminServiceClient<tonic::transport::Channel>,
}
