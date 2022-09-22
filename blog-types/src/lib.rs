use chrono::{Datelike, Local, TimeZone, Timelike};
use serde::Serialize;

#[derive(Serialize, Default)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub is_del: bool,
}

impl From<blog_proto::Category> for Category {
    fn from(c: blog_proto::Category) -> Self {
        Self {
            id: c.id,
            name: c.name,
            is_del: c.is_del,
        }
    }
}
#[derive(Serialize, Default)]
pub struct Dateline {
    pub timestamp: i64,
}
impl Dateline {
    pub fn format(&self, layout: &str) -> String {
        let dt: chrono::DateTime<Local> = self.into();
        dt.format(layout).to_string()
    }
    pub fn date_str(&self) -> String {
        self.format("%Y-%m-%d")
    }
    pub fn time_str(&self) -> String {
        self.format("%H:%M:%S")
    }
    pub fn datetime_str(&self) -> String {
        self.format("%Y-%m-%d %H:%M:%S")
    }
}
impl ToString for Dateline {
    fn to_string(&self) -> String {
        self.datetime_str()
    }
}

impl From<chrono::DateTime<Local>> for Dateline {
    fn from(dt: chrono::DateTime<Local>) -> Self {
        Self {
            timestamp: dt.timestamp(),
        }
    }
}
impl From<prost_types::Timestamp> for Dateline {
    fn from(ts: prost_types::Timestamp) -> Self {
        Self {
            timestamp: ts.seconds,
        }
    }
}
impl Into<chrono::DateTime<Local>> for Dateline {
    fn into(self) -> chrono::DateTime<Local> {
        Local.timestamp(self.timestamp, 0)
    }
}
impl Into<chrono::DateTime<Local>> for &Dateline {
    fn into(self) -> chrono::DateTime<Local> {
        Local.timestamp(self.timestamp, 0)
    }
}

impl Into<prost_types::Timestamp> for Dateline {
    fn into(self) -> prost_types::Timestamp {
        let dt: chrono::DateTime<Local> = self.into();
        prost_types::Timestamp::date_time(
            dt.year().into(),
            dt.month() as u8,
            dt.day() as u8,
            dt.hour() as u8,
            dt.minute() as u8,
            dt.second() as u8,
        )
        .unwrap()
    }
}
#[derive(Serialize, Default)]
pub struct Topic {
    pub id: i64,
    pub title: String,
    pub category_id: i32,
    pub summary: String,
    pub content: String,
    pub hit: i32,
    pub is_del: bool,
    pub dateline: Dateline,
    pub category_name: String,
}

impl From<blog_proto::Topic> for Topic {
    fn from(t: blog_proto::Topic) -> Self {
        let dl = match t.dateline {
            Some(dl) => dl.into(),
            None => Default::default(),
        };
        Self {
            id: t.id,
            title: t.title,
            category_id: t.category_id,
            summary: t.summary,
            content: t.content,
            hit: t.hit,
            is_del: t.is_del,
            dateline: dl,
            ..Default::default()
        }
    }
}
#[derive(Serialize)]
pub struct Paginate<T: Serialize> {
    pub page: i32,
    /// 每页条数
    pub page_size: i32,
    /// 总页数
    pub page_totoal: i64,
    /// 总记录数
    pub record_total: i64,
    pub data: Vec<T>,
}
