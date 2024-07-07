use bbs_utils::usizedb;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use like_system::model::LikeStatusFlag;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLikeStatusQuery {
    pub flag: LikeStatusFlag,
    pub flag_ref_id: usizedb,
    pub created_by_id: usizedb,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum LikeAction {
    Like,
    Dislike,
    UnknownLike,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LikeActionBody {
    pub action: LikeAction,
    pub flag: LikeStatusFlag,
}

