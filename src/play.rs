use serde::de::Error;

use crate::dbdriver;

#[derive(Debug,Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Play{
    play_id: i32,
    title: String,
    author: String,
    director: String,
}

impl Play   {
    pub  fn new(play_id: i32 ,title: String , author: String, director: String) -> Self {
        Self {
            play_id,
            title,
            author,
            director,
        }
    }
    pub fn get_play_id(&self) -> i32{
        self.play_id
    }
    pub fn get_title(&self) -> String{
        self.title.clone()
    }
    pub fn get_author(&self) -> String{
        self.author.clone()
    }
    pub fn get_director(&self) -> String{
        self.director.clone()
    }
    // pub  fn write(&self) {
    //    dbdriver::writer( "INSERT INTO actor (name, surname,role) VALUES ($1, $2, $3)".to_string(), vec![self.get_name(),self.get_surname(),self.get_role()]).unwrap();
    // }
}