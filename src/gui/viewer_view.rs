use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use egui_extras::{Column, TableBuilder};

use crate::{
    dbworker::dbdriver,
    entity::{self, theater},
};

pub fn viewer_view(
    ctx: &egui::Context,
    viewer: &Vec<entity::viewer::Model>,
    viewer_viewport: &mut Arc<AtomicBool>,
) {
    let viewer_viewport = viewer_viewport.clone();
    let viewer = viewer.clone();
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
