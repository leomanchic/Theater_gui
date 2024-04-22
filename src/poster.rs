use serde::de::Error;

use crate::dbdriver;

#[derive(Debug,Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Poster{
    poster_id: i32,
    performance_id: i32,
    start_date: String,
    end_date: String,
    content: String,
}

impl Poster   {
    pub  fn new(poster_id: i32 ,performance_id: i32 , start_date: String, end_date: String, content: String) -> Self {
        Self {
            poster_id,
            performance_id,
            start_date,
            end_date,
            content,
        }
    }
    pub fn get_id(&self) -> i32{
        self.poster_id
    }
    pub fn get_pid(&self) -> i32{
        self.performance_id
    }
    pub fn get_sd(&self) -> String{
        self.start_date.clone()
    }
    pub fn get_content(&self) -> String{
        self.content.clone()
    }
    pub fn get_edate(&self) -> String{
        self.end_date.clone()
    }
    // pub  fn write(&self) {
    //    dbdriver::writer( "INSERT INTO actor (name, surname,role) VALUES ($1, $2, $3)".to_string(), vec![self.get_name(),self.get_surname(),self.get_role()]).unwrap();
    // }
}