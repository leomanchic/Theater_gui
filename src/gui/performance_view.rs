use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use egui_extras::{Column, TableBuilder};

use crate::{dbworker::dbdriver, entity, gui::time};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Performance_View {
    pub view_enabled: Arc<AtomicBool>,
    pub content: Arc<Mutex<Vec<entity::performance::Model>>>,
}
impl Performance_View {
    pub fn new() -> Performance_View {
        Performance_View {
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
    // let pl_id: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let pl_id: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let st_id: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let st_da_ti: Arc<Mutex<Option<chrono::NaiveDate>>> = Arc::new(Mutex::new(None));

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
                let pl_id = Arc::clone(&pl_id);
                let st_id = Arc::clone(&st_id);
                let st_da_ti = Arc::clone(&st_da_ti);

                egui::CollapsingHeader::new("Add performance").show(ui, |ui| {
                    egui::Grid::new("some_unique_id").show(ui, |ui| {
                        time::time_piker(ui);
                        ui.label("Play ID:");
                        // let response2 =
                        ui.add_sized(
                            ui.min_size(),
                            egui::TextEdit::singleline(&mut *pl_id.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Stage ID:");
                        // let response1 =
                        let response = ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *st_id.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Performance date");
                        let mut bindi = st_da_ti.lock().unwrap();
                        let mut date_ =
                            bindi.get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
                        ui.add(egui_extras::DatePickerButton::new(&mut date_));

                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            // …available_size()
                            let pl_id: Arc<Mutex<String>> = Arc::clone(&pl_id);
                            let st_id: Arc<Mutex<String>> = Arc::clone(&st_id);
                            let st_da_ti = Arc::clone(&st_da_ti);
                            let runtime = tokio::runtime::Runtime::new();
                            runtime.unwrap().block_on(async {
                                // let act = entity::actor::ActiveModel {
                                //     name: sea_orm::ActiveValue::Set(Some(
                                //         name.lock().unwrap().to_string(),
                                //     )),
                                //     surname: sea_orm::ActiveValue::Set(Some(
                                //         sn.lock().unwrap().to_string(),
                                //     )),
                                //     role: sea_orm::ActiveValue::Set(Some(
                                //         role.lock().unwrap().to_string(),
                                //     )),
                                //     ..Default::default()
                                // };
                                // ActorS::insert(act).exec(&db).await.unwrap();
                                dbdriver::performance_creator(&pl_id, &st_id, &st_da_ti).await
                                // dbdriver::writer(format!{"insert into actor(name,surname,role) values ('{}', '{}', '{}');", *name.lock().unwrap(),*sn.lock().unwrap(),*role.lock().unwrap()}).await.unwrap();
                            });
                            // println!("{:?} {:?} {:? }", role, sn, name);
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
