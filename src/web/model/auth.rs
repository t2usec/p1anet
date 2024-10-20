use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CallbackRequest {
    pub code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CallbackResponse {
    pub code: u16,
    pub data: crate::model::user::Model,
    pub token: String,
}
