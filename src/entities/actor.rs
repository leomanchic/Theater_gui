#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Actor {
    actor_id: Option<i32>,
    name: Option<String>,
    surname: Option<String>,
    role: Option<String>,
}

impl Actor {
    pub fn new(
        actor_id: Option<i32>,
        name: Option<String>,
        surname: Option<String>,
        role: Option<String>,
    ) -> Self {
        Self {
            actor_id,
            name,
            surname,
            role,
        }
    }
    pub fn get_id(&self) -> i32 {
        self.actor_id.unwrap()
    }
    pub fn get_name(&self) -> String {
        self.name.clone().unwrap()
    }
    pub fn get_surname(&self) -> String {
        self.surname.clone().unwrap()
    }
    pub fn get_role(&self) -> String {
        self.role.clone().unwrap()
    }
    // pub  fn write(&self) {
    //    dbdriver::writer( "INSERT INTO actor (name, surname,role) VALUES ($1, $2, $3)".to_string(), vec![self.get_name(),self.get_surname(),self.get_role()]).unwrap();
    // }
    pub fn drop(mut self) {
        println!("droppint");
    }
}
