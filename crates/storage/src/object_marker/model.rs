use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use bbs_utils::usizedb;

#[derive(
    Debug, Deserialize_repr, Serialize_repr, PartialEq, Clone, Copy, sqlx::Type, PartialOrd,
)]
//枚举数据存储情况
#[repr(u8)]
pub enum ObjectFlag {
    Captcha,       //验证码
    UserAvatar,    //头像
    PostCover,     //帖子的封面
    CategoryCover, //类型的封面
}
//
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarkedObject {
    pub key: String,
    pub created_at: usizedb,
    pub flag: ObjectFlag,
    pub flag_ref_id: usizedb,
    pub permanent: bool,
}
