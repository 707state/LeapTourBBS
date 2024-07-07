use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use bbs_utils::usizedb;

#[derive(Debug, FromRow)]
pub struct Verification {
    pub id: usizedb,
    pub create_at: usizedb,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerificationKeyPicture {
    pub verification: usizedb,
    pub secret_key_picture_url: String,
}
