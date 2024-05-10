use chrono::naive::NaiveDateTime;
use chrono::NaiveDate;
use postgres::types::ToSql;
use postgres::{Client, NoTls};
use sea_orm::{
    ColumnTrait, Database, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
};
use std::error::Error;
use std::time::Duration;
use std::{env, thread, time};
use tokio::time::sleep;
use tokio_postgres;

use crate::entities::{actor, performance, ticket, viewer};
use crate::entity::prelude::{
    ActorS, PerformanceActorsS, PerformanceS, PlayS, PosterS, StageS, TheaterS, TicketS, ViewerS, ViewerTicketS,
};
use crate::Performance;
use crate::PerformanceActors;
use crate::Play;
use crate::Poster;
use crate::Stage;
use crate::Theater;
use crate::Ticket;
use crate::Viewer;
use crate::ViewerTicket;
use crate::{entity, Actor};

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost/Theatre";

thread_local!(static CONF: String  = env::var("DATABASE_CONF").unwrap());

// const CONFA: Option<String> = Some(env::var("DATABASE_CONF"));

pub(super) async fn sea_connection() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    Ok(db)
}

pub async fn get_actors() -> Result<Vec<entity::actor::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(err) => panic!("{}", "naebnulos"),
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
        Err(err) => panic!("{}", "naebnulos"),
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
        Err(err) => panic!("{}", "naebnulos"),
    };

    let theaters: Vec<entity::theater::Model> = TheaterS::find().all(&db).await?;
    Ok(theaters)
}
pub async fn get_performances() -> Result<Vec<entity::performance::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "naebnulos"),
    };
    let performances: Vec<entity::performance::Model> = PerformanceS::find().all(&db).await?;
    Ok(performances)
}
pub async fn get_poster() -> Result<Vec<entity::poster::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "naebnulos"),
    };
    let posters: Vec<entity::poster::Model> = PosterS::find().all(&db).await?;
    Ok(posters)
}
pub async fn get_viewer() -> Result<Vec<entity::viewer::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "naebnulos"),
    };
    let viewers: Vec<entity::viewer::Model> = ViewerS::find().all(&db).await?;
    Ok(viewers)
}

pub async fn get_stage() -> Result<Vec<entity::stage::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "naebnulos"),
    };
    let stages: Vec<entity::stage::Model> = StageS::find().all(&db).await?;
    Ok(stages)
}

pub async fn get_per_actors() -> Result<Vec<entity::performance_actors::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "naebnulos"),
    };
    let per_act: Vec<entity::performance_actors::Model> =
        PerformanceActorsS::find().all(&db).await?;
    Ok(per_act)
}

pub async fn get_plays() -> Result<Vec<entity::play::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "naebnulos"),
    };
    let per_act: Vec<entity::play::Model> = PlayS::find().all(&db).await?;
    Ok(per_act)
}

pub async fn get_viewer_ticket() -> Result<Vec<entity::viewer_ticket::Model>, Box<dyn Error>> {
    let db = match sea_connection().await {
        Ok(db) => db,
        Err(_) => panic!("{}", "naebnulos"),
    };
    let per_act: Vec<entity::viewer_ticket::Model> = ViewerTicketS::find().all(&db).await?;
    Ok(per_act)
}

pub async fn actors() -> Result<Vec<Actor>, Box<dyn Error>> {
    let (client, connection) =
        tokio_postgres::connect(&CONF.with(|text| text.clone()), NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    // CONF.with(|text| { *text.clone()});
    let mut actors: Vec<Actor> = vec![];
    for row in client
        .query("SELECT actor_id, name, surname, role  FROM actor", &[])
        .await?
    {
        let id: Option<i32> = row.get(0);
        let name: Option<&str> = row.get(1);
        let surname: Option<&str> = row.get(2);
        let role: Option<&str> = row.get(3);
        let a = Actor::new(
            id,
            Some(name.unwrap_or_default().to_owned()),
            Some(surname.unwrap_or_default().to_owned()),
            Some(role.unwrap_or_default().to_owned()),
        );
        // println!("found person: {:?} ", a);
        actors.push(a);
    }
    Ok(actors)
}
// #[tokio::main]

pub async fn performance() -> Result<Vec<Performance>, Box<dyn Error>> {
    // let mut  client = Client::connect("host=localhost user=postgres dbname='Theatre' password=postgres", NoTls)?;

    let (client, connection) =
        tokio_postgres::connect(&CONF.with(|text| text.clone()), NoTls).await?;
    let mut performance: Vec<Performance> = vec![];

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    for row in client
        .query(
            "SELECT performance_id, play_id, stage_id, start_datetime  FROM performance",
            &[],
        )
        .await?
    {
        let per_id: Option<i32> = row.get(0);
        let p_id: Option<i32> = row.get(1);
        let s_id: Option<i32> = row.get(2);
        let date: Option<NaiveDateTime> = row.get(3);
        let a = Performance::new(
            per_id,
            p_id,
            s_id,
            Some(date.unwrap_or_default().to_string()),
        );
        // println!("found person: {:?} ", a);
        performance.push(a);
    }
    Ok(performance)
}

// #[tokio::main]
pub async fn performance_actors() -> Result<Vec<PerformanceActors>, Box<dyn Error>> {
    // let mut  client = Client::connect("host=localhost user=postgres dbname='Theatre' password=postgres", NoTls)?;

    // async
    // let (client, connection) = tokio_postgres::connect(
    //     "host=localhost user=postgres dbname='Theatre' password=postgres",
    //     NoTls,
    // )
    // .await?;

    let (client, connection) =
        tokio_postgres::connect(&CONF.with(|text| text.clone()), NoTls).await?;

    // let mut performance_a: Vec<PerformanceActors> = vec![];
    //     for row in client.query("SELECT performance_performance_id, actor_actor_id, amount, actors_perfor_id  FROM performance_actors", &[])?{
    //         let performance_performance_id: i32 = row.get(0);
    //         let actor_actor_id: i32 = row.get(1);
    //         let amount: i32 = row.get(2);
    //         let aactors_perfor_id: i32 =row.get(3);
    //         let a = PerformanceActors::new(performance_performance_id , actor_actor_id, amount,aactors_perfor_id );
    //         println!("found person: {:?} ", a);
    //         performance_a.push(a);
    //     }

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let mut performance_a: Vec<PerformanceActors> = vec![];
    for row in client.query("SELECT performance_performance_id, actor_actor_id, amount, actors_perfor_id  FROM performance_actors", &[]).await?{
        let performance_performance_id: Option<i32> = row.get(0);
        let actor_actor_id:  Option<i32>= row.get(1);
        let amount:  Option<i32>= row.get(2);
        let aactors_perfor_id:  Option<i32> =row.get(3);
        let a = PerformanceActors::new(performance_performance_id , actor_actor_id, amount,aactors_perfor_id );
        // println!("found person: {:?} ", a);
        performance_a.push(a);
    }
    Ok(performance_a)
}

// #[tokio::main]
pub async fn play() -> Result<Vec<Play>, Box<dyn Error>> {
    // let mut  client = Client::connect("host=localhost user=postgres dbname='Theatre' password=postgres", NoTls)?;

    // async
    // let (client, connection) = tokio_postgres::connect(
    //     "host=localhost user=postgres dbname='Theatre' password=postgres",
    //     NoTls,
    // )
    // .await?;
    let (client, connection) =
        tokio_postgres::connect(&CONF.with(|text| text.clone()), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let mut play: Vec<Play> = vec![];
    for row in client
        .query("SELECT play_id, title ,author,  director  FROM play", &[])
        .await?
    {
        let play_id: Option<i32> = row.get(0);
        let title: Option<String> = row.get(1);
        let author: Option<String> = row.get(2);
        let director: Option<String> = row.get(3);
        let a = Play::new(play_id, title, author, director);
        // println!("found play: {:?} ", a);
        play.push(a);
    }

    Ok(play)
}

// #[tokio::main]
pub async fn poster() -> Result<Vec<Poster>, Box<dyn Error>> {
    // let (client, connection) = tokio_postgres::connect(
    //     "host=localhost user=postgres dbname='Theatre' password=postgres",
    //     NoTls,
    // )
    // .await?;

    let (client, connection) =
        tokio_postgres::connect(&CONF.with(|text| text.clone()), NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let mut poster: Vec<Poster> = vec![];
    for row in client
        .query(
            "select poster_id, performance_id, start_date, end_date, content from poster",
            &[],
        )
        .await?
    {
        let poster_id: Option<i32> = row.get(0);
        let performance_id: Option<i32> = row.get(1);
        let start_date: Option<NaiveDate> = row.get(2);
        let end_date: Option<NaiveDate> = row.get(3);
        let content: Option<String> = row.get(4);
        let a = Poster::new(
            poster_id.to_owned(),
            performance_id,
            Some(start_date.unwrap_or_default().to_string()),
            Some(end_date.unwrap_or_default().to_string()),
            content,
        );
        // println!("found play: {:?} ", a);
        poster.push(a);
    }

    Ok(poster)
}
// #[tokio::main]
pub async fn stage() -> Result<Vec<Stage>, Box<dyn Error>> {
    // let (client, connection) = tokio_postgres::connect(
    //     "host=localhost user=postgres dbname='Theatre' password=postgres",
    //     NoTls,
    // )
    // .await?;
    let (client, connection) =
        tokio_postgres::connect(&CONF.with(|text| text.clone()), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let mut stage: Vec<Stage> = vec![];
    for row in client
        .query("select stage_id, theater_id, capacity from stage", &[])
        .await?
    {
        let stage_id: Option<i32> = row.get(0);
        let theater_id: Option<i32> = row.get(1);
        let capacity: Option<i32> = row.get(2);
        let a = Stage::new(stage_id, theater_id, capacity);
        // println!("found stage: {:?} ", a);
        stage.push(a);
    }

    Ok(stage)
}

// #[tokio::main]
// pub  async fn theater() -> Result<Vec<PerformanceActors>, Box<dyn Error>>{
//     "select theater_id, name, address, capacity from theater"
// }
// #[tokio::main]
// pub  async fn ticket() -> Result<Vec<PerformanceActors>, Box<dyn Error>>{
//     "select ticket_id, performance_id, seat_number,date, cost, status from ticket"
// }
// #[tokio::main]
// pub  async fn viewer() -> Result<Vec<PerformanceActors>, Box<dyn Error>>{
//     "select viewer_id, name, email,phone from viewer"
// }
// #[tokio::main]
// pub  async fn viewer_ticket() -> Result<Vec<PerformanceActors>, Box<dyn Error>>{
//     "select viewer_viewer_id, ticket_ticket_id, bought_date,vi_ti_id from viewer_ticket"

// }

pub async fn theater() -> Result<Vec<Theater>, Box<dyn Error>> {
    let (client, connection) =
        tokio_postgres::connect(&CONF.with(|text| text.clone()), NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let mut theater: Vec<Theater> = vec![];
    for row in client
        .query(
            "select theater_id,name, address, capacity from theater",
            &[],
        )
        .await?
    {
        let theater_id: Option<i32> = row.get(0);
        let name: Option<String> = row.get(1);
        let address: Option<String> = row.get(2);
        let capacity: Option<i32> = row.get(3);
        let a = Theater::new(theater_id, name, address, capacity);
        // println!("found theater: {:?} ", a);
        theater.push(a);
    }

    Ok(theater)
}

pub async fn ticket() -> Result<Vec<Ticket>, Box<dyn Error>> {
    let (client, connection) =
        tokio_postgres::connect(&CONF.with(|text| text.clone()), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let mut ticket: Vec<Ticket> = vec![];
    for row in client
        .query(
            "select ticket_id, performance_id, seat_number, date, cost, status from ticket",
            &[],
        )
        .await?
    {
        let ticket_id: Option<i32> = row.get(0);
        let per_id: Option<i32> = row.get(1);
        let seat_number: Option<i32> = row.get(2);
        let date: Option<NaiveDate> = row.get(3);
        let cost: Option<i32> = row.get(4);
        let status: Option<String> = row.get(5);
        let a = Ticket::new(
            ticket_id,
            per_id,
            seat_number,
            Some(date.unwrap().to_string()),
            cost,
            status,
        );
        // println!("found theater: {:?} ", a);
        ticket.push(a);
    }

    Ok(ticket)
}

pub async fn viewer_ticket() -> Result<Vec<ViewerTicket>, Box<dyn Error>> {
    let (client, connection) =
        tokio_postgres::connect(&CONF.with(|text| text.clone()), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let mut vticket: Vec<ViewerTicket> = vec![];
    for row in client
        .query(
            "select viewer_viewer_id, ticket_ticket_id, bought_date, vi_ti_id from viewer_ticket",
            &[],
        )
        .await?
    {
        let viewer_viewer_id: Option<i32> = row.get(0);
        let ticket_ticket_id: Option<i32> = row.get(1);
        let bought_date: Option<NaiveDate> = row.get(2);
        let vi_ti_id: Option<i32> = row.get(3);
        let a = ViewerTicket::new(
            viewer_viewer_id,
            ticket_ticket_id,
            Some(bought_date.unwrap().to_string()),
            vi_ti_id,
        );
        // println!("found theater: {:?} ", a);
        vticket.push(a);
    }

    Ok(vticket)
}

pub async fn viewer() -> Result<Vec<Viewer>, Box<dyn Error>> {
    let (client, connection) =
        tokio_postgres::connect(&CONF.with(|text| text.clone()), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let mut viewer: Vec<Viewer> = vec![];
    for row in client
        .query("select viewer_id, name, email, phone from viewer", &[])
        .await?
    {
        let viewer_id: Option<i32> = row.get(0);
        let name: Option<String> = row.get(1);
        let email: Option<String> = row.get(2);
        let phone: Option<String> = row.get(3);
        let a = Viewer::new(viewer_id, name, email, phone);
        // println!("found viewer: {:?} ", a);
        viewer.push(a);
    }

    Ok(viewer)
}

pub async fn writer(statment: String) -> Result<(), Box<dyn Error>> {
    let (client, connection) =
        tokio_postgres::connect(&CONF.with(|text| text.clone()), NoTls).await?;

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

    let (client, connection) =
        tokio_postgres::connect(&CONF.with(|text| text.clone()), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    // let vec_of_to_sql: Vec<&(dyn ToSql + Sync)> = params.iter().map(|s| s as &(dyn ToSql + Sync)).collect();

    let mut result = client.query(&statment, &[]).await?;

    Ok("test".to_string())
}
