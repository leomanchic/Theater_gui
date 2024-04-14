#[derive(Debug)]
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
}