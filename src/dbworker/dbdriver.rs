use chrono::naive::NaiveDateTime;
use chrono::{NaiveDate, Utc};
use sea_orm::{
    ColumnTrait, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, DbErr,
    EntityTrait, QueryFilter, QueryOrder, QueryResult, Statement,
};
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio_postgres;
use tokio_postgres::{Client, NoTls};

use crate::entity::prelude::{
    Actor, Performance, PerformanceActors, Play, Poster, Stage, Theater, Ticket, Viewer,
    ViewerTicket,
};

use crate::entity;

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost/Theatre";
const CONF: &str = "host=localhost user=postgres dbname='Theatre' password=postgres";

// thread_local!(static CONF: String  = env::var("DATABASE_CONF").unwrap());
//

pub async fn sea_connection() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    Ok(db)
}

pub async fn get_actors() -> Result<Vec<entity::actor::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(err) => panic!("{}", "Error on actors get"),
    };
    let actors: Vec<entity::actor::Model> = Actor::find()
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
    let ticket: Vec<entity::ticket::Model> = Ticket::find()
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

    let theaters: Vec<entity::theater::Model> = Theater::find().all(&db).await?;
    Ok(theaters)
}
pub async fn get_performances() -> Result<Vec<entity::performance::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "Error on performances get"),
    };
    let performances: Vec<entity::performance::Model> = Performance::find().all(&db).await?;
    Ok(performances)
}
pub async fn get_poster() -> Result<Vec<entity::poster::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "Error on posters get"),
    };
    let posters: Vec<entity::poster::Model> = Poster::find().all(&db).await?;
    Ok(posters)
}
pub async fn get_viewer() -> Result<Vec<entity::viewer::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "Error on viewers get"),
    };
    let viewers: Vec<entity::viewer::Model> = Viewer::find().all(&db).await?;
    Ok(viewers)
}

pub async fn get_stage() -> Result<Vec<entity::stage::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "Error on stages get"),
    };
    let stages: Vec<entity::stage::Model> = Stage::find().all(&db).await?;
    Ok(stages)
}

pub async fn get_per_actors() -> Result<Vec<entity::performance_actors::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "Error on performance actors get"),
    };
    let per_act: Vec<entity::performance_actors::Model> =
        PerformanceActors::find().all(&db).await?;
    Ok(per_act)
}

pub async fn get_plays() -> Result<Vec<entity::play::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "Error on plays get"),
    };
    let per_act: Vec<entity::play::Model> = Play::find().all(&db).await?;
    Ok(per_act)
}

pub async fn get_viewer_ticket() -> Result<Vec<entity::viewer_ticket::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "Error on viewrer ticket get"),
    };
    let per_act: Vec<entity::viewer_ticket::Model> = ViewerTicket::find().all(&db).await?;
    Ok(per_act)
}

pub async fn actor_creator(
    name: &Arc<Mutex<String>>,
    surmame: &Arc<Mutex<String>>,
    role: &Arc<Mutex<String>>,
) -> () {
    let db = sea_connection().await.unwrap();
    let act = entity::actor::ActiveModel {
        name: sea_orm::ActiveValue::Set(Some(name.lock().unwrap().to_string())),
        surname: sea_orm::ActiveValue::Set(Some(surmame.lock().unwrap().to_string())),
        role: sea_orm::ActiveValue::Set(Some(role.lock().unwrap().to_string())),
        ..Default::default()
    };
    Actor::insert(act).exec(&db).await.unwrap();
}

pub async fn performance_creator(
    name: &Arc<Mutex<String>>,
    surmame: &Arc<Mutex<String>>,
    role: &Arc<Mutex<std::option::Option<NaiveDate>>>,
) -> () {
    let db = sea_connection().await.unwrap();
    let per = entity::performance::ActiveModel {
        play_id: sea_orm::ActiveValue::Set(Some(name.lock().unwrap().parse::<i32>().unwrap())),
        stage_id: sea_orm::ActiveValue::Set(Some(surmame.lock().unwrap().parse::<i32>().unwrap())),
        // start_datetime: sea_orm::ActiveValue::Set(Some(role.lock().unwrap().unwrap())),
        ..Default::default()
    };
    Performance::insert(per).exec(&db).await.unwrap();
}

pub async fn performance_actor_creator(
    per_id: &Arc<Mutex<String>>,
    act_id: &Arc<Mutex<String>>,
    amount: &Arc<Mutex<String>>,
) -> Result<String, Box<dyn Error>> {
    let db = sea_connection().await.unwrap();
    let per = entity::performance_actors::ActiveModel {
        performance_performance_id: sea_orm::ActiveValue::Set(
            per_id.lock().unwrap().parse::<i32>().unwrap(),
        ),
        actor_actor_id: sea_orm::ActiveValue::Set(act_id.lock().unwrap().parse::<i32>().unwrap()),
        amount: sea_orm::ActiveValue::Set(Some(amount.lock().unwrap().parse::<i32>().unwrap())),
        // start_datetime: sea_orm::ActiveValue::Set(Some(role.lock().unwrap().unwrap())),
        ..Default::default()
    };

    let response = PerformanceActors::insert(per).exec(&db).await;
    match response {
        Ok(c) => Ok(format! {"succesfull creation"}),
        Err(err) => Ok(format!("Err: {}", err.to_string())),
    }
}

pub async fn play_creator(
    title: &Arc<Mutex<String>>,
    author: &Arc<Mutex<String>>,
    director: &Arc<Mutex<String>>,
) -> Result<(), Box<dyn Error>> {
    let db = sea_connection().await.unwrap();
    let per = entity::play::ActiveModel {
        title: sea_orm::ActiveValue::Set(Some(title.lock().unwrap().to_string())),
        author: sea_orm::ActiveValue::Set(Some(author.lock().unwrap().to_string())),
        director: sea_orm::ActiveValue::Set(Some(director.lock().unwrap().to_string())),
        // start_datetime: sea_orm::ActiveValue::Set(Some(role.lock().unwrap().unwrap())),
        ..Default::default()
    };
    Play::insert(per).exec(&db).await.unwrap();
    // let response = PlayS::insert(per).exec(&db).await;
    // match response {
    //     Ok(c) => Ok(format! {"succesfull creation"}),
    //     Err(err) => Ok(format!("Err: {}", err.to_string())),
    // }
    Ok(())
}

pub async fn stage_creator(theater_id: &Arc<Mutex<String>>, capacity: &Arc<Mutex<String>>) {
    let db = sea_connection().await.unwrap();
    let per = entity::stage::ActiveModel {
        theater_id: sea_orm::ActiveValue::Set(Some(
            theater_id.lock().unwrap().parse::<i32>().unwrap(),
        )),
        capacity: sea_orm::ActiveValue::Set(Some(capacity.lock().unwrap().parse::<i32>().unwrap())),
        // start_datetime: sea_orm::ActiveValue::Set(Some(role.lock().unwrap().unwrap())),
        ..Default::default()
    };
    Stage::insert(per).exec(&db).await.unwrap();
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
    // todo!("psql functionality");

    // let db = sea_connection().await.unwrap();
    // let query_res_vec = db
    //     .query_all(Statement::from_string(DatabaseBackend::Postgres, &statment))
    //     .await;

    let (client, connection) = tokio_postgres::connect(&CONF, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    // let vec_of_to_sql: Vec<&(dyn ToSql + Sync)> = params.iter().map(|s| s as &(dyn ToSql + Sync)).collect();

    let query_res_vec = client.query(&statment, &[]).await;

    match query_res_vec {
        Ok(c) => {
            let r = format!("{:?}", c);
            Ok(r)
        }
        Err(k) => Ok(format! {"Error {k}"}),
    }
}
