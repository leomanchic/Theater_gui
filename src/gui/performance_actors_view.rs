use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use egui_extras::{Column, TableBuilder};

use crate::{
    dbworker::dbdriver,
    entity::{self},
};

pub fn performance_actors_view(
    ctx: &egui::Context,
    performance_actors: &Vec<entity::performance_actors::Model>,
    performance_actors_viewport: &mut Arc<AtomicBool>,
) {
    let performance_actors_viewport = performance_actors_viewport.clone();
    let performance_actors = performance_actors.clone();
    ctx.show_viewport_deferred(
        egui::ViewportId::from_hash_of("performance_actors_rel"),
        egui::ViewportBuilder::default()
            .with_title("Actors_in_Per")
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
                                ui.heading("performance_performance_id");
                            });
                            header.col(|ui| {
                                ui.heading("actor_actor_id");
                            });
                            header.col(|ui| {
                                ui.heading("amount");
                            });
                            header.col(|ui| {
                                ui.heading("actors_perfor_id");
                            });
                        })
                        .body(|mut body| {
                            for i in &*performance_actors {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.performance_performance_id.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.actor_actor_id.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.amount.unwrap().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.actor_actor_id.to_string());
                                    });
                                });
                            }
                        });
                });
            });
            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent to close us.
                performance_actors_viewport.store(false, Ordering::Relaxed);
            }
        },
    );
}
