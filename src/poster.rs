use serde::de::Error;

use crate::dbdriver;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Poster {
    poster_id: Option<i32>,
    performance_id: Option<i32>,
    start_date: Option<String>,
    end_date: Option<String>,
    content: Option<String>,
}

impl Poster {
    pub fn new(
        poster_id: Option<i32>,
        performance_id: Option<i32>,
        start_date: Option<String>,
        end_date: Option<String>,
        content: Option<String>,
    ) -> Self {
        Self {
            poster_id,
            performance_id,
            start_date,
            end_date,
            content,
        }
    }
    pub fn get_id(&self) -> i32 {
        self.poster_id.unwrap_or_default()
    }
    pub fn get_pid(&self) -> i32 {
        self.performance_id.unwrap_or_default()
    }
    pub fn get_sd(&self) -> String {
        self.start_date.clone().unwrap_or_default()
    }
    pub fn get_content(&self) -> String {
        self.content.clone().unwrap_or_default()
    }
    pub fn get_edate(&self) -> String {
        self.end_date.clone().unwrap_or_default()
    }
    // pub  fn write(&self) {
    //    dbdriver::writer( "INSERT INTO actor (name, surname,role) VALUES ($1, $2, $3)".to_string(), vec![self.get_name(),self.get_surname(),self.get_role()]).unwrap();
    // }
}
