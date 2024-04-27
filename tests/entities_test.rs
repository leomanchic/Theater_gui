extern crate theater_gui;
use std::any::type_name_of_val;

use tokio;

#[test]
fn entities_creation() {
    #[tokio::main]
    async fn creation() {
        let m = theater_gui::actors().await.unwrap()[0].clone();
        assert_eq!("i32", type_name_of_val(&m.get_id()));
        assert_eq!("alloc::string::String", type_name_of_val(&m.get_name()));
        assert_eq!("alloc::string::String", type_name_of_val(&m.get_surname()));
        assert_eq!("alloc::string::String", type_name_of_val(&m.get_role()));
        assert_eq!("alloc::string::String", type_name_of_val(&m.get_role()));
    } 
    creation()
}
#[test]
fn env_test() {
    
}