use axum::async_trait;
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

use crate::database::get_db;

use super::record;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub github_id: i64,
    pub login: String,
    pub avatar_url: String,
    pub last_access_at: i64,
}

impl Model {}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Record,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Record => Entity::has_many(record::Entity).into(),
        }
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            ..ActiveModelTrait::default()
        }
    }

    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(self)
    }
}
