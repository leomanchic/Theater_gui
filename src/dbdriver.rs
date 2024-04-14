use postgres::{Client, NoTls};
use std::error::Error;

use crate::Actor;


pub fn ger() -> String {
    "EE".to_owned()
}


pub  fn main() -> Result<String, Box<dyn Error>>{
        let mut client = Client::connect("host=localhost user=postgres dbname='Theatre' password=postgres", NoTls)?;
        
        // client.batch_execute("
        //     CREATE TABLE person (
        //         id      SERIAL PRIMARY KEY,
        //         name    TEXT NOT NULL,
        //         data    BYTEA
        //     )
        // ")?;

        // let name = "Ferris";
        // let data = None::<&[u8]>;
        // client.execute(
        //     "INSERT INTO person (name, data) VALUES ($1, $2)",
        //     &[&name, &data],
        // )?;
        let mut actors: Vec<Actor> = vec![];
        for row in client.query("SELECT actor_id, name, surname, role  FROM actor", &[])? {
            let id: i32 = row.get(0);
            let name: &str = row.get(1);
            let surname: &str = row.get(2);
            let role: &str = row.get(3);
            let a = Actor::new(id , name.to_owned(), surname.to_owned(), role.to_owned());
            println!("found person: {:?} ", a);
            actors.push(a);
        }
        Ok(format!("{:#?}",actors))
}