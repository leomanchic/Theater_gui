use chrono::naive::NaiveDateTime;
use chrono::{NaiveDate};
use postgres::types::ToSql;
use postgres::{Client, NoTls};
use tokio::time::sleep;
use tokio_postgres;
use std::error::Error;
use std::{thread, time};
use std::time::Duration;

use crate::performance::Performance;
use crate::play::Play;
use crate::stage::Stage;
use crate::Actor;
use crate::PerformanceActors;
use crate::poster::Poster;


// #[tokio::main]
pub  async fn actors() -> Result<Vec<Actor>, Box<dyn Error>>{
 
    // let mut  client = Client::connect("host=localhost user=postgres dbname='Theatre' password=postgres", NoTls)?;

    
    // let mut actors: Vec<Actor> = vec![];
    //     for row in client.query("SELECT actor_id, name, surname, role  FROM actor", &[])?{
    //         let id: i32 = row.get(0);
    //         let name: &str = row.get(1);
    //         let surname: &str = row.get(2);
    //         let role: &str = row.get(3);
    //         let a = Actor::new(id , name.to_owned(), surname.to_owned(), role.to_owned());
    //         println!("found person: {:?} ", a);
    //         actors.push(a);
    //     }

    //     Ok(actors)
    // thread::sleep(time::Duration::from_secs(5));

    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres dbname='Theatre' password=postgres", NoTls).await?;
        tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });
        
        let mut actors: Vec<Actor> = vec![];
        for row in client
        .query("SELECT actor_id, name, surname, role  FROM actor", &[])
        .await?{
            let id: i32 = row.get(0);
            let name: &str = row.get(1);
            let surname: &str = row.get(2);
            let role: &str = row.get(3);
            let a = Actor::new(id , name.to_owned(), surname.to_owned(), role.to_owned());
            println!("found person: {:?} ", a);
            actors.push(a);
        }
    Ok(actors)
}
// #[tokio::main]


pub async  fn performance() -> Result<Vec<Performance>, Box<dyn Error>>{
 
    // let mut  client = Client::connect("host=localhost user=postgres dbname='Theatre' password=postgres", NoTls)?;

    
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres dbname='Theatre' password=postgres", NoTls).await?;
    let mut performance: Vec<Performance> = vec![];
        // for row in client.query("SELECT performance_id, play_id, stage_id, start_datetime  FROM performance", &[])?{
        //     let per_id: i32 = row.get(0);
        //     let p_id: i32 = row.get(1);
        //     let s_id: i32 = row.get(2);
        //     let date: NaiveDateTime =row.get(3);
        //     let a = Performance::new(per_id , p_id, s_id, date.to_string());
        //     println!("found person: {:?} ", a);
        //     performance.push(a);
        // }

        // Ok(performance)


       
        tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });
        
        for row in client
        .query("SELECT performance_id, play_id, stage_id, start_datetime  FROM performance", &[])
        .await?{
            let per_id: i32 = row.get(0);
            let p_id: i32 = row.get(1);
            let s_id: i32 = row.get(2);
            let date: NaiveDateTime =row.get(3);
            let a = Performance::new(per_id , p_id, s_id, date.to_string());
            println!("found person: {:?} ", a);
            performance.push(a);
        }
        Ok(performance)
    }

// #[tokio::main]
pub  async fn performance_actors() -> Result<Vec<PerformanceActors>, Box<dyn Error>>{
 
    // let mut  client = Client::connect("host=localhost user=postgres dbname='Theatre' password=postgres", NoTls)?;

    // async 
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres dbname='Theatre' password=postgres", NoTls).await?;

    
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
        let performance_performance_id: i32 = row.get(0);
        let actor_actor_id: i32 = row.get(1);
        let amount: i32 = row.get(2);
        let aactors_perfor_id: i32 =row.get(3);
        let a = PerformanceActors::new(performance_performance_id , actor_actor_id, amount,aactors_perfor_id );
        println!("found person: {:?} ", a);
        performance_a.push(a);
    } 
        Ok(performance_a)
}


// #[tokio::main]
pub async  fn play() -> Result<Vec<Play>, Box<dyn Error>>{

    // let mut  client = Client::connect("host=localhost user=postgres dbname='Theatre' password=postgres", NoTls)?;

    // async 
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres dbname='Theatre' password=postgres", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    
    let mut play: Vec<Play> = vec![];
        for row in client.query("SELECT play_id, title ,author,  director  FROM play", &[]).await?{
            let play_id: i32 = row.get(0);
            let title: String = row.get(1);
            let author: String = row.get(2);
            let director: String =row.get(3);
            let a = Play::new(play_id , title, author,director );
            println!("found play: {:?} ", a);
            play.push(a);
        }
    
    Ok(play)
}

// #[tokio::main]
pub  async fn poster() -> Result<Vec<Poster>, Box<dyn Error>>{
    
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres dbname='Theatre' password=postgres", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    
    let mut poster: Vec<Poster> = vec![];
        for row in client.query("select poster_id, performance_id, start_date, end_date, content from poster", &[]).await?{
            let poster_id: i32 = row.get(0);
            let performance_id: i32 = row.get(1);
            let start_date: NaiveDate =row.get(2);
            let end_date: NaiveDate =row.get(3);
            let content: String = row.get(4);
            let a = Poster::new(poster_id.to_owned() , performance_id, start_date.to_string(),end_date.to_string(),content);
            println!("found play: {:?} ", a);
            poster.push(a);
        }
    
    Ok(poster)
}
// #[tokio::main]
pub  async fn stage() -> Result<Vec<Stage>, Box<dyn Error>>{
    

    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres dbname='Theatre' password=postgres", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    
    let mut stage: Vec<Stage> = vec![];
        for row in client.query("select stage_id, theater_id, capacity from stage", &[]).await?{
            let stage_id: i32 = row.get(0);
            let theater_id: i32 = row.get(1);
            let capacity: i32 =row.get(2);
            let a = Stage::new(stage_id ,theater_id, capacity);
            println!("found stage: {:?} ", a);
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
    


pub  fn  writer(statment: String, params: Vec<String>) -> Result<(),Box<dyn Error>>{
    let mut client =Client::connect("host=localhost user=postgres dbname='Theatre' password=postgres", NoTls)?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
        // tokio::spawn(async move {
        //     if let Err(e) = connection.await {
        //         eprintln!("connection error: {}", e);
        //     }
        // });
        let vec_of_to_sql: Vec<&(dyn ToSql + Sync)> = params.iter().map(|s| s as &(dyn ToSql + Sync)).collect();

        client.execute(
            &statment,
            &vec_of_to_sql,
        )?;
        Ok(())
}