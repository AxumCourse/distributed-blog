use std::sync::Arc;

use blog_proto::{
    topic_service_server::TopicService, CreateTopicReply, CreateTopicRequest, EditTopicReply,
    EditTopicRequest, GetTopicReply, GetTopicRequest, ListTopicReply, ListTopicRequest,
    ToggleTopicReply, ToggleTopicRequest,
};
use sqlx::PgPool;

pub struct Topic {
    pool: Arc<PgPool>,
}

impl Topic {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
}

#[tonic::async_trait]
impl TopicService for Topic {
    async fn create_topic(
        &self,
        request: tonic::Request<CreateTopicRequest>,
    ) -> Result<tonic::Response<CreateTopicReply>, tonic::Status> {
        unimplemented!()
    }
    async fn edit_topic(
        &self,
        request: tonic::Request<EditTopicRequest>,
    ) -> Result<tonic::Response<EditTopicReply>, tonic::Status> {
        unimplemented!()
    }
    async fn toggle_topic(
        &self,
        request: tonic::Request<ToggleTopicRequest>,
    ) -> Result<tonic::Response<ToggleTopicReply>, tonic::Status> {
        unimplemented!()
    }
    async fn get_topic(
        &self,
        request: tonic::Request<GetTopicRequest>,
    ) -> Result<tonic::Response<GetTopicReply>, tonic::Status> {
        unimplemented!()
    }
    async fn list_topic(
        &self,
        request: tonic::Request<ListTopicRequest>,
    ) -> Result<tonic::Response<ListTopicReply>, tonic::Status> {
        unimplemented!()
    }
}
