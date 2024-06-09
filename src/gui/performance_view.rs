use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use chrono::NaiveDateTime;
use egui_extras::{Column, TableBuilder};

use crate::{dbworker::dbdriver, entity};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PerformanceView {
    pub view_enabled: Arc<AtomicBool>,
    pub content: Arc<Mutex<Vec<entity::performance::Model>>>,
}
impl PerformanceView {
    pub fn new() -> PerformanceView {
        PerformanceView {
            view_enabled: Arc::new(AtomicBool::new(false)),
            content: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

pub fn performance_view(
    ctx: &egui::Context,
    performance: &Vec<entity::performance::Model>,
    performance_viewport: &mut Arc<AtomicBool>,
) {
    let performance_viewport = performance_viewport.clone();
    let performance = performance.clone();
    let pl_id: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let st_id: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let st_da_ti: Arc<Mutex<Option<chrono::NaiveDate>>> = Arc::new(Mutex::new(None));
    let time = Arc::new(Mutex::new(0f32));
    ctx.show_viewport_deferred(
        egui::ViewportId::from_hash_of("performance"),
        egui::ViewportBuilder::default()
            .with_title("Performances")
            .with_inner_size([200.0, 100.0]),
        move |ctx, class| {
            assert!(
                class == egui::ViewportClass::Deferred,
                "This egui backend doesn't support multiple viewports"
            );

            egui::CentralPanel::default().show(ctx, |ui| {
                egui::CollapsingHeader::new("Add performance").show(ui, |ui| {
                    egui::Grid::new("performance_unique_id").show(ui, |ui| {
                        // time::time_piker(ui);

                        ui.label("Play ID:");
                        // let response2 =
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *pl_id.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Stage ID:");
                        // let response1 =
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *st_id.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Performance date");
                        let mut bindi = st_da_ti.lock().unwrap();
                        let mut date_ =
                            bindi.get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
                        ui.add(egui_extras::DatePickerButton::new(&mut date_));
                        let res: egui::Response = ui.add(
                            egui::DragValue::new(&mut *time.lock().unwrap())
                                .clamp_range(0..=((60 * 60 * 24) - 1))
                                .custom_formatter(|n, _| {
                                    let n = n as i32;
                                    let hours = n / (60 * 60);
                                    let mins = (n / 60) % 60;
                                    let secs = n % 60;

                                    // println!("{} {} {}", hours, mins, secs);
                                    format!("{hours:02}:{mins:02}:{secs:02}")
                                })
                                .custom_parser(|s| {
                                    let parts: Vec<&str> = s.split(':').collect();
                                    if parts.len() == 3 {
                                        parts[0]
                                            .parse::<i32>()
                                            .and_then(|h| {
                                                parts[1].parse::<i32>().and_then(|m| {
                                                    parts[2].parse::<i32>().map(|s| {
                                                        // println!{"{}, {}, {}",h,m,s};
                                                        ((h * 60 * 60) + (m * 60) + s) as f64
                                                    })
                                                })
                                            })
                                            .ok()
                                    } else {
                                        None
                                    }
                                }),
                        );
                        if res.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            let time = *time.lock().unwrap();
                            let m = bindi
                                .unwrap()
                                .and_hms_opt(
                                    (time / (60f32 * 60f32)) as u32,
                                    ((time / 60f32) % 60f32) as u32,
                                    (time % 60f32) as u32,
                                )
                                .unwrap();

                            let runtime = tokio::runtime::Runtime::new();
                            runtime.unwrap().block_on(async {
                                dbdriver::performance_creator(&pl_id, &st_id, Some(m)).await
                            });
                        }
                        ui.end_row();
                    });
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    TableBuilder::new(ui)
                        .column(Column::auto().resizable(true))
                        .column(Column::auto().resizable(true))
                        .column(Column::auto().resizable(true))
                        .column(Column::auto().resizable(true))
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.heading("performance_id");
                            });
                            header.col(|ui| {
                                ui.heading("play_id");
                            });
                            header.col(|ui| {
                                ui.heading("stage_id");
                            });
                            header.col(|ui| {
                                ui.heading("date");
                            });
                        })
                        .body(|mut body| {
                            for i in &*performance {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.performance_id.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.play_id.unwrap_or_default().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.stage_id.unwrap_or_default().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.start_datetime.unwrap_or_default().to_string());
                                    });
                                });
                            }
                        });
                });
            });
            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent to close us.
                performance_viewport.store(false, Ordering::Relaxed);
            }
        },
    );
}
