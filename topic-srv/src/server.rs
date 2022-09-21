use std::sync::Arc;

use blog_proto::{
    topic_service_server::TopicService, CreateTopicReply, CreateTopicRequest, EditTopicReply,
    EditTopicRequest, GetTopicReply, GetTopicRequest, ListTopicReply, ListTopicRequest,
    ToggleTopicReply, ToggleTopicRequest,
};
use sqlx::{Executor, PgPool, Row};

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
        let CreateTopicRequest {
            title,
            category_id,
            content,
            summary,
        } = request.into_inner();

        let summary = match summary {
            Some(summary) => summary,
            None => get_summary(&content),
        };
        let row = sqlx::query("INSERT INTO topics (title,category_id,content,summary) VALUES($1,$2,$3,$4) RETURNING id")
        .bind(title)
        .bind(category_id)
        .bind(content)
        .bind(summary)
        .fetch_one(&*self.pool)
        .await.map_err(|err|tonic::Status::internal(err.to_string()))?;
        let reply = CreateTopicReply { id: row.get("id") };
        Ok(tonic::Response::new(reply))
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
        let ToggleTopicRequest { id } = request.into_inner();
        let row = sqlx::query("UPDATE topics SET is_del=(NOT is_del) WHERE id=$1 RETURNING is_del")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;
        if row.is_none() {
            return Err(tonic::Status::not_found("不存在的文章"));
        }
        Ok(tonic::Response::new(ToggleTopicReply {
            id,
            is_del: row.unwrap().get("is_del"),
        }))
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

fn get_summary(content: &str) -> String {
    if content.len() <= 255 {
        return String::from(content);
    }
    content.chars().into_iter().take(255).collect()
}
