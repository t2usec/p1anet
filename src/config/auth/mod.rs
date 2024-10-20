pub mod github;
pub mod jwt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub jwt: jwt::Config,
    pub github: github::Config,
}
