use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use crate::{
    dbworker::dbdriver,
    entity::{self},
};
use egui::Vec2;
use egui_extras::{Column, TableBuilder};
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Performance_Actors_View {
    pub view_enabled: Arc<AtomicBool>,
    pub content: Arc<Mutex<Vec<entity::performance_actors::Model>>>,
}
impl Performance_Actors_View {
    pub fn new() -> Performance_Actors_View {
        Performance_Actors_View {
            view_enabled: Arc::new(AtomicBool::new(false)),
            content: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

pub fn performance_actors_view(
    ctx: &egui::Context,
    performance_actors: &Vec<entity::performance_actors::Model>,
    performance_actors_viewport: &mut Arc<AtomicBool>,
) {
    //performance_actors_id, performance_id, amount;
    let performance_actors_viewport = performance_actors_viewport.clone();
    let performance_actors = performance_actors.clone();

    let per_act_q: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let actr_id: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let amount_q: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let notif = Arc::new(Mutex::new(String::new()));

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
                egui::CollapsingHeader::new("Add actors for performance").show(ui, |ui| {
                    egui::Grid::new("some_unique_id").show(ui, |ui| {
                        ui.label("Performance-id");
                        // let response2 =
                        ui.add_sized(
                            ui.min_size(),
                            egui::TextEdit::singleline(&mut *per_act_q.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Actor-id");
                        // let response1 =
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *actr_id.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Ammount");
                        let response = ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *amount_q.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Message");
                        let messege = ui.add_sized(
                            [200.0, 5.0],
                            egui::TextEdit::singleline(&mut *notif.lock().unwrap()),
                        );
                        ui.end_row();
                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            // let title: Arc<Mutex<String>> = Arc::clone(&title);
                            // let author: Arc<Mutex<String>> = Arc::clone(&author);
                            // let director = Arc::clone(&director);
                            let runtime = tokio::runtime::Runtime::new();
                            runtime.unwrap().block_on(async {
                                // dbdriver::actor_creator(&name, &sn, &role).await
                                // dbdriver::writer(format!{"insert into actor(name,surname,role) values ('{}', '{}', '{}');", *name.lock().unwrap(),*sn.lock().unwrap(),*role.lock().unwrap()}).await.unwrap();
                                *notif.lock().unwrap() = dbdriver::performance_actor_creator(
                                    &per_act_q, &actr_id, &amount_q,
                                )
                                .await
                                .unwrap();
                            });
                        }
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
