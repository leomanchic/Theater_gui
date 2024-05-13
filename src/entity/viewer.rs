//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, serde::Deserialize, serde::Serialize)]
#[sea_orm(table_name = "viewer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub viewer_id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::viewer_ticket::Entity")]
    ViewerTicket,
}

impl Related<super::viewer_ticket::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ViewerTicket.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
