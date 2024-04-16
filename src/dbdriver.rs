use chrono::naive::NaiveDateTime;
use postgres::types::ToSql;
use postgres::{Client, NoTls};
use std::error::Error;

use crate::performance::Performance;
use crate::Actor;
use crate::PerformanceActors;

pub  fn actors() -> Result<Vec<Actor>, Box<dyn Error>>{
 
    let mut  client = Client::connect("host=localhost user=postgres dbname='Theatre' password=postgres", NoTls)?;

    
    let mut actors: Vec<Actor> = vec![];
        for row in client.query("SELECT actor_id, name, surname, role  FROM actor", &[])?{
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


pub  fn performance() -> Result<Vec<Performance>, Box<dyn Error>>{
 
    let mut  client = Client::connect("host=localhost user=postgres dbname='Theatre' password=postgres", NoTls)?;

    
    let mut performance: Vec<Performance> = vec![];
        for row in client.query("SELECT performance_id, play_id, stage_id, start_datetime  FROM performance", &[])?{
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


pub  fn performance_actors() -> Result<Vec<PerformanceActors>, Box<dyn Error>>{
 
    let mut  client = Client::connect("host=localhost user=postgres dbname='Theatre' password=postgres", NoTls)?;

    
    let mut performance_a: Vec<PerformanceActors> = vec![];
        for row in client.query("SELECT performance_performance_id, actor_actor_id, amount, actors_perfor_id  FROM performance_actors", &[])?{
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