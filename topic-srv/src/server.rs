use std::sync::Arc;

use blog_proto::{
    topic_service_server::TopicService, CreateTopicReply, CreateTopicRequest, EditTopicReply,
    EditTopicRequest, GetTopicReply, GetTopicRequest, ListTopicReply, ListTopicRequest,
    ToggleTopicReply, ToggleTopicRequest,
};
use chrono::{DateTime, Datelike, Local, Timelike};
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
        let r = request.into_inner();
        let summary = match r.summary {
            Some(s) => s,
            None => get_summary(&r.content),
        };
        let rows_affected = sqlx::query(
            "UPDATE topics SET title=$1,content=$2,summary=$3,category_id=$4 WHERE id=$5",
        )
        .bind(r.title)
        .bind(r.content)
        .bind(summary)
        .bind(r.category_id)
        .bind(r.id)
        .execute(&*self.pool)
        .await
        .map_err(|err| tonic::Status::internal(err.to_string()))?
        .rows_affected();
        Ok(tonic::Response::new(EditTopicReply {
            id: r.id,
            ok: rows_affected > 0,
        }))
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
        let GetTopicRequest {
            id,
            is_del,
            inc_hit,
        } = request.into_inner();

        let inc_hit = inc_hit.unwrap_or(false); // 增加点击量
        if inc_hit {
            sqlx::query("UPDATE topics SET hit=hit+1 WHERE id=$1")
                .bind(id)
                .execute(&*self.pool)
                .await
                .map_err(|err| tonic::Status::internal(err.to_string()))?;
        }

        let query = match is_del {
            Some(is_del) => sqlx::query("SELECT id,title,content,summary,is_del,category_id,dateline,hit FROM topics WHERE id=$1 AND is_del=$2")
            .bind(id).bind(is_del),
            None => sqlx::query("SELECT id,title,content,summary,is_del,category_id,dateline,hit FROM topics WHERE id=$1")
            .bind(id),
        };
        let row = query
            .fetch_optional(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;
        if row.is_none() {
            return Err(tonic::Status::not_found("不存在的文章"));
        }
        let row = row.unwrap();
        let dt: DateTime<Local> = row.get("dateline");
        let dateline = dt_conver(&dt);

        Ok(tonic::Response::new(GetTopicReply {
            topic: Some(blog_proto::Topic {
                id: row.get("id"),
                title: row.get("title"),
                category_id: row.get("category_id"),
                content: row.get("content"),
                summary: row.get("summary"),
                hit: row.get("hit"),
                is_del: row.get("is_del"),
                dateline,
            }),
        }))
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

fn dt_conver(dt: &DateTime<Local>) -> Option<prost_types::Timestamp> {
    if let Ok(dt) = prost_types::Timestamp::date_time(
        dt.year().into(),
        dt.month() as u8,
        dt.day() as u8,
        dt.hour() as u8,
        dt.minute() as u8,
        dt.second() as u8,
    ) {
        Some(dt)
    } else {
        None
    }
}
