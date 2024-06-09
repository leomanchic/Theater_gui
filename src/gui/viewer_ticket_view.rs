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

pub struct ViewerTicketView {
    pub view_enabled: Arc<AtomicBool>,
    pub content: Arc<Mutex<Vec<entity::viewer_ticket::Model>>>,
}

impl ViewerTicketView {
    pub fn new() -> ViewerTicketView {
        ViewerTicketView {
            view_enabled: Arc::new(AtomicBool::new(false)),
            content: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

pub fn viewer_ticket_view(
    ctx: &egui::Context,
    vticket: &Vec<entity::viewer_ticket::Model>,
    vticket_viewport: &mut Arc<AtomicBool>,
) {
    let vticket_viewport = vticket_viewport.clone();
    let vticket = vticket.clone();
    let viewer_id = Arc::new(Mutex::new(String::new()));
    let ticket_id = Arc::new(Mutex::new(String::new()));
    let st_da_ti: Arc<Mutex<Option<chrono::NaiveDate>>> = Arc::new(Mutex::new(None));

    ctx.show_viewport_deferred(
        egui::ViewportId::from_hash_of("vticket"),
        egui::ViewportBuilder::default()
            .with_title("vticket")
            .with_inner_size([200.0, 100.0]),
        move |ctx, class| {
            assert!(
                class == egui::ViewportClass::Deferred,
                "This egui backend doesn't support multiple viewports"
            );

            egui::CentralPanel::default().show(ctx, |ui| {
                egui::CollapsingHeader::new("Add viewer for ticket").show(ui, |ui| {
                    egui::Grid::new("performance_unique_id").show(ui, |ui| {
                        // time::time_piker(ui);

                        ui.label("Viewer ID:");
                        // let response2 =
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *viewer_id.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Ticket ID:");
                        // let response1 =
                        let res = ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *ticket_id.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Time date");
                        let mut bindi = st_da_ti.lock().unwrap();
                        let mut date_ =
                            bindi.get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
                        ui.add(egui_extras::DatePickerButton::new(&mut date_));

                        if res.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            let runtime = tokio::runtime::Runtime::new();
                            runtime.unwrap().block_on(async {
                                dbdriver::viewer_ticket_creator(
                                    &*viewer_id.lock().unwrap(),
                                    &*ticket_id.lock().unwrap(),
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
                                ui.heading("viewer_viewer_id");
                            });
                            header.col(|ui| {
                                ui.heading("ticket_ticket_id");
                            });
                            header.col(|ui| {
                                ui.heading("bought_date");
                            });
                            header.col(|ui| {
                                ui.heading("vi_ti_id");
                            });
                        })
                        .body(|mut body| {
                            for i in &*vticket {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.viewer_viewer_id.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.ticket_ticket_id.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.bought_date.unwrap().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.vi_ti_id.to_string());
                                    });
                                });
                            }
                        });
                });
            });
            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent to close us.
                vticket_viewport.store(false, Ordering::Relaxed);
            }
        },
    );
}
