use serde::de::Error;

use crate::dbdriver;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Play {
    play_id: Option<i32>,
    title: Option<String>,
    author: Option<String>,
    director: Option<String>,
}

impl Play {
    pub fn new(
        play_id: Option<i32>,
        title: Option<String>,
        author: Option<String>,
        director: Option<String>,
    ) -> Self {
        Self {
            play_id,
            title,
            author,
            director,
        }
    }
    pub fn get_play_id(&self) -> i32 {
        self.play_id.unwrap_or_default()
    }
    pub fn get_title(&self) -> String {
        self.title.clone().unwrap_or_default()
    }
    pub fn get_author(&self) -> String {
        self.author.clone().unwrap_or_default()
    }
    pub fn get_director(&self) -> String {
        self.director.clone().unwrap_or_default()
    }
    // pub  fn write(&self) {
    //    dbdriver::writer( "INSERT INTO actor (name, surname,role) VALUES ($1, $2, $3)".to_string(), vec![self.get_name(),self.get_surname(),self.get_role()]).unwrap();
    // }
}
