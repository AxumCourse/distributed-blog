#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Category {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub is_del: bool,
}
/// --- 创建分类
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCategoryRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCategoryReply {
    #[prost(int32, tag="1")]
    pub id: i32,
}
/// -- 修改分类
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditCategoryRequest {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditCategoryReply {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(bool, tag="2")]
    pub ok: bool,
}
/// -- 分类列表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCategoryRequest {
    /// 根据分类名称查找
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// 是否删除
    #[prost(bool, optional, tag="2")]
    pub is_del: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCategoryReply {
    #[prost(message, repeated, tag="1")]
    pub categories: ::prost::alloc::vec::Vec<Category>,
}
/// -- 删除/恢复分类
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToggleCategoryRequest {
    #[prost(int32, tag="1")]
    pub id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToggleCategoryReply {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(bool, tag="2")]
    pub is_del: bool,
}
/// -- 分类是否存在
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CategoryExistsRequest {
    #[prost(oneof="category_exists_request::Condition", tags="1, 2")]
    pub condition: ::core::option::Option<category_exists_request::Condition>,
}
/// Nested message and enum types in `CategoryExistsRequest`.
pub mod category_exists_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Condition {
        #[prost(string, tag="1")]
        Name(::prost::alloc::string::String),
        #[prost(int32, tag="2")]
        Id(i32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CategoryExistsReply {
    #[prost(bool, tag="1")]
    pub exists: bool,
}
/// -- 分类详情
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCategoryRequest {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(bool, optional, tag="2")]
    pub is_del: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCategoryReply {
    #[prost(message, optional, tag="1")]
    pub category: ::core::option::Option<Category>,
}
/// Generated client implementations.
pub mod category_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct CategoryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CategoryServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CategoryServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CategoryServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            CategoryServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// 创建分类
        pub async fn create_category(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCategoryRequest>,
        ) -> Result<tonic::Response<super::CreateCategoryReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.CategoryService/CreateCategory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 修改分类
        pub async fn edit_category(
            &mut self,
            request: impl tonic::IntoRequest<super::EditCategoryRequest>,
        ) -> Result<tonic::Response<super::EditCategoryReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.CategoryService/EditCategory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 分类列表
        pub async fn list_category(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCategoryRequest>,
        ) -> Result<tonic::Response<super::ListCategoryReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.CategoryService/ListCategory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 删除/恢复分类
        pub async fn toggle_category(
            &mut self,
            request: impl tonic::IntoRequest<super::ToggleCategoryRequest>,
        ) -> Result<tonic::Response<super::ToggleCategoryReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.CategoryService/ToggleCategory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 分类是否存在
        pub async fn category_exists(
            &mut self,
            request: impl tonic::IntoRequest<super::CategoryExistsRequest>,
        ) -> Result<tonic::Response<super::CategoryExistsReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.CategoryService/CategoryExists",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 获取分类详情
        pub async fn get_category(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCategoryRequest>,
        ) -> Result<tonic::Response<super::GetCategoryReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.CategoryService/GetCategory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod category_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with CategoryServiceServer.
    #[async_trait]
    pub trait CategoryService: Send + Sync + 'static {
        /// 创建分类
        async fn create_category(
            &self,
            request: tonic::Request<super::CreateCategoryRequest>,
        ) -> Result<tonic::Response<super::CreateCategoryReply>, tonic::Status>;
        /// 修改分类
        async fn edit_category(
            &self,
            request: tonic::Request<super::EditCategoryRequest>,
        ) -> Result<tonic::Response<super::EditCategoryReply>, tonic::Status>;
        /// 分类列表
        async fn list_category(
            &self,
            request: tonic::Request<super::ListCategoryRequest>,
        ) -> Result<tonic::Response<super::ListCategoryReply>, tonic::Status>;
        /// 删除/恢复分类
        async fn toggle_category(
            &self,
            request: tonic::Request<super::ToggleCategoryRequest>,
        ) -> Result<tonic::Response<super::ToggleCategoryReply>, tonic::Status>;
        /// 分类是否存在
        async fn category_exists(
            &self,
            request: tonic::Request<super::CategoryExistsRequest>,
        ) -> Result<tonic::Response<super::CategoryExistsReply>, tonic::Status>;
        /// 获取分类详情
        async fn get_category(
            &self,
            request: tonic::Request<super::GetCategoryRequest>,
        ) -> Result<tonic::Response<super::GetCategoryReply>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct CategoryServiceServer<T: CategoryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CategoryService> CategoryServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CategoryServiceServer<T>
    where
        T: CategoryService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/pb.CategoryService/CreateCategory" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCategorySvc<T: CategoryService>(pub Arc<T>);
                    impl<
                        T: CategoryService,
                    > tonic::server::UnaryService<super::CreateCategoryRequest>
                    for CreateCategorySvc<T> {
                        type Response = super::CreateCategoryReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCategoryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_category(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateCategorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.CategoryService/EditCategory" => {
                    #[allow(non_camel_case_types)]
                    struct EditCategorySvc<T: CategoryService>(pub Arc<T>);
                    impl<
                        T: CategoryService,
                    > tonic::server::UnaryService<super::EditCategoryRequest>
                    for EditCategorySvc<T> {
                        type Response = super::EditCategoryReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EditCategoryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).edit_category(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditCategorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.CategoryService/ListCategory" => {
                    #[allow(non_camel_case_types)]
                    struct ListCategorySvc<T: CategoryService>(pub Arc<T>);
                    impl<
                        T: CategoryService,
                    > tonic::server::UnaryService<super::ListCategoryRequest>
                    for ListCategorySvc<T> {
                        type Response = super::ListCategoryReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCategoryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_category(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListCategorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.CategoryService/ToggleCategory" => {
                    #[allow(non_camel_case_types)]
                    struct ToggleCategorySvc<T: CategoryService>(pub Arc<T>);
                    impl<
                        T: CategoryService,
                    > tonic::server::UnaryService<super::ToggleCategoryRequest>
                    for ToggleCategorySvc<T> {
                        type Response = super::ToggleCategoryReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ToggleCategoryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).toggle_category(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ToggleCategorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.CategoryService/CategoryExists" => {
                    #[allow(non_camel_case_types)]
                    struct CategoryExistsSvc<T: CategoryService>(pub Arc<T>);
                    impl<
                        T: CategoryService,
                    > tonic::server::UnaryService<super::CategoryExistsRequest>
                    for CategoryExistsSvc<T> {
                        type Response = super::CategoryExistsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CategoryExistsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).category_exists(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CategoryExistsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.CategoryService/GetCategory" => {
                    #[allow(non_camel_case_types)]
                    struct GetCategorySvc<T: CategoryService>(pub Arc<T>);
                    impl<
                        T: CategoryService,
                    > tonic::server::UnaryService<super::GetCategoryRequest>
                    for GetCategorySvc<T> {
                        type Response = super::GetCategoryReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCategoryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_category(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCategorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: CategoryService> Clone for CategoryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: CategoryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CategoryService> tonic::server::NamedService for CategoryServiceServer<T> {
        const NAME: &'static str = "pb.CategoryService";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Admin {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub password: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, tag="4")]
    pub is_del: bool,
}
/// -- 添加管理员
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAdminRequest {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAdminReply {
    #[prost(int32, tag="1")]
    pub id: i32,
}
/// -- 管理员列表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAdminRequest {
    /// 根据EMAIL进行查找
    #[prost(string, optional, tag="1")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
    /// 是否删除
    #[prost(bool, optional, tag="2")]
    pub is_del: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAdminReply {
    #[prost(message, repeated, tag="1")]
    pub admins: ::prost::alloc::vec::Vec<Admin>,
}
/// -- 修改管理员
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditAdminRequest {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
    /// 现用密码
    #[prost(string, tag="3")]
    pub password: ::prost::alloc::string::String,
    /// 如果有密码，则修改密码
    #[prost(string, optional, tag="4")]
    pub new_password: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditAdminReply {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(bool, tag="2")]
    pub ok: bool,
}
/// -- 删除/恢复管理员
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToggleAdminRequest {
    #[prost(int32, tag="1")]
    pub id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToggleAdminReply {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(bool, tag="2")]
    pub is_del: bool,
}
/// -- 管理员是否存在
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminExistsRequest {
    #[prost(oneof="admin_exists_request::Condition", tags="1, 2")]
    pub condition: ::core::option::Option<admin_exists_request::Condition>,
}
/// Nested message and enum types in `AdminExistsRequest`.
pub mod admin_exists_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Condition {
        #[prost(string, tag="1")]
        Email(::prost::alloc::string::String),
        #[prost(int32, tag="2")]
        Id(i32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminExistsReply {
    #[prost(bool, tag="1")]
    pub exists: bool,
}
/// -- 获取管理员
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdminRequest {
    #[prost(oneof="get_admin_request::Condition", tags="1, 2")]
    pub condition: ::core::option::Option<get_admin_request::Condition>,
}
/// Nested message and enum types in `GetAdminRequest`.
pub mod get_admin_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ByAuth {
        #[prost(string, tag="1")]
        pub email: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub password: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ById {
        #[prost(int32, tag="1")]
        pub id: i32,
        #[prost(bool, optional, tag="2")]
        pub is_del: ::core::option::Option<bool>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Condition {
        /// 通过ID直接获取
        #[prost(message, tag="1")]
        ById(ById),
        /// 通过登录信息获取
        #[prost(message, tag="2")]
        ByAuth(ByAuth),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdminReply {
    #[prost(message, optional, tag="1")]
    pub admin: ::core::option::Option<Admin>,
}
/// Generated client implementations.
pub mod admin_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AdminServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AdminServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AdminServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AdminServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AdminServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// 添加管理员
        pub async fn create_admin(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAdminRequest>,
        ) -> Result<tonic::Response<super::CreateAdminReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.AdminService/CreateAdmin",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 管理员列表
        pub async fn list_admin(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAdminRequest>,
        ) -> Result<tonic::Response<super::ListAdminReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.AdminService/ListAdmin",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 修改管理员
        pub async fn edit_admin(
            &mut self,
            request: impl tonic::IntoRequest<super::EditAdminRequest>,
        ) -> Result<tonic::Response<super::EditAdminReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.AdminService/EditAdmin",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 删除/恢复管理员
        pub async fn toggle_admin(
            &mut self,
            request: impl tonic::IntoRequest<super::ToggleAdminRequest>,
        ) -> Result<tonic::Response<super::ToggleAdminReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.AdminService/ToggleAdmin",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 管理员是否存在
        pub async fn admin_exists(
            &mut self,
            request: impl tonic::IntoRequest<super::AdminExistsRequest>,
        ) -> Result<tonic::Response<super::AdminExistsReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.AdminService/AdminExists",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 获取管理员
        pub async fn get_admin(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdminRequest>,
        ) -> Result<tonic::Response<super::GetAdminReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pb.AdminService/GetAdmin");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod admin_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with AdminServiceServer.
    #[async_trait]
    pub trait AdminService: Send + Sync + 'static {
        /// 添加管理员
        async fn create_admin(
            &self,
            request: tonic::Request<super::CreateAdminRequest>,
        ) -> Result<tonic::Response<super::CreateAdminReply>, tonic::Status>;
        /// 管理员列表
        async fn list_admin(
            &self,
            request: tonic::Request<super::ListAdminRequest>,
        ) -> Result<tonic::Response<super::ListAdminReply>, tonic::Status>;
        /// 修改管理员
        async fn edit_admin(
            &self,
            request: tonic::Request<super::EditAdminRequest>,
        ) -> Result<tonic::Response<super::EditAdminReply>, tonic::Status>;
        /// 删除/恢复管理员
        async fn toggle_admin(
            &self,
            request: tonic::Request<super::ToggleAdminRequest>,
        ) -> Result<tonic::Response<super::ToggleAdminReply>, tonic::Status>;
        /// 管理员是否存在
        async fn admin_exists(
            &self,
            request: tonic::Request<super::AdminExistsRequest>,
        ) -> Result<tonic::Response<super::AdminExistsReply>, tonic::Status>;
        /// 获取管理员
        async fn get_admin(
            &self,
            request: tonic::Request<super::GetAdminRequest>,
        ) -> Result<tonic::Response<super::GetAdminReply>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct AdminServiceServer<T: AdminService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AdminService> AdminServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AdminServiceServer<T>
    where
        T: AdminService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/pb.AdminService/CreateAdmin" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAdminSvc<T: AdminService>(pub Arc<T>);
                    impl<
                        T: AdminService,
                    > tonic::server::UnaryService<super::CreateAdminRequest>
                    for CreateAdminSvc<T> {
                        type Response = super::CreateAdminReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateAdminRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_admin(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateAdminSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.AdminService/ListAdmin" => {
                    #[allow(non_camel_case_types)]
                    struct ListAdminSvc<T: AdminService>(pub Arc<T>);
                    impl<
                        T: AdminService,
                    > tonic::server::UnaryService<super::ListAdminRequest>
                    for ListAdminSvc<T> {
                        type Response = super::ListAdminReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAdminRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_admin(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListAdminSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.AdminService/EditAdmin" => {
                    #[allow(non_camel_case_types)]
                    struct EditAdminSvc<T: AdminService>(pub Arc<T>);
                    impl<
                        T: AdminService,
                    > tonic::server::UnaryService<super::EditAdminRequest>
                    for EditAdminSvc<T> {
                        type Response = super::EditAdminReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EditAdminRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).edit_admin(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditAdminSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.AdminService/ToggleAdmin" => {
                    #[allow(non_camel_case_types)]
                    struct ToggleAdminSvc<T: AdminService>(pub Arc<T>);
                    impl<
                        T: AdminService,
                    > tonic::server::UnaryService<super::ToggleAdminRequest>
                    for ToggleAdminSvc<T> {
                        type Response = super::ToggleAdminReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ToggleAdminRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).toggle_admin(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ToggleAdminSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.AdminService/AdminExists" => {
                    #[allow(non_camel_case_types)]
                    struct AdminExistsSvc<T: AdminService>(pub Arc<T>);
                    impl<
                        T: AdminService,
                    > tonic::server::UnaryService<super::AdminExistsRequest>
                    for AdminExistsSvc<T> {
                        type Response = super::AdminExistsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AdminExistsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).admin_exists(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AdminExistsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.AdminService/GetAdmin" => {
                    #[allow(non_camel_case_types)]
                    struct GetAdminSvc<T: AdminService>(pub Arc<T>);
                    impl<
                        T: AdminService,
                    > tonic::server::UnaryService<super::GetAdminRequest>
                    for GetAdminSvc<T> {
                        type Response = super::GetAdminReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAdminRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_admin(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAdminSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: AdminService> Clone for AdminServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: AdminService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AdminService> tonic::server::NamedService for AdminServiceServer<T> {
        const NAME: &'static str = "pb.AdminService";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Topic {
    #[prost(int64, tag="1")]
    pub id: i64,
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub category_id: i32,
    #[prost(string, tag="4")]
    pub summary: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub content: ::prost::alloc::string::String,
    #[prost(int32, tag="6")]
    pub hit: i32,
    #[prost(bool, tag="7")]
    pub is_del: bool,
    #[prost(message, optional, tag="8")]
    pub dateline: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatelineRange {
    #[prost(message, optional, tag="1")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="2")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
}
/// -- 创建文章
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTopicRequest {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub category_id: i32,
    #[prost(string, tag="3")]
    pub content: ::prost::alloc::string::String,
    /// 如果没有提供摘要，则自动从内容中截取
    #[prost(string, optional, tag="4")]
    pub summary: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTopicReply {
    #[prost(int64, tag="1")]
    pub id: i64,
}
/// -- 修改文章
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditTopicRequest {
    #[prost(int64, tag="1")]
    pub id: i64,
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub category_id: i32,
    /// 如果没有提供摘要，则自动从内容中截取
    #[prost(string, optional, tag="4")]
    pub summary: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="5")]
    pub content: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditTopicReply {
    #[prost(int64, tag="1")]
    pub id: i64,
    #[prost(bool, tag="2")]
    pub ok: bool,
}
/// -- 文章列表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicRequest {
    /// 页码
    #[prost(int32, optional, tag="1")]
    pub page: ::core::option::Option<i32>,
    /// 分类
    #[prost(int32, optional, tag="2")]
    pub category_id: ::core::option::Option<i32>,
    /// 关键字
    #[prost(string, optional, tag="3")]
    pub keyword: ::core::option::Option<::prost::alloc::string::String>,
    /// 是否删除
    #[prost(bool, optional, tag="4")]
    pub is_del: ::core::option::Option<bool>,
    /// 时间区间
    #[prost(message, optional, tag="5")]
    pub dateline_range: ::core::option::Option<DatelineRange>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicReply {
    /// 当前页码
    #[prost(int32, tag="1")]
    pub page: i32,
    /// 每页条数
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// 文章列表
    #[prost(message, repeated, tag="3")]
    pub topics: ::prost::alloc::vec::Vec<Topic>,
}
/// -- 删除/恢复文章
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToggleTopicRequest {
    #[prost(int64, tag="1")]
    pub id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToggleTopicReply {
    #[prost(int64, tag="1")]
    pub id: i64,
    #[prost(bool, tag="2")]
    pub is_del: bool,
}
/// -- 获取文章详情
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTopicRequest {
    #[prost(int64, tag="1")]
    pub id: i64,
    #[prost(bool, optional, tag="2")]
    pub is_del: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTopicReply {
    #[prost(message, optional, tag="1")]
    pub topic: ::core::option::Option<Topic>,
}
/// Generated client implementations.
pub mod topic_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct TopicServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TopicServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TopicServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TopicServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            TopicServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// 创建文章
        pub async fn create_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTopicRequest>,
        ) -> Result<tonic::Response<super::CreateTopicReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.TopicService/CreateTopic",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 修改文章
        pub async fn edit_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::EditTopicRequest>,
        ) -> Result<tonic::Response<super::EditTopicReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.TopicService/EditTopic",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 文章列表
        pub async fn list_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTopicRequest>,
        ) -> Result<tonic::Response<super::ListTopicReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.TopicService/ListTopic",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 删除/恢复文章
        pub async fn toggle_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::ToggleTopicRequest>,
        ) -> Result<tonic::Response<super::ToggleTopicReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/pb.TopicService/ToggleTopic",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 获取文章详情
        pub async fn get_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTopicRequest>,
        ) -> Result<tonic::Response<super::GetTopicReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pb.TopicService/GetTopic");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod topic_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with TopicServiceServer.
    #[async_trait]
    pub trait TopicService: Send + Sync + 'static {
        /// 创建文章
        async fn create_topic(
            &self,
            request: tonic::Request<super::CreateTopicRequest>,
        ) -> Result<tonic::Response<super::CreateTopicReply>, tonic::Status>;
        /// 修改文章
        async fn edit_topic(
            &self,
            request: tonic::Request<super::EditTopicRequest>,
        ) -> Result<tonic::Response<super::EditTopicReply>, tonic::Status>;
        /// 文章列表
        async fn list_topic(
            &self,
            request: tonic::Request<super::ListTopicRequest>,
        ) -> Result<tonic::Response<super::ListTopicReply>, tonic::Status>;
        /// 删除/恢复文章
        async fn toggle_topic(
            &self,
            request: tonic::Request<super::ToggleTopicRequest>,
        ) -> Result<tonic::Response<super::ToggleTopicReply>, tonic::Status>;
        /// 获取文章详情
        async fn get_topic(
            &self,
            request: tonic::Request<super::GetTopicRequest>,
        ) -> Result<tonic::Response<super::GetTopicReply>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct TopicServiceServer<T: TopicService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TopicService> TopicServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TopicServiceServer<T>
    where
        T: TopicService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/pb.TopicService/CreateTopic" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTopicSvc<T: TopicService>(pub Arc<T>);
                    impl<
                        T: TopicService,
                    > tonic::server::UnaryService<super::CreateTopicRequest>
                    for CreateTopicSvc<T> {
                        type Response = super::CreateTopicReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTopicRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_topic(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateTopicSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.TopicService/EditTopic" => {
                    #[allow(non_camel_case_types)]
                    struct EditTopicSvc<T: TopicService>(pub Arc<T>);
                    impl<
                        T: TopicService,
                    > tonic::server::UnaryService<super::EditTopicRequest>
                    for EditTopicSvc<T> {
                        type Response = super::EditTopicReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EditTopicRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).edit_topic(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditTopicSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.TopicService/ListTopic" => {
                    #[allow(non_camel_case_types)]
                    struct ListTopicSvc<T: TopicService>(pub Arc<T>);
                    impl<
                        T: TopicService,
                    > tonic::server::UnaryService<super::ListTopicRequest>
                    for ListTopicSvc<T> {
                        type Response = super::ListTopicReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTopicRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_topic(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListTopicSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.TopicService/ToggleTopic" => {
                    #[allow(non_camel_case_types)]
                    struct ToggleTopicSvc<T: TopicService>(pub Arc<T>);
                    impl<
                        T: TopicService,
                    > tonic::server::UnaryService<super::ToggleTopicRequest>
                    for ToggleTopicSvc<T> {
                        type Response = super::ToggleTopicReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ToggleTopicRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).toggle_topic(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ToggleTopicSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pb.TopicService/GetTopic" => {
                    #[allow(non_camel_case_types)]
                    struct GetTopicSvc<T: TopicService>(pub Arc<T>);
                    impl<
                        T: TopicService,
                    > tonic::server::UnaryService<super::GetTopicRequest>
                    for GetTopicSvc<T> {
                        type Response = super::GetTopicReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTopicRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_topic(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTopicSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: TopicService> Clone for TopicServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: TopicService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TopicService> tonic::server::NamedService for TopicServiceServer<T> {
        const NAME: &'static str = "pb.TopicService";
    }
}
