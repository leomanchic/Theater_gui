use serde::de::Error;

use crate::dbdriver;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Stage {
    stage_id: Option<i32>,
    theater_id: Option<i32>,
    capacity: Option<i32>,
}

impl Stage {
    pub fn new(stage_id: Option<i32>, theater_id: Option<i32>, capacity: Option<i32>) -> Self {
        Self {
            stage_id,
            theater_id,
            capacity,
        }
    }
    pub fn get_sid(&self) -> i32 {
        self.stage_id.unwrap_or_default()
    }
    pub fn get_tid(&self) -> i32 {
        self.theater_id.unwrap_or_default()
    }
    pub fn get_capacity(&self) -> i32 {
        self.capacity.unwrap_or_default()   
    }
    // pub  fn write(&self) {
    //    dbdriver::writer( "INSERT INTO actor (name, surname,role) VALUES ($1, $2, $3)".to_string(), vec![self.get_name(),self.get_surname(),self.get_role()]).unwrap();
    // }
}
