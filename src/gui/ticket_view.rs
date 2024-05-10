use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use egui_extras::{Column, TableBuilder};

use crate::{
    dbworker::dbdriver,
    entity::{self, theater, ticket},
};

pub fn ticket_view(
    ctx: &egui::Context,
    ticket: &Vec<entity::ticket::Model>,
    ticket_viewport: &mut Arc<AtomicBool>,
) {
    let ticket_viewport = ticket_viewport.clone();
    let ticket = ticket.clone();
    ctx.show_viewport_deferred(
        egui::ViewportId::from_hash_of("ticket"),
        egui::ViewportBuilder::default()
            .with_title("ticket")
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
                        .column(Column::auto().resizable(true))
                        .column(Column::auto().resizable(true))
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.heading("ticket_id");
                            });
                            header.col(|ui| {
                                ui.heading("performance_id");
                            });
                            header.col(|ui| {
                                ui.heading("seat_number");
                            });
                            header.col(|ui| {
                                ui.heading("date");
                            });
                            header.col(|ui| {
                                ui.heading("cost");
                            });
                            header.col(|ui| {
                                ui.heading("status");
                            });
                        })
                        .body(|mut body| {
                            for i in &*ticket {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.ticket_id.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.performance_id.unwrap_or_default().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.seat_number.unwrap_or_default().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.date.unwrap_or_default().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.cost.unwrap_or_default().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.status.clone().unwrap_or_default());
                                    });
                                });
                            }
                        });
                });
            });
            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent to close us.
                ticket_viewport.store(false, Ordering::Relaxed);
            }
        },
    );
}
