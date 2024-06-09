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

pub struct TicketView {
    pub view_enabled: Arc<AtomicBool>,
    pub content: Arc<Mutex<Vec<entity::ticket::Model>>>,
}
impl TicketView {
    pub fn new() -> TicketView {
        TicketView {
            view_enabled: Arc::new(AtomicBool::new(false)),
            content: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Enum {
    Bought,
    Pending,
}

pub fn ticket_view(
    ctx: &egui::Context,
    ticket: &Vec<entity::ticket::Model>,
    ticket_viewport: &mut Arc<AtomicBool>,
) {
    let ticket_viewport = ticket_viewport.clone();
    let ticket = ticket.clone();
    let per_id = Arc::new(Mutex::new(String::new()));
    let seat_num = Arc::new(Mutex::new(String::new()));
    let cost = Arc::new(Mutex::new(String::new()));
    let status = Mutex::new(Enum::Pending);
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
                egui::CollapsingHeader::new("Add Ticket").show(ui, |ui| {
                    egui::Grid::new("some_unique_id").show(ui, |ui| {
                        ui.label("Performance ID:");
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *per_id.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Seat number:");
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *seat_num.lock().unwrap()),
                        );
                        ui.end_row();

                        ui.label("Cost:");
                        let res = ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *cost.lock().unwrap()),
                        );
                        ui.end_row();
                        // ui.label("Status:");
                        // let response = ui.add_sized(
                        //     ui.available_size(),
                        //     egui::TextEdit::singleline(&mut *status.lock().unwrap()),
                        // );
                        egui::ComboBox::from_label("Status")
                            .selected_text(format!("{:?}", *status.lock().unwrap()))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut *status.lock().unwrap(),
                                    Enum::Bought,
                                    "Bought",
                                );
                                ui.selectable_value(
                                    &mut *status.lock().unwrap(),
                                    Enum::Pending,
                                    "Pending",
                                );
                            });
                        ui.end_row();
                        if res.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            let runtime = tokio::runtime::Runtime::new();
                            runtime.unwrap().block_on(async {
                                dbdriver::ticket_creator(
                                    &per_id,
                                    &seat_num,
                                    &cost,
                                    &format!("{:?}", status.lock().unwrap()),
                                )
                                .await
                                .unwrap()
                            });
                        }
                        // ui.end_row();
                    });
                });
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
                ticket_viewport.store(false, Ordering::Relaxed);
            }
        },
    );
}
