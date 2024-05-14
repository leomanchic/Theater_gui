use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use egui_extras::{Column, TableBuilder};

use crate::{
    dbworker::dbdriver,
    entity::{self, theater},
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Theater_View {
    pub view_enabled: Arc<AtomicBool>,
    pub content: Arc<Mutex<Vec<entity::theater::Model>>>,
}

impl Theater_View {
    pub fn new() -> Theater_View {
        Theater_View {
            view_enabled: Arc::new(AtomicBool::new(false)),
            content: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

pub fn theater_view(
    ctx: &egui::Context,
    theater: &Vec<entity::theater::Model>,
    theater_viewport: &mut Arc<AtomicBool>,
) {
    let theater_viewport = theater_viewport.clone();
    let theater = theater.clone();
    let name = Arc::new(Mutex::new(String::new()));
    let address = Arc::new(Mutex::new(String::new()));
    let capacity = Arc::new(Mutex::new(String::new()));
    ctx.show_viewport_deferred(
        egui::ViewportId::from_hash_of("theater"),
        egui::ViewportBuilder::default()
            .with_title("theater")
            .with_inner_size([200.0, 100.0]),
        move |ctx, class| {
            assert!(
                class == egui::ViewportClass::Deferred,
                "This egui backend doesn't support multiple viewports"
            );

            egui::CentralPanel::default().show(ctx, |ui| {
                egui::CollapsingHeader::new("Add Theater").show(ui, |ui| {
                    egui::Grid::new("some_unique_id").show(ui, |ui| {
                        ui.label("Name:");
                        // let response2 =
                        ui.add_sized(
                            ui.min_size(),
                            egui::TextEdit::singleline(&mut *name.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Adress:");
                        // let response1 =
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *address.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Capacity");
                        let response = ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *capacity.lock().unwrap()),
                        );

                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
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
                                dbdriver::theater_creator(
                                    &*name.lock().unwrap(),
                                    &*address.lock().unwrap(),
                                    &capacity.lock().unwrap(),
                                )
                                .await
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
                                ui.heading("theater_id");
                            });
                            header.col(|ui| {
                                ui.heading("name");
                            });
                            header.col(|ui| {
                                ui.heading("address");
                            });
                            header.col(|ui| {
                                ui.heading("capacity");
                            });
                        })
                        .body(|mut body| {
                            for i in &*theater {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.theater_id.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(i.name.as_ref().unwrap());
                                    });
                                    row.col(|ui| {
                                        ui.label(i.address.as_ref().unwrap());
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
                theater_viewport.store(false, Ordering::Relaxed);
            }
        },
    );
}
