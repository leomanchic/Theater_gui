use serde::de::Error;

use crate::dbdriver;

#[derive(Debug,Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Performance{
    performance_id: i32,
    play_id: i32,
    stage_id: i32,
    start_datetime: String,
}

impl Performance   {
    pub  fn new(performance_id: i32 ,play_id: i32 , stage_id: i32, start_datetime: String) -> Self {
        Self {
            performance_id,
            play_id,
            stage_id,
            start_datetime,
        }
    }
    pub fn get_id(&self) -> i32{
        self.performance_id
    }
    pub fn get_plid(&self) -> i32{
        self.play_id
    }
    pub fn get_sid(&self) -> i32{
        self.stage_id
    }
    pub fn get_date(&self) -> String{
        self.start_datetime.clone()
    }
    // pub  fn write(&self) {
    //    dbdriver::writer( "INSERT INTO actor (name, surname,role) VALUES ($1, $2, $3)".to_string(), vec![self.get_name(),self.get_surname(),self.get_role()]).unwrap();
    // }
}