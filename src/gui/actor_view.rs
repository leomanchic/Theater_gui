use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use egui_extras::{Column, TableBuilder};
use sea_orm::EntityTrait;

use crate::{
    dbworker::dbdriver,
    entity::{self, prelude::Actor},
};

//struct for View

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ActorView {
    pub view_enabled: Arc<AtomicBool>,
    pub content: Arc<Mutex<Vec<entity::actor::Model>>>,
}
impl ActorView {
    pub fn new() -> ActorView {
        ActorView {
            view_enabled: Arc::new(AtomicBool::new(false)),
            content: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

pub fn actor_view(
    ctx: &egui::Context,
    actr: &Vec<entity::actor::Model>,
    actors_viewport: &mut Arc<AtomicBool>,
) {
    let actors_viewport = Arc::clone(&actors_viewport);
    let actr = actr.clone();
    let name = Arc::new(Mutex::new(String::new()));
    let sn = Arc::new(Mutex::new(String::new()));
    let role = Arc::new(Mutex::new(String::new()));
    ctx.show_viewport_deferred(
        egui::ViewportId::from_hash_of("Actors"),
        egui::ViewportBuilder::default()
            .with_title("Actors")
            .with_inner_size([200.0, 100.0]),
        move |ctx, class| {
            assert!(
                class == egui::ViewportClass::Deferred,
                "This egui backend doesn't support multiple viewports"
            );

            egui::CentralPanel::default().show(ctx, |ui| {
                let name = Arc::clone(&name);
                let sn = Arc::clone(&sn);
                let role = Arc::clone(&role);
                egui::CollapsingHeader::new("Add actor").show(ui, |ui| {
                    egui::Grid::new("some_unique_id").show(ui, |ui| {
                        ui.label("Actor name:");
                        // let response2 =
                        ui.add_sized(
                            ui.min_size(),
                            egui::TextEdit::singleline(&mut *name.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Actor surname:");
                        // let response1 =
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *sn.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Actor`s role:");
                        let response = ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *role.lock().unwrap()),
                        );
                        if response.changed() {
                            // …
                        }
                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            // …available_size()
                            let name = Arc::clone(&name);
                            let sn: Arc<Mutex<String>> = Arc::clone(&sn);
                            let role = Arc::clone(&role);
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
                                dbdriver::actor_creator(&name, &sn, &role).await
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
                                ui.heading("actor_id");
                            });
                            header.col(|ui| {
                                ui.heading("name");
                            });
                            header.col(|ui| {
                                ui.heading("surname");
                            });
                            header.col(|ui| {
                                ui.heading("role");
                            });
                        })
                        .body(|mut body| {
                            for i in &*actr {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.actor_id.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(i.name.clone().unwrap_or_default());
                                    });
                                    row.col(|ui| {
                                        ui.label(i.surname.clone().unwrap_or_default());
                                    });
                                    row.col(|ui| {
                                        ui.label(i.role.clone().unwrap_or_default());
                                    });
                                });
                            }
                        });
                });
            });
            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent to close us.
                actors_viewport.store(false, Ordering::Relaxed);
            }
        },
    );
}
