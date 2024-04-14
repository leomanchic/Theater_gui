#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;
mod dbdriver;
pub use dbdriver::main;
pub use dbdriver::ger;
mod actor;
pub use actor::Actor;