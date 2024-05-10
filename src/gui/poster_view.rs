use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use egui_extras::{Column, TableBuilder};

use crate::{
    dbworker::dbdriver,
    entity::{self},
};

pub fn poster_view(
    ctx: &egui::Context,
    poster: &Vec<entity::poster::Model>,
    poster_vieport: &mut Arc<AtomicBool>,
) {
    let poster_vieport = poster_vieport.clone();
    let poster = poster.clone();
    // let start_date = Arc::new(Mutex::new(String::new()));/
    let date_s: Arc<Mutex<Option<chrono::NaiveDate>>> = Arc::new(Mutex::new(None));
    let date_e: Arc<Mutex<Option<chrono::NaiveDate>>> = Arc::new(Mutex::new(None));
    let field_pi = Arc::new(Mutex::new(String::new()));

    ctx.show_viewport_deferred(
        egui::ViewportId::from_hash_of("Poster"),
        egui::ViewportBuilder::default()
            .with_title("Poster")
            .with_inner_size([200.0, 100.0]),
        move |ctx, class| {
            egui::CentralPanel::default().show(ctx, |ui| {
                //performance_id | start_date |  end_date  | content

                egui::CollapsingHeader::new("Add performance in poster").show(ui, |ui| {
                    egui::Grid::new("some_unique_id").show(ui, |ui| {
                        ui.label("performance_id");
                        ui.end_row();

                        ui.label("start_date_s");
                        // let mut date_s = date.lock().unwrap().get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
                        // let mut binding = date_s.lock().unwrap();
                        // let mut  date_s = binding.get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
                        // ui.add(egui_extras::DatePickerButton::new(&mut date_s));
                        ui.end_row();

                        ui.label("end_date");
                        let mut bindi = date_e.lock().unwrap();
                        let mut date_e =
                            bindi.get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
                        ui.add(egui_extras::DatePickerButton::new(&mut date_e));

                        ui.end_row();

                        ui.label("content");
                        ui.end_row();
                    });
                });

                egui::ScrollArea::vertical().show(ui, |ui| {
                    TableBuilder::new(ui)
                        .column(Column::auto().resizable(true))
                        .column(Column::auto().resizable(true))
                        .column(Column::auto().resizable(true))
                        .column(Column::auto().resizable(true))
                        .column(Column::auto().resizable(true))
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.heading("poster_id");
                            });
                            header.col(|ui| {
                                ui.heading("performance_id");
                            });
                            header.col(|ui| {
                                ui.heading("start_date");
                            });
                            header.col(|ui| {
                                ui.heading("end_date");
                            });
                            header.col(|ui| {
                                ui.heading("content");
                            });
                        })
                        .body(|mut body| {
                            for i in &*poster {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.poster_id.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.performance_id.unwrap_or_default().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.start_date.unwrap().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.end_date.unwrap().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(i.content.as_ref().unwrap());
                                    });
                                });
                            }
                        });
                });
            });
            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent to close us.
                poster_vieport.store(false, Ordering::Relaxed);
            }
        },
    );
}
