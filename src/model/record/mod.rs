use axum::async_trait;
use sea_orm::{entity::prelude::*, sea_query::ForeignKeyAction};
use serde::{Deserialize, Serialize};

use super::user;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "records")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub obtained_at: i64,

    #[sea_orm(ignore)]
    pub user: Option<user::Model>,
}

impl Model {}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(user::Entity)
                .from(Column::UserId)
                .to(user::Column::GithubId)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
        }
    }
}

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
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
