use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use eframe::{glow::Context, WindowBuilder};
use egui::{FontData, FontDefinitions, ScrollArea, Vec2, Window};

use egui_extras::{Column, TableBuilder, DatePickerButton};
use serde;
use tokio;

// struct Texts {
//     text: Mutex<String>,
// }
// struct Views {
//     poster_vieport: Arc<AtomicBool>,
//     actors_viewport: Arc<AtomicBool>,
//     performance_viewport: Arc<AtomicBool>,
//     performance_actors_viewport: Arc<AtomicBool>,
//     play_viewport: Arc<AtomicBool>,
//     stage_viewport: Arc<AtomicBool>,
// }

use crate::{
    dbworker::dbdriver, entity, Actor, Performance, PerformanceActors, Play, Poster, Stage, Theater, Ticket, Viewer, ViewerTicket
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    #[serde(skip)] // This how you opt-out of serialization of a field
    // texts: Arc<Mutex<String>>,

    actr: Arc<Vec<entity::actor::Model>>,
    ticket: Arc<Vec<entity::ticket::Model>>,
    theater: Arc<Vec<entity::theater::Model>>,
    performance: Arc<Vec<entity::performance::Model>>,
    viewer: Arc<Vec<entity::viewer::Model>>,
    poster: Arc<Vec<entity::poster::Model>>,
    stage: Arc<Vec<entity::stage::Model>>,
    performance_actors: Arc<Vec<entity::performance_actors::Model>>,
    plays: Arc<Vec<entity::play::Model>>,
    vticket: Arc<Vec<entity::viewer_ticket::Model>>,


    poster_vieport: Arc<AtomicBool>,
    actors_viewport: Arc<AtomicBool>,
    performance_viewport: Arc<AtomicBool>,
    performance_actors_viewport: Arc<AtomicBool>,
    play_viewport: Arc<AtomicBool>,
    stage_viewport: Arc<AtomicBool>,
    ticket_viewport: Arc<AtomicBool>,
    theater_viewport: Arc<AtomicBool>,
    viewer_viewport: Arc<AtomicBool>,
    vticket_viewport: Arc<AtomicBool>,

    // views: Views,
    tables: Vec<String>,
    rsql: Arc<AtomicBool>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            actr: Arc::new(Vec::new()),
            ticket: Arc::new(Vec::new()),
            performance: Arc::new(Vec::new()),
            theater: Arc::new(Vec::new()),
            viewer: Arc::new(Vec::new()),
            poster: Arc::new(Vec::new()),
            performance_actors: Arc::new(Vec::new()),
            plays: Arc::new(Vec::new()),
            stage: Arc::new(Vec::new()),
            vticket: Arc::new(Vec::new()),


            tables: vec![
                "actor".to_owned(),
                "performance".to_owned(),
                "performance_actors".to_owned(),
                "play".to_owned(),
                "posters".to_owned(),
                "stage".to_owned(),
                "theater".to_owned(),
                "ticket".to_owned(),
                "viewer".to_owned(),
                "viewer_ticket".to_owned(),
            ],

            poster_vieport: Arc::new(AtomicBool::new(false)),
            performance_viewport: Arc::new(AtomicBool::new(false)),
            actors_viewport: Arc::new(AtomicBool::new(false)),
            performance_actors_viewport: Arc::new(AtomicBool::new(false)),
            play_viewport: Arc::new(AtomicBool::new(false)),
            stage_viewport: Arc::new(AtomicBool::new(false)),
            theater_viewport: Arc::new(AtomicBool::new(false)),
            ticket_viewport: Arc::new(AtomicBool::new(false)),
            viewer_viewport: Arc::new(AtomicBool::new(false)),
            vticket_viewport: Arc::new(AtomicBool::new(false)),
            rsql: Arc::new(AtomicBool::new(false)),
        }
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    let mut font = FontDefinitions::default();
    font.font_data.insert(
        String::from("cfonts"),
        FontData::from_static(include_bytes!("../../cfonts.ttf")),
    );
    ctx.set_fonts(font);
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "cfonts".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("cfonts".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        // Install my own font (maybe supporting non-latin characters).
        // .ttf and .otf files supported.

        setup_custom_fonts(&cc.egui_ctx);
        // Put my font first (highest priority) for proportional text:

        Default::default()
    }
}

fn sqlwtool(ctx: &egui::Context, refrsql: &mut Arc<AtomicBool>) {
    let rsql = refrsql.clone();
    let sql_querry_buffer = Arc::new(Mutex::new(String::new()));
    let result = Arc::new(Mutex::new(String::new()));
    ctx.show_viewport_deferred(
        egui::ViewportId::from_hash_of("sqltool"),
        egui::ViewportBuilder::default()
            .with_title("rsqltool")
            .with_inner_size([200.0, 100.0]),
        move |ctx, class| {
          
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Raw SQL querry writer");
                egui::CollapsingHeader::new("querry").show(ui, |ui| {
                let response = ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::singleline(&mut *sql_querry_buffer.lock().unwrap()),
                );

                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    // …available_size()
                }
                });
                egui::CollapsingHeader::new("result").show(ui, |ui| {
                    let response = ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut *result.lock().unwrap()),
                    );
    
                    // if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    //     // …available_size()
                    // }

                });

            });
            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent to close us.
                rsql.store(false, Ordering::Relaxed);
            }
        },
    );
    // Tell parent to close us.
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
                let mut rsql = self.rsql.load(Ordering::Relaxed);
                let m = egui::Button::new("rsql");
                let responce = ui.add(m).clicked();
                if responce {
                    rsql = true;
                    self.rsql.store(rsql, Ordering::Relaxed);
                }
                if self.rsql.load(Ordering::Relaxed) {
                    sqlwtool(ctx, &mut self.rsql);
                }
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
            let mut performance_actors_viewport =
                self.performance_actors_viewport.load(Ordering::Relaxed);
            let mut play_viewport = self.play_viewport.load(Ordering::Relaxed);
            let mut poster_viewport = self.poster_vieport.load(Ordering::Relaxed);
            let mut stage_viewport: bool = self.stage_viewport.load(Ordering::Relaxed);
            let mut theater_viewport: bool = self.theater_viewport.load(Ordering::Relaxed);
            let mut ticket_viewport: bool = self.ticket_viewport.load(Ordering::Relaxed);
            let mut viewer_viewport: bool = self.viewer_viewport.load(Ordering::Relaxed);
            let mut vticket_viewport: bool = self.vticket_viewport.load(Ordering::Relaxed);

            ui.vertical_centered(|ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let runtime = tokio::runtime::Runtime::new();
                    runtime.unwrap().block_on(async {
                        // for a in &self.tables{
                        if ui
                            .add(
                                egui::Button::new(self.tables.get(0).unwrap())
                                    .min_size(Vec2 { x: 200.0, y: 50.0 }),
                            )
                            .clicked()
                        {
                            // ui.checkbox(&mut actors_viewport, "Show deferred child viewport");
                            actors_viewport = true;
                            self.actors_viewport
                                .store(actors_viewport, Ordering::Relaxed);
                            self.actr = Arc::new(dbdriver::get_actors().await.unwrap());

                            // self.actors = Arc::new(dbdriver::actors().await.unwrap());
                        }

                        if ui
                            .add(
                                egui::Button::new(self.tables.get(1).unwrap())
                                    .min_size(Vec2 { x: 200.0, y: 50.0 }),
                            )
                            .clicked()
                        {
                            // ui.checkbox(&mut actors_viewport, "Show deferred child viewport");
                            performance_viewport = true;
                            self.performance_viewport
                                .store(performance_viewport, Ordering::Relaxed);
                            self.performance = Arc::new(dbdriver::get_performances().await.unwrap())
                        }

                        if ui
                            .add(
                                egui::Button::new(self.tables.get(2).unwrap())
                                    .min_size(Vec2 { x: 200.0, y: 50.0 }),
                            )
                            .clicked()
                        {
                            // ui.checkbox(&mut actors_viewport, "Show deferred child viewport");
                            performance_actors_viewport = true;
                            self.performance_actors_viewport
                                .store(performance_actors_viewport, Ordering::Relaxed);
                            self.performance_actors =
                                Arc::new(dbdriver::get_per_actors().await.unwrap());
                        }

                        if ui
                            .add(
                                egui::Button::new(self.tables.get(3).unwrap())
                                    .min_size(Vec2 { x: 200.0, y: 50.0 }),
                            )
                            .clicked()
                        {
                            // ui.checkbox(&mut actors_viewport, "Show deferred child viewport");
                            play_viewport = true;
                            self.play_viewport.store(play_viewport, Ordering::Relaxed);
                            self.plays = Arc::new(dbdriver::get_plays().await.unwrap());
                        }
                        if ui
                            .add(
                                egui::Button::new(self.tables.get(4).unwrap())
                                    .min_size(Vec2 { x: 200.0, y: 50.0 }),
                            )
                            .clicked()
                        {
                            poster_viewport = true;
                            self.poster_vieport
                                .store(poster_viewport, Ordering::Relaxed);
                            self.poster = Arc::new(dbdriver::get_poster().await.unwrap());
                        }
                        if ui
                            .add(
                                egui::Button::new(self.tables.get(5).unwrap())
                                    .min_size(Vec2 { x: 200.0, y: 50.0 }),
                            )
                            .clicked()
                        {
                            stage_viewport = true;
                            self.stage_viewport.store(stage_viewport, Ordering::Relaxed);
                            self.stage = Arc::new(dbdriver::get_stage().await.unwrap());
                        }
                        if ui
                            .add(
                                egui::Button::new(self.tables.get(6).unwrap())
                                    .min_size(Vec2 { x: 200.0, y: 50.0 }),
                            )
                            .clicked()
                        {
                            theater_viewport = true;
                            self.theater_viewport
                                .store(theater_viewport, Ordering::Relaxed);
                            self.theater = Arc::new(dbdriver::get_theaters().await.unwrap());
                        }
                        if ui
                            .add(
                                egui::Button::new(self.tables.get(7).unwrap())
                                    .min_size(Vec2 { x: 200.0, y: 50.0 }),
                            )
                            .clicked()
                        {
                            ticket_viewport = true;
                            self.ticket_viewport
                                .store(ticket_viewport, Ordering::Relaxed);
                            self.ticket = Arc::new(dbdriver::get_tickets().await.unwrap());

                        }
                        if ui
                            .add(
                                egui::Button::new(self.tables.get(8).unwrap())
                                    .min_size(Vec2 { x: 200.0, y: 50.0 }),
                            )
                            .clicked()
                        {
                            viewer_viewport = true;
                            self.viewer_viewport
                                .store(viewer_viewport, Ordering::Relaxed);
                            self.viewer = Arc::new(dbdriver::get_viewer().await.unwrap());
                        }
                        if ui
                            .add(
                                egui::Button::new(self.tables.get(9).unwrap())
                                    .min_size(Vec2 { x: 200.0, y: 50.0 }),
                            )
                            .clicked()
                        {
                            vticket_viewport = true;
                            self.vticket_viewport
                                .store(vticket_viewport, Ordering::Relaxed);
                            self.vticket = Arc::new(dbdriver::get_viewer_ticket().await.unwrap());
                        }
                    });
                });
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                // powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });

        if self.vticket_viewport.load(Ordering::Relaxed) {
            let vticket_viewport = self.vticket_viewport.clone();
            let vticket = self.vticket.clone();
            let mess = Arc::new(Mutex::new(String::new()));

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
                        let mut k = mess.lock().unwrap();
                        // let response =
                        //     ui.add(egui::TextEdit::singleline( &mut *k));
                        // if response.changed() {
                        //     println!("{}", k)
                        // }
                        // if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        //     println!("pressed")
                        // }
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

        if self.viewer_viewport.load(Ordering::Relaxed) {
            let viewer_viewport = self.viewer_viewport.clone();
            let viewer = self.viewer.clone();
            ctx.show_viewport_deferred(
                egui::ViewportId::from_hash_of("viewer"),
                egui::ViewportBuilder::default()
                    .with_title("viewer")
                    .with_inner_size([200.0, 100.0]),
                move |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Deferred,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default().show(ctx, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            TableBuilder::new(ui)
                                .columns(Column::auto().resizable(true), 4usize)
                                .header(20.0, |mut header| {
                                    header.col(|ui| {
                                        ui.heading("viewer_id");
                                    });
                                    header.col(|ui| {
                                        ui.heading("name");
                                    });
                                    header.col(|ui| {
                                        ui.heading("email");
                                    });
                                    header.col(|ui| {
                                        ui.heading("phone");
                                    });
                                })
                                .body(|mut body| {
                                    for i in &*viewer {
                                        body.row(30.0, |mut row| {
                                            row.col(|ui| {
                                                ui.label(&i.viewer_id.to_string());
                                            });
                                            row.col(|ui| {
                                                ui.label(i.name.as_ref().unwrap());
                                            });
                                            row.col(|ui| {
                                                ui.label(i.email.as_ref().unwrap());
                                            });
                                            row.col(|ui| {
                                                ui.label(i.phone.as_ref().unwrap());
                                            });
                                        });
                                    }
                                });
                        });
                });
                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent to close us.
                        viewer_viewport.store(false, Ordering::Relaxed);
                    }
                },
            );
        }

        if self.ticket_viewport.load(Ordering::Relaxed) {
            let ticket_viewport = self.ticket_viewport.clone();
            let ticket = self.ticket.clone();
            ctx.show_viewport_deferred(
                egui::ViewportId::from_hash_of("ticket"),
                egui::ViewportBuilder::default()
                    .with_title("ticket")
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
                                .column(Column::auto().resizable(true))
                                .header(20.0, |mut header| {
                                    header.col(|ui| {
                                        ui.heading("ticket_id");
                                    });
                                    header.col(|ui| {
                                        ui.heading("performance_id");
                                    });
                                    header.col(|ui| {
                                        ui.heading("seat_number");
                                    });
                                    header.col(|ui| {
                                        ui.heading("date");
                                    });
                                    header.col(|ui| {
                                        ui.heading("cost");
                                    });
                                    header.col(|ui| {
                                        ui.heading("status");
                                    });
                                })
                                .body(|mut body| {
                                    for i in &*ticket {
                                        body.row(30.0, |mut row| {
                                            row.col(|ui| {
                                                ui.label(&i.ticket_id.to_string());
                                            });
                                            row.col(|ui| {
                                                ui.label(&i.performance_id.unwrap_or_default().to_string());
                                            });
                                            row.col(|ui| {
                                                ui.label(&i.seat_number.unwrap_or_default().to_string());
                                            });
                                            row.col(|ui| {
                                                ui.label(&i.date.unwrap_or_default().to_string());
                                            });
                                            row.col(|ui| {
                                                ui.label(&i.cost.unwrap_or_default().to_string());
                                            });
                                            row.col(|ui| {
                                                ui.label(&i.status.clone().unwrap_or_default());
                                            });
                                        });
                                    }
                                });
                        });
                    });
                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent to close us.
                        ticket_viewport.store(false, Ordering::Relaxed);
                    }
                },
            );
        }

        if self.theater_viewport.load(Ordering::Relaxed) {
            let theater_viewport = self.theater_viewport.clone();
            let theater = self.theater.clone();
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

        if self.stage_viewport.load(Ordering::Relaxed) {
            let stage_viewport = self.stage_viewport.clone();
            let stage = self.stage.clone();
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

        if self.poster_vieport.load(Ordering::Relaxed) {
            let poster_vieport = self.poster_vieport.clone();
            let poster = self.poster.clone();
            // let start_date = Arc::new(Mutex::new(String::new()));/
            let date_s : Arc<Mutex<Option<chrono::NaiveDate>>> = Arc::new(Mutex::new(None));
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

                        egui::CollapsingHeader::new("Add performance in poster")
                        .show(ui, |ui| {
                            
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
                            let mut  date_e = bindi.get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
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
                                    for i in &*performance_actors {
                                        body.row(30.0, |mut row| {
                                            row.col(|ui| {
                                                ui.label(
                                                    &i.performance_performance_id.to_string(),
                                                );
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

        if self.actors_viewport.load(Ordering::Relaxed) {
            let actors_viewport = self.actors_viewport.clone();
            let actr = self.actr.clone();
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
                        egui::CollapsingHeader::new("Add actor")
                            .show(ui, |ui| {
                                
                            
                            egui::Grid::new("some_unique_id").show(ui, |ui| {
                            ui.label("Actor name:");
                            // let response2 =
                                ui.add_sized(ui.min_size(), egui::TextEdit::singleline(&mut *name.lock().unwrap()));
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
                            if response.lost_focus()
                                && ui.input(|i| i.key_pressed(egui::Key::Enter))
                            {
                                // …available_size()
                                let  name = Arc::clone(&name);
                                let  sn = Arc::clone(&sn);
                                let  role = Arc::clone(&role);
                                let runtime = tokio::runtime::Runtime::new();
                                runtime.unwrap().block_on(async {
                                dbdriver::writer(format!{"insert into actor(name,surname,role) values ('{}', '{}', '{}');", *name.lock().unwrap(),*sn.lock().unwrap(),*role.lock().unwrap()}).await.unwrap();});
                            }
                            ui.end_row();
                            // ui.label("Actor surname:");
                            // ui.label("Actor`s role:");
                        });});

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
                                    for i in &*performance {
                                        body.row(30.0, |mut row| {
                                            row.col(|ui| {
                                                ui.label(&i.performance_id.to_string());
                                            });
                                            row.col(|ui| {
                                                ui.label(&i.play_id.unwrap().to_string());
                                            });
                                            row.col(|ui| {
                                                ui.label(&i.stage_id.unwrap().to_string());
                                            });
                                            row.col(|ui| {
                                                ui.label(&i.start_datetime.unwrap().to_string());
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
