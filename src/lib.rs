#![warn(clippy::all, rust_2018_idioms)]

//App appirience
mod gui;
pub use gui::app::TemplateApp;
//Working with database
mod dbworker;
pub use dbworker::dbdriver::writer;
pub use dbworker::querry;
mod entity;
