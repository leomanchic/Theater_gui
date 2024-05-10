use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use egui_extras::{Column, TableBuilder};

use crate::{
    dbworker::dbdriver,
    entity::{self, theater},
};

pub fn theater_view(
    ctx: &egui::Context,
    theater: &Vec<entity::theater::Model>,
    theater_viewport: &mut Arc<AtomicBool>,
) {
    let theater_viewport = theater_viewport.clone();
    let theater = theater.clone();
    ctx.show_viewport_deferred(
        egui::ViewportId::from_hash_of("theater"),
        egui::ViewportBuilder::default()
            .with_title("theater")
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
                                ui.heading("theater_id");
                            });
                            header.col(|ui| {
                                ui.heading("name");
                            });
                            header.col(|ui| {
                                ui.heading("address");
                            });
                            header.col(|ui| {
                                ui.heading("capacity");
                            });
                        })
                        .body(|mut body| {
                            for i in &*theater {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.theater_id.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(i.name.as_ref().unwrap());
                                    });
                                    row.col(|ui| {
                                        ui.label(i.address.as_ref().unwrap());
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
                theater_viewport.store(false, Ordering::Relaxed);
            }
        },
    );
}
