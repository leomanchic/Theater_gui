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

pub struct Stage_View {
    pub view_enabled: Arc<AtomicBool>,
    pub content: Arc<Mutex<Vec<entity::stage::Model>>>,
}

impl Stage_View {
    pub fn new() -> Stage_View {
        Stage_View {
            view_enabled: Arc::new(AtomicBool::new(false)),
            content: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
pub fn stage_view(
    ctx: &egui::Context,
    stage: &Vec<entity::stage::Model>,
    stage_viewport: &mut Arc<AtomicBool>,
) {
    let stage_viewport = stage_viewport.clone();
    let stage = stage.clone();
    ctx.show_viewport_deferred(
        egui::ViewportId::from_hash_of("Stage"),
        egui::ViewportBuilder::default()
            .with_title("Stage")
            .with_inner_size([200.0, 100.0]),
        move |ctx, class| {
            // assert!(
            //     class == egui::ViewportClass::Deferred,
            //     "This egui backend doesn't support multiple viewports"
            // );

            egui::CentralPanel::default().show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    TableBuilder::new(ui)
                        .column(Column::auto().resizable(true))
                        .column(Column::auto().resizable(true))
                        .column(Column::auto().resizable(true))
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.heading("stage_id");
                            });
                            header.col(|ui| {
                                ui.heading("theater_id");
                            });
                            header.col(|ui| {
                                ui.heading("capacity");
                            });
                        })
                        .body(|mut body| {
                            for i in &*stage {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.stage_id.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.theater_id.unwrap().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.capacity.unwrap().to_string());
                                    });
                                });
                            }
                        });
                });
            });
            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent to close us.
                stage_viewport.store(false, Ordering::Relaxed);
            }
        },
    );
}
