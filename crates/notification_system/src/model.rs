use bbs_utils::usizedb;
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserNotificationArguments {
    pub ref_id: usizedb,
    pub target_user_id: usizedb,
    pub n_type: UserNotificationType,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserNotification {
    pub id: usizedb,
    pub ref_id: usizedb,
    pub target_user_id: usizedb,
    pub created_by_id: usizedb,
    pub created_at: usizedb,
    pub n_type: UserNotificationType,
    pub readed: bool,
}

//枚举所有的用户通知状态
#[derive(Debug, Deserialize, Serialize_repr, PartialEq, Clone, sqlx::Type)]
#[repr(u8)]
pub enum UserNotificationType {
    Comment,
    ReplyComment,
    LikePost,
    DislikePost,
    LikeComment,
    DislikeComment,
}
