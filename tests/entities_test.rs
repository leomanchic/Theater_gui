extern crate theater_gui;
use std::{any::type_name_of_val, error::Error};

use tokio;

#[test]
fn entities_selection() {
    #[tokio::main]
    async fn select() {
        let m = theater_gui::actors().await.unwrap()[0].clone();
        assert_eq!("i32", type_name_of_val(&m.get_id()));
        assert_eq!("alloc::string::String", type_name_of_val(&m.get_name()));
        assert_eq!("alloc::string::String", type_name_of_val(&m.get_surname()));
        assert_eq!("alloc::string::String", type_name_of_val(&m.get_role()));
        assert_eq!("alloc::string::String", type_name_of_val(&m.get_role()));
    } 
    select()
}
#[test]
fn entities_insertion() -> Result<(), Box<dyn Error>>{
    #[tokio::main]
    async fn creation() -> Result<(), Box<dyn Error>>{
        let m = "insert into stage  (theater_id,capacity) values (1,230),(1,250)";
        theater_gui::writer(m.to_owned()).await?;
        Ok(())
    } 
    creation()
}