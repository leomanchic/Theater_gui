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

pub struct Plays_View {
    pub view_enabled: Arc<AtomicBool>,
    pub content: Arc<Mutex<Vec<entity::play::Model>>>,
}

impl Plays_View {
    pub fn new() -> Plays_View {
        Plays_View {
            view_enabled: Arc::new(AtomicBool::new(false)),
            content: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

pub fn play_view(
    ctx: &egui::Context,
    plays: &Vec<entity::play::Model>,
    play_viewport: &mut Arc<AtomicBool>,
) {
    let play_viewport = play_viewport.clone();
    let plays = plays.clone();
    let title: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let author: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let director: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    ctx.show_viewport_deferred(
        egui::ViewportId::from_hash_of("Play"),
        egui::ViewportBuilder::default()
            .with_title("Plays")
            .with_inner_size([200.0, 100.0]),
        move |ctx, class| {
            assert!(
                class == egui::ViewportClass::Deferred,
                "This egui backend doesn't support multiple viewports"
            );

            egui::CentralPanel::default().show(ctx, |ui| {
                egui::CollapsingHeader::new("Add Play").show(ui, |ui| {
                    egui::Grid::new("some_unique_id").show(ui, |ui| {
                        ui.label("Title");
                        // let response2 =
                        ui.add_sized(
                            ui.min_size(),
                            egui::TextEdit::singleline(&mut *title.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Author");
                        // let response1 =
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *author.lock().unwrap()),
                        );
                        ui.end_row();
                        ui.label("Director");
                        let response = ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut *director.lock().unwrap()),
                        );

                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            // let title: Arc<Mutex<String>> = Arc::clone(&title);
                            // let author: Arc<Mutex<String>> = Arc::clone(&author);
                            // let director = Arc::clone(&director);
                            let runtime = tokio::runtime::Runtime::new();
                            runtime.unwrap().block_on(async {
                                // dbdriver::actor_creator(&name, &sn, &role).await
                                // todo!("play_creator()")
                                dbdriver::play_creator(&title, &author, &director)
                                    .await
                                    .unwrap();
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
                        .column(Column::auto().resizable(true))
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.heading("play_id");
                            });
                            header.col(|ui| {
                                ui.heading("title");
                            });
                            header.col(|ui| {
                                ui.heading("author");
                            });
                            header.col(|ui| {
                                ui.heading("director");
                            });
                        })
                        .body(|mut body| {
                            for i in &*plays {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.play_id.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(i.title.as_ref().unwrap());
                                    });
                                    row.col(|ui| {
                                        ui.label(i.author.as_ref().unwrap());
                                    });
                                    row.col(|ui| {
                                        ui.label(i.director.as_ref().unwrap());
                                    });
                                });
                            }
                        });
                });
            });
            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent to close us.
                play_viewport.store(false, Ordering::Relaxed);
            }
        },
    );
}
