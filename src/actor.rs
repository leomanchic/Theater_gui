use serde::de::Error;

use crate::dbdriver;

#[derive(Debug,Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Actor{
    actor_id: i32,
    name: String,
    surname: String,
    role: String,
}



impl Actor {
   pub  fn new(actor_id: i32 ,name: String , surname: String, role: String) -> Self {
        Self {
            actor_id,
            name,
            surname,
            role,
        }
    }
    pub fn get_id(&self) -> i32{
        self.actor_id
    }
    pub fn get_name(&self) -> String{
        self.name.clone()
    }
    pub fn get_surname(&self) -> String{
        self.surname.clone()
    }
    pub fn get_role(&self) -> String{
        self.role.clone()
    }
    pub  fn write(&self) {
       dbdriver::writer( "INSERT INTO actor (name, surname,role) VALUES ($1, $2, $3)".to_string(), vec![self.get_name(),self.get_surname(),self.get_role()]).unwrap();
    }
}