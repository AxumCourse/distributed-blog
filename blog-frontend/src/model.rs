use blog_proto::{
    category_service_client::CategoryServiceClient, topic_service_client::TopicServiceClient,
};

use tera::Tera;

pub struct AppState {
    pub cate: CategoryServiceClient<tonic::transport::Channel>,
    pub topic: TopicServiceClient<tonic::transport::Channel>,
    pub tera: Tera,
}

impl AppState {
    pub fn new(
        cate: CategoryServiceClient<tonic::transport::Channel>,
        topic: TopicServiceClient<tonic::transport::Channel>,
        tera: Tera,
    ) -> Self {
        Self { cate, topic, tera }
    }
}
