//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "page_rate_vote")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub rate_id: i64,
    pub user_id: Option<i32>,
    pub page_id: Option<i32>,
    pub rate: i32,
    pub date: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::page::Entity",
        from = "Column::PageId",
        to = "super::page::Column::PageId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Page,
}

impl Related<super::page::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Page.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
