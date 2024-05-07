use serde::de::Error;

use crate::dbdriver;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Theater {
    theater_id: Option<i32>,
    name: Option<String>,
    address: Option<String>,
    capacity: Option<i32>,
}

impl Theater {
    pub fn new(
        theater_id: Option<i32>,
        name: Option<String>,
        address: Option<String>,
        capacity: Option<i32>,
    ) -> Self {
        Self {
            theater_id,
            name,
            address,
            capacity,
        }
    }
    pub fn get_tid(&self) -> i32 {
        self.theater_id.unwrap_or_default()
    }
    pub fn get_name(&self) -> String {
        self.name.clone().unwrap_or_default()
    }
    pub fn get_address(&self) -> String {
        self.address.clone().unwrap_or_default()
    }
    pub fn get_capacity(&self) -> i32 {
        self.capacity.unwrap_or_default()
    }
    // pub  fn write(&self) {
    //    dbdriver::writer( "INSERT INTO actor (name, surname,role) VALUES ($1, $2, $3)".to_string(), vec![self.get_name(),self.get_surname(),self.get_role()]).unwrap();
    // }
}
