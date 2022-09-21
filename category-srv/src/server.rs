use blog_proto::{
    category_service_server::CategoryService, CategoryExistsReply, CategoryExistsRequest,
    CreateCategoryReply, CreateCategoryRequest, EditCategoryReply, EditCategoryRequest,
    GetCategoryReply, GetCategoryRequest, ListCategoryReply, ListCategoryRequest,
    ToggleCategoryReply, ToggleCategoryRequest,
};

pub struct Category {}

impl Category {
    pub fn new() -> Self {
        Self {}
    }
}
#[tonic::async_trait]
impl CategoryService for Category {
    async fn create_category(
        &self,
        request: tonic::Request<CreateCategoryRequest>,
    ) -> Result<tonic::Response<CreateCategoryReply>, tonic::Status> {
        unimplemented!()
    }
    async fn category_exists(
        &self,
        request: tonic::Request<CategoryExistsRequest>,
    ) -> Result<tonic::Response<CategoryExistsReply>, tonic::Status> {
        unimplemented!()
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
