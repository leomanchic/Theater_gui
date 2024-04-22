#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod performance;
pub use app::TemplateApp;
mod dbdriver;
pub use dbdriver::actors;
pub use dbdriver::performance;
pub use dbdriver::performance_actors;
pub use performance::Performance;
mod actor;
pub use actor::Actor;
mod performance_actors;
pub use performance_actors::PerformanceActors;
mod play;
mod poster;
mod stage;