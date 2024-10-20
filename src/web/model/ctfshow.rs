use serde::{Deserialize, Serialize};
use thirtyfour::Cookie;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetCookiesResponse {
    pub code: u16,
    pub data: Vec<Cookie>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetStatusResponse {
    pub code: u16,
    pub data: Option<crate::model::record::Model>,
}
