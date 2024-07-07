use serde::{Deserialize, Serialize};

use bbs_utils::usizedb;

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationKey {
    pub verification_id: usizedb,
    pub secret_key: String,
}

