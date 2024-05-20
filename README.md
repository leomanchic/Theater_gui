## Театр.
В терминах ER-модели описать предметную область системы
бронирования билетов на спектакли некоторого театра.
Театр имеет несколько сценических площадок и вместимостью
зрительного зала . В репертуар театра входит ограниченное количество пьес
разных авторов. В каждой пьесе имеется ограниченное количество ролей, в
которых заняты не только актеры своей труппы. Спектакли ставятся
режиссерами. Дата и время представления спектаклей указывается в афише
на текущий и следующий месяцы. Часть билетов с указанием мест поступают
кассиру для последующего бронирования в последних числах текущего
месяца. Зритель может заказать билеты с условием выкупа их не позднее,
чем за 1 день. Если он их не выкупает, они поступают в свободную продажу.
Система должна содержать информацию о:
всех сценах театра (адрес, их вместимость)
репертуаре театра (названия пьесы, автор, режиссер)
программа спектакля (роли и занятые актеры)
афиша (место и время начала спектакля)
билеты
с
указанием
номера места, даты и
стоимости
•заказы билетов

**Система должна эффективно выдавать ответы на следующие
запросы:**
1. Найти все пьесы, в которых занят заданный актер.
2. Выдать афишу театра за заданный период.
3. Можно ли забронировать билеты на определенный спектакль.
4. Список стоимости билетов на указанный спектакль
5. Программу спектакля
6. Регистрация нового поступления билетов для бронирования.
7. Ежедневное снятие с брони не выкупленных билетов
8. Регистрация заказа на бронирование.
9. Продажа заказанного билета.




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


