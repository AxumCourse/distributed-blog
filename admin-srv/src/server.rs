use std::sync::Arc;

use blog_proto::{
    admin_service_server::AdminService, AdminExistsReply, AdminExistsRequest, CreateAdminReply,
    CreateAdminRequest, EditAdminReply, EditAdminRequest, GetAdminReply, GetAdminRequest,
    ListAdminReply, ListAdminRequest, ToggleAdminReply, ToggleAdminRequest,
};
use sqlx::PgPool;

pub struct Admin {
    pub pool: Arc<PgPool>,
}

impl Admin {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
}

#[tonic::async_trait]
impl AdminService for Admin {
    async fn admin_exists(
        &self,
        request: tonic::Request<AdminExistsRequest>,
    ) -> Result<tonic::Response<AdminExistsReply>, tonic::Status> {
        unimplemented!()
    }
    async fn get_admin(
        &self,
        request: tonic::Request<GetAdminRequest>,
    ) -> Result<tonic::Response<GetAdminReply>, tonic::Status> {
        unimplemented!()
    }
    async fn edit_admin(
        &self,
        request: tonic::Request<EditAdminRequest>,
    ) -> Result<tonic::Response<EditAdminReply>, tonic::Status> {
        unimplemented!()
    }
    async fn list_admin(
        &self,
        request: tonic::Request<ListAdminRequest>,
    ) -> Result<tonic::Response<ListAdminReply>, tonic::Status> {
        unimplemented!()
    }
    async fn create_admin(
        &self,
        request: tonic::Request<CreateAdminRequest>,
    ) -> Result<tonic::Response<CreateAdminReply>, tonic::Status> {
        unimplemented!()
    }
    async fn toggle_admin(
        &self,
        request: tonic::Request<ToggleAdminRequest>,
    ) -> Result<tonic::Response<ToggleAdminReply>, tonic::Status> {
        unimplemented!()
    }
}
