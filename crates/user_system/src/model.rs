use bbs_utils::{calc_hash, usizedb};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
//枚举状态
#[derive(Debug, Deserialize_repr, Serialize_repr, PartialEq, Clone, Copy, sqlx::Type)]
#[repr(u8)]
pub enum UserStatus {
    Active,
    Banned,
    OnlyComment,
    Observer,
}
//枚举用户种类
#[derive(
    Debug, Deserialize_repr, Serialize_repr, PartialEq, Clone, Copy, sqlx::Type, PartialOrd,
)]
#[repr(u8)]
pub enum UserType {
    Guest,
    General,
    Administrator,
}
//定义用户信息
#[derive(Debug, sqlx::FromRow, Deserialize, Serialize, Clone)]
pub struct UserInfo {
    pub id: usizedb,
    pub email: String,
    pub username: String,
    pub alias: String,
    pub password: String,
    pub group_ids: Vec<usizedb>,
    pub status: UserStatus,
    pub user_type: UserType,
    pub avatar_url: Option<String>,
    pub signature: String,
    pub created_at: usizedb,
    pub total_post: usizedb,
    pub total_comment: usizedb,
}
//采用安全方法实现
#[derive(Debug, sqlx::FromRow, Deserialize, Serialize, Clone)]
pub struct SafeUserInfo {
    pub id: usizedb,
    pub alias: String,
    pub username: String,
    pub group_ids: Vec<usizedb>,
    pub status: UserStatus,
    pub user_type: UserType,
    pub avatar_url: Option<String>,
    pub signature: String,
    pub created_at: usizedb,
    pub total_post: usizedb,
    pub total_comment: usizedb,
}
//创建用户
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserToCreate {
    pub email: String,
    pub username: String,
    pub password: String,
    pub alias: String,
}
//更新用户
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserToUpdate {
    pub email: String,
    pub password: String,
    pub alias: String,
    pub avatar_url: Option<String>,
    pub signature: String,
}
//实现一个生成方法
impl UserInfo {
    pub fn generate_auth(&self) -> String {
        let now = Utc::now().timestamp();
        let content = format!("{}{}{}-{}", &self.id, &self.username, &self.password, now);
        calc_hash(&content).to_string()
    }
    //判断是否为管理员
    pub fn is_admin(&self) -> bool {
        self.user_type == UserType::Administrator
    }
}
//转换方法
impl From<UserInfo> for SafeUserInfo {
    fn from(val: UserInfo) -> Self {
        SafeUserInfo {
            id: val.id,
            alias: val.alias,
            username: val.username,
            group_ids: val.group_ids,
            status: val.status,
            user_type: val.user_type,
            avatar_url: val.avatar_url,
            signature: val.signature,
            created_at: val.created_at,
            total_post: val.total_post,
            total_comment: val.total_comment,
        }
    }
}
