use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread, time::{self, Duration}};

use eframe::WindowBuilder;
use egui::{FontData, FontDefinitions, ScrollArea, Vec2, Window};

use egui_extras::{TableBuilder, Column};
use serde::de::value;
use tokio::time::sleep;


use crate::{actor::Actor, dbdriver, performance, performance_actors, play::Play, poster::{self, Poster}, stage::{self, Stage}, Performance, PerformanceActors};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    #[serde(skip)] // This how you opt-out of serialization of a field
    actors: Arc<Vec<Actor>>,
    performance: Arc<Vec<Performance>>,
    performance_actors: Arc<Vec<PerformanceActors>>,
    plays: Arc<Vec<Play>>,
    poster: Arc<Vec<Poster>>,
    stage: Arc<Vec<Stage>>,

    poster_vieport: Arc<AtomicBool>,
    actors_viewport: Arc<AtomicBool>,
    performance_viewport: Arc<AtomicBool>,
    performance_actors_viewport:Arc<AtomicBool>,
    play_viewport: Arc<AtomicBool>,
    stage_viewport: Arc<AtomicBool>,
    
    tables: Vec<String>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            tables: vec!["actor".to_owned(),"performance".to_owned(),"performance_actors".to_owned(),"play".to_owned(),"posters".to_owned(),"stage".to_owned(),"theater".to_owned(),"ticket".to_owned(),
            "viewer".to_owned(),"viewer_ticket".to_owned()],
            actors: Arc::new(Vec::new()),
            performance_actors:Arc::new(Vec::new()),
            performance:Arc::new(Vec::new()),
            plays:Arc::new(Vec::new()),
            poster: Arc::new(Vec::new()),
            stage: Arc::new(Vec::new()),
            
            poster_vieport: Arc::new(AtomicBool::new(false)),
            performance_viewport: Arc::new(AtomicBool::new(false)),
            actors_viewport: Arc::new(AtomicBool::new(false)),
            performance_actors_viewport: Arc::new(AtomicBool::new(false)),
            play_viewport:  Arc::new(AtomicBool::new(false)),
            stage_viewport: Arc::new(AtomicBool::new(false)),
            
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let mut font = FontDefinitions::default();
        font.font_data.insert(
            String::from("AnimeAcev3"),
            FontData::from_static(include_bytes!("../AnimeAcev3.ttf")),
        );
        cc.egui_ctx.set_fonts(font);
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Database");
            // let v: Vec<String> = vec!["actor".to_owned(),"performance".to_owned(),"performance_actors".to_owned(),"
            // play".to_owned(),"posters".to_owned(),"stage".to_owned(),"theater".to_owned(),"ticket".to_owned(),
            // "viewer".to_owned(),"viewer_ticket".to_owned()];


            let mut actors_viewport = self.actors_viewport.load(Ordering::Relaxed);
            let mut performance_viewport = self.performance_viewport.load(Ordering::Relaxed);
            let mut performance_actors_viewport = self.performance_actors_viewport.load(Ordering::Relaxed);
            let mut play_viewport = self.play_viewport.load(Ordering::Relaxed);
            let mut poster_viewport = self.poster_vieport.load(Ordering::Relaxed);
            let mut stage_viewport = self.stage_viewport.load(Ordering::Relaxed);





            ui.vertical_centered(|ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let runtime = tokio::runtime::Runtime::new();
                    runtime.unwrap().block_on( async {
                    // for a in &self.tables{
                    if ui.add(egui::Button::new(self.tables.get(0).unwrap()).min_size(Vec2{x: 200.0, y:50.0})).clicked() {
                        // ui.checkbox(&mut actors_viewport, "Show deferred child viewport");
                        actors_viewport=true;
                        self.actors_viewport
                            .store(actors_viewport, Ordering::Relaxed);

                        self.actors = Arc::new(dbdriver::actors().await.unwrap());
                    }

                    if ui.add(egui::Button::new(self.tables.get(1).unwrap()).min_size(Vec2{x: 200.0, y:50.0})).clicked() {

                        // ui.checkbox(&mut actors_viewport, "Show deferred child viewport");
                        performance_viewport=true;
                        self.performance_viewport
                            .store(performance_viewport, Ordering::Relaxed);
                        self.performance = Arc::new(dbdriver::performance().await.unwrap());
                    }

                    if ui.add(egui::Button::new(self.tables.get(2).unwrap()).min_size(Vec2{x: 200.0, y:50.0})).clicked() {

                        // ui.checkbox(&mut actors_viewport, "Show deferred child viewport");
                        performance_actors_viewport=true;
                        self.performance_actors_viewport
                            .store(performance_actors_viewport, Ordering::Relaxed);
                        self.performance_actors = Arc::new(dbdriver::performance_actors().await.unwrap());
                    }

                    if ui.add(egui::Button::new(self.tables.get(3).unwrap()).min_size(Vec2{x: 200.0, y:50.0})).clicked() {

                        // ui.checkbox(&mut actors_viewport, "Show deferred child viewport");
                        play_viewport=true;
                        self.play_viewport
                            .store(play_viewport, Ordering::Relaxed);
                        self.plays = Arc::new(dbdriver::play().await.unwrap());
                    }
                    if ui.add(egui::Button::new(self.tables.get(4).unwrap()).min_size(Vec2{x: 200.0, y:50.0})).clicked() {
                        poster_viewport=true;
                        self.poster_vieport
                            .store(poster_viewport, Ordering::Relaxed);
                        self.poster = Arc::new(dbdriver::poster().await.unwrap());
                    }   
                    if ui.add(egui::Button::new(self.tables.get(5).unwrap()).min_size(Vec2{x: 200.0, y:50.0})).clicked() {
                        stage_viewport=true;
                        self.stage_viewport
                            .store(stage_viewport, Ordering::Relaxed);
                        self.stage = Arc::new(dbdriver::stage().await.unwrap());
                        
                    }
                    if ui.add(egui::Button::new(self.tables.get(6).unwrap()).min_size(Vec2{x: 200.0, y:50.0})).clicked() {

                      
                    }
                    if ui.add(egui::Button::new(self.tables.get(7).unwrap()).min_size(Vec2{x: 200.0, y:50.0})).clicked() {

                      
                    }
                    if ui.add(egui::Button::new(self.tables.get(8).unwrap()).min_size(Vec2{x: 200.0, y:50.0})).clicked() {

                      
                    }
                    if ui.add(egui::Button::new(self.tables.get(9).unwrap()).min_size(Vec2{x: 200.0, y:50.0})).clicked() {

                      
                    }
                })
                    
                    
                });

            });

        

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                // powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });

        if self.stage_viewport.load(Ordering::Relaxed) {
            let stage_viewport = self.stage_viewport.clone();
            let stage = self.stage.clone();
            ctx.show_viewport_deferred(
                egui::ViewportId::from_hash_of("Stage"),
                egui::ViewportBuilder::default()
                    .with_title("Stage")
                    .with_inner_size([200.0, 100.0]),
                move |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Deferred,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default().show(ctx, |ui| {
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
                            for i in &*stage{
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.get_sid().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_tid().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_capacity().to_string());
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

        if self.poster_vieport.load(Ordering::Relaxed) {
            let poster_vieport = self.poster_vieport.clone();
            let poster = self.poster.clone();
            ctx.show_viewport_deferred(
                egui::ViewportId::from_hash_of("Poster"),
                egui::ViewportBuilder::default()
                    .with_title("Poster")
                    .with_inner_size([200.0, 100.0]),
                move |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Deferred,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default().show(ctx, |ui| {
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
                            for i in &*poster{
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.get_id().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_pid().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_sd());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_edate());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_content());
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

        if self.performance_actors_viewport.load(Ordering::Relaxed) {
            let performance_actors_viewport = self.performance_actors_viewport.clone();
            let performance_actors = self.performance_actors.clone();
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
                            for i in &*performance_actors{
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.get_performance_performance_id().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_actor_actor_id().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_amount().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_actors_perfor_id().to_string());
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

        if self.actors_viewport.load(Ordering::Relaxed) {
            let actors_viewport = self.actors_viewport.clone();
            let actors = self.actors.clone();
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
                            for i in &*actors{
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.get_id().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_name());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_surname());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_role());
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



        if self.play_viewport.load(Ordering::Relaxed) {
            let play_viewport = self.play_viewport.clone();
            let plays = self.plays.clone();
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
                            for i in &*plays{
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.get_play_id().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_title());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_author());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_director());
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


        if self.performance_viewport.load(Ordering::Relaxed) {
            let performance_viewport = self.performance_viewport.clone();
            let performance = self.performance.clone();
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
                            for i in &*performance{
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.get_id().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_plid().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_sid().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_date().to_string());
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
    }
}
