use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use egui_extras::{Column, TableBuilder};

use crate::{
    dbworker::dbdriver,
    entity::{self},
};
#[derive(serde::Deserialize, serde::Serialize)]

pub struct Plays_View {
    pub view_enabled: Arc<AtomicBool>,
    pub content: Arc<Mutex<Vec<entity::play::Model>>>,
}

impl Plays_View {
    pub fn new() -> Plays_View {
        Plays_View {
            view_enabled: Arc::new(AtomicBool::new(false)),
            content: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

pub fn play_view(
    ctx: &egui::Context,
    plays: &Vec<entity::play::Model>,
    play_viewport: &mut Arc<AtomicBool>,
) {
    let play_viewport = play_viewport.clone();
    let plays = plays.clone();
    ctx.show_viewport_deferred(
        egui::ViewportId::from_hash_of("Play"),
        egui::ViewportBuilder::default()
            .with_title("Plays")
            .with_inner_size([200.0, 100.0]),
        move |ctx, class| {
            assert!(
                class == egui::ViewportClass::Deferred,
                "This egui backend doesn't support multiple viewports"
            );

            egui::CentralPanel::default().show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    TableBuilder::new(ui)
                        .column(Column::auto().resizable(true))
                        .column(Column::auto().resizable(true))
                        .column(Column::auto().resizable(true))
                        .column(Column::auto().resizable(true))
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.heading("play_id");
                            });
                            header.col(|ui| {
                                ui.heading("title");
                            });
                            header.col(|ui| {
                                ui.heading("author");
                            });
                            header.col(|ui| {
                                ui.heading("director");
                            });
                        })
                        .body(|mut body| {
                            for i in &*plays {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.play_id.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(i.title.as_ref().unwrap());
                                    });
                                    row.col(|ui| {
                                        ui.label(i.author.as_ref().unwrap());
                                    });
                                    row.col(|ui| {
                                        ui.label(i.director.as_ref().unwrap());
                                    });
                                });
                            }
                        });
                });
            });
            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent to close us.
                play_viewport.store(false, Ordering::Relaxed);
            }
        },
    );
}
