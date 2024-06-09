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

pub struct ViewerView {
    pub view_enabled: Arc<AtomicBool>,
    pub content: Arc<Mutex<Vec<entity::viewer::Model>>>,
}

impl ViewerView {
    pub fn new() -> ViewerView {
        ViewerView {
            view_enabled: Arc::new(AtomicBool::new(false)),
            content: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

pub fn viewer_view(
    ctx: &egui::Context,
    viewer: &Vec<entity::viewer::Model>,
    viewer_viewport: &mut Arc<AtomicBool>,
) {
    let viewer_viewport = viewer_viewport.clone();
    let viewer = viewer.clone();
    let name = Arc::new(Mutex::new(String::new()));
    let email = Arc::new(Mutex::new(String::new()));
    let phone = Arc::new(Mutex::new(String::new()));
    ctx.show_viewport_deferred(
        egui::ViewportId::from_hash_of("viewer"),
        egui::ViewportBuilder::default()
            .with_title("viewer")
            .with_inner_size([200.0, 100.0]),
        move |ctx, class| {
            assert!(
                class == egui::ViewportClass::Deferred,
                "This egui backend doesn't support multiple viewports"
            );

            egui::CentralPanel::default().show(ctx, |ui| {
                egui::CollapsingHeader::new("Add Viewer").show(ui, |ui| {
                    egui::Grid::new("viewer_unique_id").show(ui, |ui| {
                        ui.label("Name:");
                        // let response2 =
                        ui.add_sized(
                            ui.min_size(),
                            egui::TextEdit::singleline(&mut *name.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Email:");
                        // let response1 =
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *email.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Phone");
                        let response = ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *phone.lock().unwrap()),
                        );

                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            let runtime = tokio::runtime::Runtime::new();
                            runtime.unwrap().block_on(async {
                                dbdriver::viewer_creator(
                                    &*name.lock().unwrap(),
                                    &*email.lock().unwrap(),
                                    &phone.lock().unwrap(),
                                )
                                .await
                            });
                        }
                        ui.end_row();
                    });
                });

                egui::ScrollArea::vertical().show(ui, |ui| {
                    TableBuilder::new(ui)
                        .columns(Column::auto().resizable(true), 4usize)
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.heading("viewer_id");
                            });
                            header.col(|ui| {
                                ui.heading("name");
                            });
                            header.col(|ui| {
                                ui.heading("email");
                            });
                            header.col(|ui| {
                                ui.heading("phone");
                            });
                        })
                        .body(|mut body| {
                            for i in &*viewer {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.viewer_id.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(i.name.as_ref().unwrap());
                                    });
                                    row.col(|ui| {
                                        ui.label(i.email.as_ref().unwrap());
                                    });
                                    row.col(|ui| {
                                        ui.label(i.phone.as_ref().unwrap());
                                    });
                                });
                            }
                        });
                });
            });
            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent to close us.
                viewer_viewport.store(false, Ordering::Relaxed);
            }
        },
    );
}
