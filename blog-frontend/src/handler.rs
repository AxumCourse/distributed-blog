use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    response::Html,
    Extension,
};
use blog_proto::{GetTopicRequest, ListCategoryRequest, ListTopicRequest};
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::model::AppState;

#[derive(Deserialize, Serialize)]
pub struct QueryParams {
    pub page: Option<i32>,
    pub category_id: Option<i32>,
    pub keyword: Option<String>,
}
#[derive(Deserialize, Serialize)]
pub struct QueryParamsForUrl {
    pub category_id: i32,
    pub keyword: String,
    pub page: i32,
}

impl From<QueryParams> for QueryParamsForUrl {
    fn from(p: QueryParams) -> Self {
        Self {
            category_id: match p.category_id {
                Some(cid) => cid,
                None => 0,
            },
            keyword: match p.keyword {
                Some(kw) => kw,
                None => "".to_string(),
            },
            page: p.page.unwrap_or(0),
        }
    }
}

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<QueryParams>,
) -> Result<Html<String>, String> {
    let mut ctx = Context::new();
    // 获取分类列表
    let mut cate = state.cate.clone();
    let resp = cate
        .list_category(tonic::Request::new(ListCategoryRequest {
            name: None,
            is_del: Some(false),
        }))
        .await
        .map_err(|err| err.to_string())?;
    let reply = resp.into_inner();
    let mut cate_list: Vec<blog_types::Category> = Vec::with_capacity(reply.categories.len());
    for reply_cate in reply.categories {
        cate_list.push(reply_cate.into());
    }
    ctx.insert("cate_list", &cate_list);

    // 文章列表
    let query_category_id = match params.category_id.clone() {
        Some(cid) => {
            if cid > 0 {
                Some(cid)
            } else {
                None
            }
        }
        None => None,
    };
    let mut tpc = state.topic.clone();
    let resp = tpc
        .list_topic(tonic::Request::new(ListTopicRequest {
            page: params.page.clone(),
            category_id: query_category_id,
            keyword: params.keyword.clone(),
            is_del: Some(false),
            dateline_range: None,
        }))
        .await
        .map_err(|err| err.to_string())?;
    let reply = resp.into_inner();
    let mut topic_list: Vec<blog_types::Topic> = Vec::with_capacity(reply.topics.capacity());
    for reply_topic in reply.topics {
        let mut t: blog_types::Topic = reply_topic.into();
        // 查找分类
        for cate in &cate_list {
            if cate.id == t.category_id {
                t.category_name = cate.name.clone();
                break;
            }
        }
        topic_list.push(t);
    }
    let paginate = blog_types::Paginate {
        page: reply.page,
        page_size: reply.page_size,
        page_totoal: reply.page_totoal,
        record_total: reply.record_total,
        data: topic_list,
    };
    ctx.insert("paginate", &paginate);

    let mut page_nums = vec![];
    for i in 0..paginate.page_totoal {
        page_nums.push(i as i32);
    }
    ctx.insert("page_nums", &page_nums);

    let params: QueryParamsForUrl = params.into();
    ctx.insert("params", &params);

    let out = state
        .tera
        .render("index.html", &ctx)
        .map_err(|err| err.to_string())?;
    Ok(Html(out))
}
pub async fn detail(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Html<String>, String> {
    let mut ctx = Context::new();
    // 获取分类列表
    let mut cate = state.cate.clone();
    let resp = cate
        .list_category(tonic::Request::new(ListCategoryRequest {
            name: None,
            is_del: Some(false),
        }))
        .await
        .map_err(|err| err.to_string())?;
    let reply = resp.into_inner();
    let mut cate_list: Vec<blog_types::Category> = Vec::with_capacity(reply.categories.len());
    for reply_cate in reply.categories {
        cate_list.push(reply_cate.into());
    }
    ctx.insert("cate_list", &cate_list);

    // 获取文章详情
    let mut tpc = state.topic.clone();
    let resp = tpc
        .get_topic(tonic::Request::new(GetTopicRequest {
            id,
            inc_hit: Some(true),
            is_del: Some(false),
        }))
        .await
        .map_err(|err| err.to_string())?;
    let reply = resp.into_inner();

    let mut t: blog_types::Topic = match reply.topic {
        Some(topic) => topic.into(),
        None => {
            return Err("不存在的文章".to_string());
        }
    };
    // 查找分类
    for cate in &cate_list {
        if cate.id == t.category_id {
            t.category_name = cate.name.clone();
            break;
        }
    }
    ctx.insert("topic", &t);
    let out = state
        .tera
        .render("detail.html", &ctx)
        .map_err(|err| err.to_string())?;
    Ok(Html(out))
}
