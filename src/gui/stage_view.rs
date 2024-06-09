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

pub struct Stage_View {
    pub view_enabled: Arc<AtomicBool>,
    pub content: Arc<Mutex<Vec<entity::stage::Model>>>,
}

impl Stage_View {
    pub fn new() -> Stage_View {
        Stage_View {
            view_enabled: Arc::new(AtomicBool::new(false)),
            content: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
pub fn stage_view(
    ctx: &egui::Context,
    stage: &Vec<entity::stage::Model>,
    stage_viewport: &mut Arc<AtomicBool>,
) {
    let stage_viewport = stage_viewport.clone();
    let stage = stage.clone();
    let theater_q: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let capacity: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    ctx.show_viewport_deferred(
        egui::ViewportId::from_hash_of("Stage"),
        egui::ViewportBuilder::default()
            .with_title("Stage")
            .with_inner_size([200.0, 100.0]),
        move |ctx, class| {
            // assert!(
            //     class == egui::ViewportClass::Deferred,
            //     "This egui backend doesn't support multiple viewports"
            // );

            egui::CentralPanel::default().show(ctx, |ui| {
                egui::CollapsingHeader::new("Add Stage").show(ui, |ui| {
                    egui::Grid::new("some_unique_id").show(ui, |ui| {
                        ui.label("Theater_id:");
                        // let response2 =
                        ui.add_sized(
                            ui.min_size(),
                            egui::TextEdit::singleline(&mut *theater_q.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Capacity:");
                        // let response1 =
                        let responce = ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *capacity.lock().unwrap()),
                        );
                        ui.end_row();

                        if responce.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            // let title: Arc<Mutex<String>> = Arc::clone(&title);
                            // let author: Arc<Mutex<String>> = Arc::clone(&author);
                            // let director = Arc::clone(&director);
                            let runtime = tokio::runtime::Runtime::new();
                            runtime.unwrap().block_on(async {
                                // dbdriver::actor_creator(&name, &sn, &role).await
                                // todo!("play_creator()")
                                // todo!("stage_creator()")
                                dbdriver::stage_creator(&theater_q, &capacity).await;
                                // dbdriver::writer(format!{"insert into actor(name,surname,role) values ('{}', '{}', '{}');", *name.lock().unwrap(),*sn.lock().unwrap(),*role.lock().unwrap()}).await.unwrap();
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
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.heading("stage_id");
                            });
                            header.col(|ui| {
                                ui.heading("theater_id");
                            });
                            header.col(|ui| {
                                ui.heading("capacity");
                            });
                        })
                        .body(|mut body| {
                            for i in &*stage {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.stage_id.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.theater_id.unwrap().to_string());
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
                stage_viewport.store(false, Ordering::Relaxed);
            }
        },
    );
}