use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sqlx::FromRow;

use bbs_utils::{usizedb, ContentType};
//帖子状态
#[derive(Debug, Deserialize_repr, Serialize_repr, PartialEq, Clone, Copy, sqlx::Type)]
#[repr(u8)]
pub enum PostStatus {
    Active,
    Archived,
    Banned,
}
//帖子内部的信息
#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
pub struct PostInfo {
    pub id: usizedb,
    pub created_by_id: usizedb,
    pub title: String,
    pub content: Option<String>,
    pub content_type: ContentType,
    pub likes: usizedb,
    pub dislikes: usizedb,
    pub views: usizedb,
    pub category_id: usizedb,
    pub tags: Vec<String>,
    pub created_at: usizedb,
    pub last_edit_at: usizedb,
    pub last_edit_by_id: usizedb,
    pub last_comment_at: usizedb,
    pub last_comment_by_id: usizedb,
    pub total_comment: usizedb,
    pub total_comment_post: usizedb,
    pub status: PostStatus,
    pub cover_url: Option<String>,
    #[serde(default)]
    pub top_index: usizedb,
}
//基本信息，例如发帖人，♥️数等
#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
pub struct PostBaseInfo {
    pub id: usizedb,
    pub created_by_id: usizedb,
    pub content_type: ContentType,
    pub likes: usizedb,
    pub dislikes: usizedb,
    pub views: usizedb,
    pub category_id: usizedb,
    pub created_at: usizedb,
    pub last_edit_at: usizedb,
    pub last_edit_by_id: usizedb,
    pub last_comment_at: usizedb,
    pub last_comment_by_id: usizedb,
    pub total_comment: usizedb,
    pub total_comment_post: usizedb,
    pub status: PostStatus,
    pub cover_url: Option<String>,
    #[serde(default)]
    pub top_index: usizedb,
}
//创建帖子时需要的消息，标题，内容，类
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PostToCreate {
    pub title: String,
    pub content: String,
    pub content_type: ContentType,
    pub category_id: usizedb,
    pub tags: Vec<String>,
    pub cover_url: Option<String>,
    #[serde(default)]
    pub top_index: usizedb,
}
//更新帖子
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PostToUpdate {
    pub title: String,
    pub content: String,
    pub content_type: ContentType,
    pub tags: Vec<String>,
    pub cover_url: Option<String>,
    #[serde(default)]
    pub top_index: usizedb,
}
//根据热门; 观看数; 喜欢数; 最新来分组
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum PostAlgorithmOrder {
    Hot,
    Views,
    Likes,
    Newest,
}
//过滤的算法
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum PostFilterTime {
    Lifetime,
    Day(usizedb),
    Week(usizedb),
    Month(usizedb),
    Year(usizedb),
}

impl PostFilterTime {
    pub fn from_str(time: &str, time_num: usizedb) -> Self {
        match time {
            "lifetime" => PostFilterTime::Lifetime,
            "day" => PostFilterTime::Day(time_num),
            "week" => PostFilterTime::Week(time_num),
            "month" => PostFilterTime::Month(time_num),
            "year" => PostFilterTime::Year(time_num),
            _ => PostFilterTime::Week(1),
        }
    }
    pub fn to_timestamp(&self, negative: bool) -> usizedb {
        let days = match self {
            PostFilterTime::Lifetime => usizedb::MAX,
            PostFilterTime::Day(v) => {
                if *v > 1 {
                    *v
                } else {
                    1
                }
            }
            PostFilterTime::Week(v) => {
                if *v > 1 {
                    v * 7
                } else {
                    7
                }
            }
            PostFilterTime::Month(v) => {
                if *v > 1 {
                    v * 30
                } else {
                    30
                }
            }
            PostFilterTime::Year(v) => {
                if *v > 1 {
                    v * 365
                } else {
                    365
                }
            }
        } as usizedb;
        let duration = Duration::days(days as _);
        let now = Utc::now();
        let time = if negative {
            now - duration
        } else {
            now + duration
        };
        time.timestamp() as _
    }

    pub fn is_lifetime(&self) -> bool {
        self == &PostFilterTime::Lifetime
    }
}
