use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use egui_extras::{Column, TableBuilder};

use crate::{
    dbworker::dbdriver,
    entity::{self, theater, ticket},
};
#[derive(serde::Deserialize, serde::Serialize)]

pub struct Ticket_View {
    pub view_enabled: Arc<AtomicBool>,
    pub content: Arc<Mutex<Vec<entity::ticket::Model>>>,
}
impl Ticket_View {
    pub fn new() -> Ticket_View {
        Ticket_View {
            view_enabled: Arc::new(AtomicBool::new(false)),
            content: Arc::new(Mutex::new(Vec::new())),
        }
    }
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
    let date: Arc<Mutex<Option<chrono::NaiveDate>>> = Arc::new(Mutex::new(None));
    let cost = Arc::new(Mutex::new(String::new()));
    let status = Arc::new(Mutex::new(String::new()));
    // ticket_id | performance_id | seat_number |    date    | cost | status
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
                egui::CollapsingHeader::new("Add performance").show(ui, |ui| {
                    egui::Grid::new("some_unique_id").show(ui, |ui| {
                        ui.label("Performance ID:");
                        // let response2 =
                        ui.add_sized(
                            ui.min_size(),
                            egui::TextEdit::singleline(&mut *per_id.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Seat number:");
                        // let response1 =
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *seat_num.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Performance date");
                        let mut bindi = date.lock().unwrap();
                        let mut date_ =
                            bindi.get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
                        ui.add(egui_extras::DatePickerButton::new(&mut date_));
                        ui.end_row();
                        ui.label("Cost:");
                        // let response1 =
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *cost.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Status:");
                        // let response1 =
                        let response = ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *status.lock().unwrap()),
                        );
                        ui.end_row();

                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            let per_id = Arc::clone(&per_id);
                            let seat_num = Arc::clone(&seat_num);
                            let date = Arc::new(date_);
                            let cost = Arc::clone(&cost);
                            let status = Arc::clone(&status);
                            let runtime = tokio::runtime::Runtime::new();
                            runtime.unwrap().block_on(async {
                                dbdriver::ticket_creator(&per_id, &seat_num, date, &cost, &status)
                                    .await
                                    .unwrap()
                                // dbdriver::writer(format!{"insert into actor(name,surname,role) values ('{}', '{}', '{}');", *name.lock().unwrap(),*sn.lock().unwrap(),*role.lock().unwrap()}).await.unwrap();
                            });
                            // println!("{:?} ", date_);
                        }
                        ui.end_row();
                        // ui.label("Actor surname:");
                        // ui.label("Actor`s role:");
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
                // Tell parent to close us.
                ticket_viewport.store(false, Ordering::Relaxed);
            }
        },
    );
}
