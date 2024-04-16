use serde::de::Error;

use crate::dbdriver;

#[derive(Debug,Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct PerformanceActors{
    performance_performance_id: i32,
    actor_actor_id: i32,
    amount: i32,
    actors_perfor_id: i32,
}

impl PerformanceActors   {
    pub  fn new(performance_performance_id: i32 ,actor_actor_id: i32 , amount: i32, actors_perfor_id: i32) -> Self {
        Self {
            performance_performance_id,
            actor_actor_id,
            amount,
            actors_perfor_id,
        }
    }
    pub fn get_performance_performance_id(&self) -> i32{
        self.performance_performance_id
    }
    pub fn get_actor_actor_id(&self) -> i32{
        self.actor_actor_id
    }
    pub fn get_amount(&self) -> i32{
        self.amount
    }
    pub fn get_actors_perfor_id(&self) -> i32{
        self.actors_perfor_id
    }
    // pub  fn write(&self) {
    //    dbdriver::writer( "INSERT INTO actor (name, surname,role) VALUES ($1, $2, $3)".to_string(), vec![self.get_name(),self.get_surname(),self.get_role()]).unwrap();
    // }
}