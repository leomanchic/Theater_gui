extern crate theater_gui;


use std::error::Error;

use tokio;

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