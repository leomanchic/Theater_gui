#![warn(clippy::all, rust_2018_idioms)]

//App appirience
mod gui;
pub use gui::app::TemplateApp;
//Working with database
mod dbworker;
pub use dbworker::dbdriver::actors;
pub use dbworker::dbdriver::performance;
pub use dbworker::dbdriver::performance_actors;
pub use dbworker::dbdriver::writer;
pub use dbworker::querry;

//Database entities
mod entities;
pub use entities::actor::Actor;
pub use entities::performance::Performance;
pub use entities::performance_actors::PerformanceActors;
pub use entities::play::Play;
pub use entities::poster::Poster;
pub use entities::stage::Stage;
pub use entities::theater::Theater;
pub use entities::ticket::Ticket;
pub use entities::viewer::Viewer;
pub use entities::viewer_ticket::ViewerTicket;


mod entity;
