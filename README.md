# Actor Data Model



Содержание:

- Уровень API, сервисы и UI
- Как актеры представлены в БД
- Архитекрура системы
---

# Схематичное представление интерфейса


[![GUI](https://app.eraser.io/workspace/woFeXqetSkAWaRbPp14v/preview?elements=Kp0B8O81kzgeG9pidlDU_A&type=embed)](https://app.eraser.io/workspace/woFeXqetSkAWaRbPp14v?elements=Kp0B8O81kzgeG9pidlDU_A)

# Приложение и его сервисы
[![App Representation: User](https://app.eraser.io/workspace/woFeXqetSkAWaRbPp14v/preview?elements=I8B62krAznuNELFuUcsBWg&type=embed)](https://app.eraser.io/workspace/woFeXqetSkAWaRbPp14v?elements=I8B62krAznuNELFuUcsBWg)

Rust actor model:

```
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "actor")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub actor_id: i32,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub role: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::performance_actors::Entity")]
    PerformanceActors,
}

impl Related<super::performance_actors::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PerformanceActors.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
```
## Представление БД
Связь многие ко многим актерами и представлениями.

[![DB Representation](https://app.eraser.io/workspace/woFeXqetSkAWaRbPp14v/preview?elements=MTnr_g8XwjA22zUwvC-fOw&type=embed)](https://app.eraser.io/workspace/woFeXqetSkAWaRbPp14v?elements=MTnr_g8XwjA22zUwvC-fOw)



---

# Сервисы


