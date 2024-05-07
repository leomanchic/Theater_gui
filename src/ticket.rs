use chrono::NaiveDateTime;
use serde::de::Error;

use crate::dbdriver;

enum Status{
    Bought,
    Pending, 
    Rejected
}

#[derive(Debug,Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Ticket{
    ticket_id: Option<i32>,
    per_id: Option<i32>,
    seat_number: Option<i32>,
    date: Option<String>,
    cost: Option<i32>,
    status: Option<String>,
}

impl Ticket   {
    pub  fn new(ticket_id: Option<i32> ,per_id: Option<i32>, seat_number: Option<i32>, date: Option<String>, cost: Option<i32>, status: Option<String>) -> Self {
        Self {
            ticket_id,
            per_id,
            seat_number,
            date,
            cost,
            status,
        }
    }
    pub fn get_tid(&self) -> i32{
        self.ticket_id.unwrap_or_default()
    }
    pub fn get_pid(&self) -> i32{
        self.per_id.unwrap_or_default()
    }
    pub fn get_snum(&self) -> i32{
        self.seat_number.unwrap()
    }
    pub fn get_date(&self) -> String{
        self.date.clone().unwrap_or_default()
    }
    pub fn get_cost(&self) -> i32{
        self.cost.unwrap_or_default()
    }
    pub fn get_status(&self) -> String{
        self.status.clone().unwrap_or_default()
    }
    // pub  fn write(&self) {
    //    dbdriver::writer( "INSERT INTO actor (name, surname,role) VALUES ($1, $2, $3)".to_string(), vec![self.get_name(),self.get_surname(),self.get_role()]).unwrap();
    // }
}