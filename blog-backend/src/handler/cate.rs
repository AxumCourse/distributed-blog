use std::sync::Arc;

use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::Html,
    Extension, Form,
};
use blog_proto::ListCategoryReply;
use tera::Context;

use crate::{form, model::AppState};

use super::redirect;

pub async fn list_cate(
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<form::CateListFilter>,
) -> Result<Html<String>, String> {
    let mut ctx = Context::new();
    let msg = params.msg.clone();
    if let Some(msg) = msg {
        ctx.insert("msg", &msg);
    }
    let mut cate = state.cate.clone();
    let resp = cate.list_category(tonic::Request::new(params.into())).await;
    let reply = match resp {
        Ok(r) => r.into_inner(),
        Err(err) => {
            if err.code() == tonic::Code::NotFound {
                ListCategoryReply { categories: vec![] }
            } else {
                return Err(err.to_string());
            }
        }
    };

    let mut cate_list = Vec::with_capacity(reply.categories.len());
    for c in reply.categories {
        let tc: blog_types::Category = c.into();
        cate_list.push(tc);
    }
    ctx.insert("cate_list", &cate_list);

    let out = state
        .tera
        .render("cate/index.html", &ctx)
        .map_err(|err| err.to_string())?;
    Ok(Html(out))
}
pub async fn add_cate_ui(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Html<String>, String> {
    let ctx = Context::new();
    let out = state
        .tera
        .render("cate/add.html", &ctx)
        .map_err(|err| err.to_string())?;
    Ok(Html(out))
}
pub async fn add_cate(
    Extension(state): Extension<Arc<AppState>>,
    Form(frm): Form<form::AddCatetory>,
) -> Result<(StatusCode, HeaderMap), String> {
    let mut cate = state.cate.clone();
    let resp = cate
        .create_category(tonic::Request::new(blog_proto::CreateCategoryRequest {
            name: frm.name,
        }))
        .await
        .map_err(|err| err.to_string())?;
    let repl = resp.into_inner();
    let url = format!("/m/cate?msg=分类(ID为{})添加成功", repl.id);
    Ok(redirect(&url))
}
