use chrono::naive::NaiveDateTime;
use chrono::NaiveDate;
use sea_orm::{
    ColumnTrait, Database, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
};
use std::error::Error;
use tokio_postgres;
use tokio_postgres::{Client, NoTls};

use crate::entity::prelude::{
    ActorS, PerformanceActorsS, PerformanceS, PlayS, PosterS, StageS, TheaterS, TicketS, ViewerS,
    ViewerTicketS,
};

use crate::entity;

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost/Theatre";
const CONF: &str = "host=localhost user=postgres dbname='Theatre' password=postgres";

// thread_local!(static CONF: String  = env::var("DATABASE_CONF").unwrap());
//

pub(super) async fn sea_connection() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    Ok(db)
}

pub async fn get_actors() -> Result<Vec<entity::actor::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(err) => panic!("{}", "Error on actors get"),
    };
    let actors: Vec<entity::actor::Model> = ActorS::find()
        // .filter(entity::actor::Column::Name.contains("chocolate"))
        .order_by_asc(entity::actor::Column::Name)
        .all(&db)
        .await?;

    Ok(actors)
}

pub async fn get_tickets() -> Result<Vec<entity::ticket::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(err) => panic!("{}", "Error on tickets get"),
    };
    let ticket: Vec<entity::ticket::Model> = TicketS::find()
        // .filter(entity::actor::Column::Name.contains("chocolate"))
        // .order_by_asc(entity::theater::Column::TheaterId)
        .all(&db)
        .await?;
    println!("{:?}", ticket);
    Ok(ticket)
}
pub async fn get_theaters() -> Result<Vec<entity::theater::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(err) => panic!("{}", "Error on theaters get"),
    };

    let theaters: Vec<entity::theater::Model> = TheaterS::find().all(&db).await?;
    Ok(theaters)
}
pub async fn get_performances() -> Result<Vec<entity::performance::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "Error on performances get"),
    };
    let performances: Vec<entity::performance::Model> = PerformanceS::find().all(&db).await?;
    Ok(performances)
}
pub async fn get_poster() -> Result<Vec<entity::poster::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "Error on posters get"),
    };
    let posters: Vec<entity::poster::Model> = PosterS::find().all(&db).await?;
    Ok(posters)
}
pub async fn get_viewer() -> Result<Vec<entity::viewer::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "Error on viewers get"),
    };
    let viewers: Vec<entity::viewer::Model> = ViewerS::find().all(&db).await?;
    Ok(viewers)
}

pub async fn get_stage() -> Result<Vec<entity::stage::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "Error on stages get"),
    };
    let stages: Vec<entity::stage::Model> = StageS::find().all(&db).await?;
    Ok(stages)
}

pub async fn get_per_actors() -> Result<Vec<entity::performance_actors::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "Error on performance actors get"),
    };
    let per_act: Vec<entity::performance_actors::Model> =
        PerformanceActorsS::find().all(&db).await?;
    Ok(per_act)
}

pub async fn get_plays() -> Result<Vec<entity::play::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "Error on plays get"),
    };
    let per_act: Vec<entity::play::Model> = PlayS::find().all(&db).await?;
    Ok(per_act)
}

pub async fn get_viewer_ticket() -> Result<Vec<entity::viewer_ticket::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "Error on viewrer ticket get"),
    };
    let per_act: Vec<entity::viewer_ticket::Model> = ViewerTicketS::find().all(&db).await?;
    Ok(per_act)
}

pub async fn writer(statment: String) -> Result<(), Box<dyn Error>> {
    let (client, connection) = tokio_postgres::connect(&CONF, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    // let vec_of_to_sql: Vec<&(dyn ToSql + Sync)> = params.iter().map(|s| s as &(dyn ToSql + Sync)).collect();

    client.execute(&statment, &[]).await?;
    Ok(())
}
pub async fn rsql_executor(statment: String) -> Result<String, Box<dyn Error>> {
    todo!("psql functionality");

    let (client, connection) = tokio_postgres::connect(CONF, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    // let vec_of_to_sql: Vec<&(dyn ToSql + Sync)> = params.iter().map(|s| s as &(dyn ToSql + Sync)).collect();

    let mut result = client.query(&statment, &[]).await?;

    Ok("test".to_string())
}
