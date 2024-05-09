#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Viewer {
    viewer_id: Option<i32>,
    name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
}

impl Viewer {
    pub fn new(
        viewer_id: Option<i32>,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
    ) -> Self {
        Self {
            viewer_id,
            name,
            email,
            phone,
        }
    }
    pub fn get_vid(&self) -> i32 {
        self.viewer_id.unwrap()
    }
    pub fn get_name(&self) -> String {
        self.name.clone().unwrap()
    }
    pub fn get_email(&self) -> String {
        self.email.clone().unwrap()
    }
    pub fn get_phone(&self) -> String {
        self.phone.clone().unwrap()
    }
    // pub  fn write(&self) {
    //    dbdriver::writer( "INSERT INTO actor (name, surname,role) VALUES ($1, $2, $3)".to_string(), vec![self.get_name(),self.get_surname(),self.get_role()]).unwrap();
    // }
}
