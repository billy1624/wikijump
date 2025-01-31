//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "page_revision")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub revision_id: i64,
    pub page_id: Option<i32>,
    pub metadata_id: Option<i32>,
    pub flags: Option<String>,
    pub flag_text: bool,
    pub flag_title: bool,
    pub flag_file: bool,
    pub flag_rename: bool,
    pub flag_meta: bool,
    pub flag_new: bool,
    pub revision_number: Option<i32>,
    pub date_last_edited: Option<DateTime>,
    pub user_id: Option<i32>,
    pub user_string: Option<String>,
    pub comments: Option<String>,
    pub site_id: Option<i32>,
    pub wikitext_hash: Vec<u8>,
    pub compiled_hash: Vec<u8>,
    #[sea_orm(column_type = "Text")]
    pub compiled_generator: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::text::Entity",
        from = "Column::CompiledHash",
        to = "super::text::Column::Hash",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Text2,
    #[sea_orm(
        belongs_to = "super::text::Entity",
        from = "Column::WikitextHash",
        to = "super::text::Column::Hash",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Text1,
}

impl ActiveModelBehavior for ActiveModel {}
