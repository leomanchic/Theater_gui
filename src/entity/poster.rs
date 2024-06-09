//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, serde::Deserialize, serde::Serialize)]
#[sea_orm(table_name = "poster")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub poster_id: i32,
    pub performance_id: Option<i32>,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    #[sea_orm(column_type = "Text", nullable)]
    pub content: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::performance::Entity",
        from = "Column::PerformanceId",
        to = "super::performance::Column::PerformanceId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Performance,
}

impl Related<super::performance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Performance.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}