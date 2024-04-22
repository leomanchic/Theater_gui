use serde::de::Error;

use crate::dbdriver;

#[derive(Debug,Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ViewerTicket{
    viewer_viewer_id: i32,
    ticket_ticket_id: i32,
    bought_date: String,
    vi_ti_id: i32,
}

impl ViewerTicket   {
    pub  fn new(viewer_viewer_id: i32 ,ticket_ticket_id: i32 ,bought_date: String, vi_ti_id: i32) -> Self {
        Self {
            viewer_viewer_id,
            ticket_ticket_id,
            bought_date,
            vi_ti_id,
        }
    }
    pub fn get_vvid(&self) -> i32{
        self.viewer_viewer_id
    }
    pub fn get_ttid(&self) -> i32{
        self.ticket_ticket_id
    }
    pub fn get_bdate(&self) -> String{
        self.bought_date
    }
    pub fn get_vtid(&self) -> i32{
        self.vi_ti_id
    }
    // pub  fn write(&self) {
    //    dbdriver::writer( "INSERT INTO actor (name, surname,role) VALUES ($1, $2, $3)".to_string(), vec![self.get_name(),self.get_surname(),self.get_role()]).unwrap();
    // }
}