use super::dbdriver;

#[tokio::main]
pub async fn querry(atributes: Vec<String>) {
    let statment = String::from("f"); //format!("");
    dbdriver::writer(statment).await.unwrap();
}
