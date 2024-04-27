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
    ticket_id: i32,
    per_id: i32,
    seat_number: i32,
    date: String,
    cost: i32,
    status: String,
}

impl Ticket   {
    pub  fn new(ticket_id: i32 ,per_id: i32, seat_number: i32, date: String, cost: i32, status: String) -> Self {
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
        self.ticket_id
    }
    pub fn get_pid(&self) -> i32{
        self.per_id
    }
    pub fn get_snum(&self) -> i32{
        self.seat_number
    }
    pub fn get_date(&self) -> String{
        self.date.clone()
    }
    pub fn get_cost(&self) -> i32{
        self.cost 
    }
    pub fn get_status(&self) -> String{
        self.status.clone()
    }
    // pub  fn write(&self) {
    //    dbdriver::writer( "INSERT INTO actor (name, surname,role) VALUES ($1, $2, $3)".to_string(), vec![self.get_name(),self.get_surname(),self.get_role()]).unwrap();
    // }
}