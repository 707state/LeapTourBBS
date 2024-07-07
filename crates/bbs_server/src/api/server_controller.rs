use self::model::ServerInfo;
use super::api_error::ApiError;
use crate::api::SDW;
use actix_web::{get, web::Json};

mod model;

#[get("/server_info")]
pub async fn get_server_info(s: SDW) -> Result<Json<ServerInfo>, ApiError> {
    let config = s.core.get_config();
    Ok(Json(ServerInfo {
        editable_seconds: config.editable_seconds as _,
        open_register: config.open_register,
        custom_post_cover_supported: config.custom_post_cover_supported,
        auth_active_days: config.auth_active_days as _,
    }))
}
