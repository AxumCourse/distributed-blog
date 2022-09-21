use std::sync::Arc;

use blog_proto::{
    category_exists_request::Condition, category_service_server::CategoryService,
    CategoryExistsReply, CategoryExistsRequest, CreateCategoryReply, CreateCategoryRequest,
    EditCategoryReply, EditCategoryRequest, GetCategoryReply, GetCategoryRequest,
    ListCategoryReply, ListCategoryRequest, ToggleCategoryReply, ToggleCategoryRequest,
};
use prost::bytes::Buf;
use sqlx::{Executor, PgPool, Row};

pub struct Category {
    pool: Arc<PgPool>,
}

impl Category {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
}
#[tonic::async_trait]
impl CategoryService for Category {
    async fn create_category(
        &self,
        request: tonic::Request<CreateCategoryRequest>,
    ) -> Result<tonic::Response<CreateCategoryReply>, tonic::Status> {
        let CreateCategoryRequest { name } = request.into_inner();
        let exists_request = tonic::Request::new(CategoryExistsRequest {
            condition: Some(Condition::Name(name.clone().into())),
        });
        let exists_reply = self.category_exists(exists_request).await?.into_inner();
        if exists_reply.exists {
            return Err(tonic::Status::already_exists("分类已存在"));
        }
        let res = sqlx::query("INSERT INTO categories (name) VALUES ($1) RETURNING id")
            .bind(name)
            .fetch_one(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;
        let reply = CreateCategoryReply { id: res.get("id") };
        Ok(tonic::Response::new(reply))
    }
    async fn category_exists(
        &self,
        request: tonic::Request<CategoryExistsRequest>,
    ) -> Result<tonic::Response<CategoryExistsReply>, tonic::Status> {
        let request = request.into_inner();
        let condition = request
            .condition
            .ok_or(tonic::Status::invalid_argument("参数错误"))?;
        let query = match condition {
            Condition::Name(name) => {
                sqlx::query("SELECT COUNT(*) FROM categories WHERE name=$1").bind(name)
            }
            Condition::Id(id) => {
                sqlx::query("SELECT COUNT(*) FROM categories WHERE id=$1").bind(id)
            }
        };
        let row = query
            .fetch_one(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;
        let count: i64 = row.get(0);
        let reply = CategoryExistsReply { exists: count > 0 };
        Ok(tonic::Response::new(reply))
    }
    async fn edit_category(
        &self,
        request: tonic::Request<EditCategoryRequest>,
    ) -> Result<tonic::Response<EditCategoryReply>, tonic::Status> {
        unimplemented!()
    }
    async fn get_category(
        &self,
        request: tonic::Request<GetCategoryRequest>,
    ) -> Result<tonic::Response<GetCategoryReply>, tonic::Status> {
        unimplemented!()
    }
    async fn list_category(
        &self,
        request: tonic::Request<ListCategoryRequest>,
    ) -> Result<tonic::Response<ListCategoryReply>, tonic::Status> {
        unimplemented!()
    }
    async fn toggle_category(
        &self,
        request: tonic::Request<ToggleCategoryRequest>,
    ) -> Result<tonic::Response<ToggleCategoryReply>, tonic::Status> {
        unimplemented!()
    }
}
