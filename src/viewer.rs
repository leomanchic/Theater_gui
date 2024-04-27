use serde::de::Error;

use crate::dbdriver;

#[derive(Debug,Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Viewer{
    viewer_id: i32,
    name: String,
    email: String,
    phone: String,
}

impl Viewer   {
    pub  fn new(viewer_id: i32 ,name: String ,email: String, phone: String) -> Self {
        Self {
            viewer_id,
            name,
            email,
            phone,
        }
    }
    pub fn get_vid(&self) -> i32{
        self.viewer_id
    }
    pub fn get_name(&self) -> String{
        self.name.clone()
    }
    pub fn get_email(&self) -> String{
        self.email.clone()
    }
    pub fn get_phone(&self) -> String{
        self.phone.clone()
    }
    // pub  fn write(&self) {
    //    dbdriver::writer( "INSERT INTO actor (name, surname,role) VALUES ($1, $2, $3)".to_string(), vec![self.get_name(),self.get_surname(),self.get_role()]).unwrap();
    // }
}