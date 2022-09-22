use std::sync::Arc;

use blog_proto::{
    admin_service_server::AdminService, AdminExistsReply, AdminExistsRequest, CreateAdminReply,
    CreateAdminRequest, EditAdminReply, EditAdminRequest, GetAdminReply, GetAdminRequest,
    ListAdminReply, ListAdminRequest, ToggleAdminReply, ToggleAdminRequest,
};
use blog_utils::password;
use sqlx::{Executor, PgPool, Row};

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
        let AdminExistsRequest { condition } = request.into_inner();
        let condition = match condition {
            Some(condition) => condition,
            None => return Err(tonic::Status::invalid_argument("请指定条件")),
        };
        let row = match condition {
            blog_proto::admin_exists_request::Condition::Email(email) => {
                sqlx::query("SELECT COUNT(*) FROM admins WHERE email=$1").bind(email)
            }
            blog_proto::admin_exists_request::Condition::Id(id) => {
                sqlx::query("SELECT COUNT(*) FROM admins WHERE id=$1").bind(id)
            }
        }
        .fetch_one(&*self.pool)
        .await
        .map_err(|err| tonic::Status::internal(err.to_string()))?;
        let count: i32 = row.get(0);
        Ok(tonic::Response::new(AdminExistsReply { exists: count > 0 }))
    }
    async fn get_admin(
        &self,
        request: tonic::Request<GetAdminRequest>,
    ) -> Result<tonic::Response<GetAdminReply>, tonic::Status> {
        let GetAdminRequest { condition } = request.into_inner();
        let condition = match condition {
            Some(condition) => condition,
            None => return Err(tonic::Status::invalid_argument("请指定条件")),
        };
        let reply = match condition {
            blog_proto::get_admin_request::Condition::ByAuth(ba) => {
                let row = sqlx::query("SELECT id,email,is_del,password FROM admins WHERE email=$1")
                    .bind(ba.email)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| tonic::Status::internal(err.to_string()))?;
                if let Some(row) = row {
                    let hashed_pwd: String = row.get("password");
                    let is_verify = password::verify(&ba.password, &hashed_pwd)
                        .map_err(tonic::Status::internal)?;
                    if !is_verify {
                        return Err(tonic::Status::invalid_argument("用户名/密码错误2"));
                    } else {
                        GetAdminReply {
                            admin: Some(blog_proto::Admin {
                                id: row.get("id"),
                                email: row.get("email"),
                                password: None,
                                is_del: row.get("is_del"),
                            }),
                        }
                    }
                } else {
                    return Err(tonic::Status::invalid_argument("用户名/密码错误"));
                }
            }
            blog_proto::get_admin_request::Condition::ById(bi) => {
                let row = match bi.is_del {
                    Some(is_del) => {
                        sqlx::query("SELECT id,email,is_del FROM admins WHERE id=$1 AND is_del=$2")
                            .bind(bi.id)
                            .bind(is_del)
                    }
                    None => {
                        sqlx::query("SELECT id,email,is_del FROM admins WHERE id=$1").bind(bi.id)
                    }
                }
                .fetch_optional(&*self.pool)
                .await
                .map_err(|err| tonic::Status::internal(err.to_string()))?;
                if let Some(row) = row {
                    GetAdminReply {
                        admin: Some(blog_proto::Admin {
                            id: row.get("id"),
                            email: row.get("email"),
                            password: None,
                            is_del: row.get("is_del"),
                        }),
                    }
                } else {
                    return Err(tonic::Status::not_found("不存在的用户"));
                }
            }
        };
        Ok(tonic::Response::new(reply))
    }
    async fn edit_admin(
        &self,
        request: tonic::Request<EditAdminRequest>,
    ) -> Result<tonic::Response<EditAdminReply>, tonic::Status> {
        let EditAdminRequest {
            id,
            email,
            password,
            new_password,
        } = request.into_inner();
        let new_password = match new_password {
            Some(n) => n,
            None => return Err(tonic::Status::invalid_argument("请设定新密码")),
        };
        let row = sqlx::query("SELECT password FROM admins WHERE id=$1 AND email=$2")
            .bind(id)
            .bind(&email)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;
        let pwd_in_db: String = match row {
            Some(row) => row.get("password"),
            None => return Err(tonic::Status::not_found("不存在的用户")),
        };
        let is_verify = password::verify(&password, &pwd_in_db).map_err(tonic::Status::internal)?;
        if !is_verify {
            return Err(tonic::Status::invalid_argument("密码错误"));
        }
        let hashed_new_pwd = password::hash(&new_password).map_err(tonic::Status::internal)?;
        let rows_affected = sqlx::query("UPDATE password=$1 WHERE id=$2 AND email=$3")
            .bind(hashed_new_pwd)
            .bind(id)
            .bind(&email)
            .execute(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?
            .rows_affected();
        Ok(tonic::Response::new(EditAdminReply {
            id: id,
            ok: rows_affected > 0,
        }))
    }
    async fn list_admin(
        &self,
        request: tonic::Request<ListAdminRequest>,
    ) -> Result<tonic::Response<ListAdminReply>, tonic::Status> {
        let ListAdminRequest { email, is_del } = request.into_inner();
        let rows = sqlx::query(
            r#"
            SELECT
                id,email,is_del 
            FROM
                admins
            WHERE 1=1
                AND ($1::text IS NULL OR email=$1::text)
                AND ($2::boolean IS NULL OR is_del=$2::bollean)
        "#,
        )
        .bind(email)
        .bind(is_del)
        .fetch_all(&*self.pool)
        .await
        .map_err(|err| tonic::Status::internal(err.to_string()))?;
        let mut admins = Vec::with_capacity(rows.len());
        for row in rows {
            let a = blog_proto::Admin {
                id: row.get("id"),
                email: row.get("email"),
                is_del: row.get("is_del"),
                password: None,
            };
            admins.push(a);
        }
        Ok(tonic::Response::new(ListAdminReply { admins }))
    }
    async fn create_admin(
        &self,
        request: tonic::Request<CreateAdminRequest>,
    ) -> Result<tonic::Response<CreateAdminReply>, tonic::Status> {
        let request = request.into_inner();
        let AdminExistsReply { exists } = self
            .admin_exists(tonic::Request::new(AdminExistsRequest {
                condition: Some(blog_proto::admin_exists_request::Condition::Email(
                    request.email.clone(),
                )),
            }))
            .await?
            .into_inner();
        if exists {
            return Err(tonic::Status::already_exists("管理员已存在"));
        }
        let pwd = password::hash(&request.password).map_err(tonic::Status::internal)?;
        let row = sqlx::query("INSERT INTO admins (email,password) VALUES ($1,$2) RETURNING id")
            .bind(request.email)
            .bind(pwd)
            .fetch_one(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;
        Ok(tonic::Response::new(CreateAdminReply { id: row.get(0) }))
    }
    async fn toggle_admin(
        &self,
        request: tonic::Request<ToggleAdminRequest>,
    ) -> Result<tonic::Response<ToggleAdminReply>, tonic::Status> {
        let ToggleAdminRequest { id } = request.into_inner();
        let row = sqlx::query("UPDATE admins SET is_del=(NOT is_del) WHERE id=$1 RETURNING is_del")
            .bind(id)
            .fetch_one(&*self.pool)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;
        Ok(tonic::Response::new(ToggleAdminReply {
            id: id,
            is_del: row.get(0),
        }))
    }
}
