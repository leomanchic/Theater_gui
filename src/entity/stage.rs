//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "stage")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub stage_id: i32,
    pub theater_id: Option<i32>,
    pub capacity: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::performance::Entity")]
    Performance,
    #[sea_orm(
        belongs_to = "super::theater::Entity",
        from = "Column::TheaterId",
        to = "super::theater::Column::TheaterId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Theater,
}

impl Related<super::performance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Performance.def()
    }
}

impl Related<super::theater::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Theater.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
