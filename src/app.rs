use std::{cell::Cell, sync::{atomic::{AtomicBool, Ordering}, Arc}, thread, time::{self, Duration}};

use eframe::WindowBuilder;
use egui::{FontData, FontDefinitions, ScrollArea, Vec2, Window};

use egui_extras::{TableBuilder, Column};
use serde::de::value;
use tokio::{select, time::sleep};


#[derive(serde::Deserialize, serde::Serialize)]

struct Views{
    poster_vieport: Arc<AtomicBool>,
    actors_viewport: Arc<AtomicBool>,
    performance_viewport: Arc<AtomicBool>,
    performance_actors_viewport:Arc<AtomicBool>,
    play_viewport: Arc<AtomicBool>,
    stage_viewport: Arc<AtomicBool>,

}

use crate::{actor::Actor, dbdriver, performance, performance_actors, play::Play, poster::{self, Poster}, stage::{self, Stage}, theater::{self, Theater}, ticket::{self, Ticket}, viewer::{self, Viewer}, viewer_ticket::ViewerTicket, Performance, PerformanceActors};

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
    theater: Arc<Vec<Theater>>,
    ticket: Arc<Vec<Ticket>>,
    viewer: Arc<Vec<Viewer>>,
    vticket: Arc<Vec<ViewerTicket>>,

    poster_vieport: Arc<AtomicBool>,
    actors_viewport: Arc<AtomicBool>,
    performance_viewport: Arc<AtomicBool>,
    performance_actors_viewport:Arc<AtomicBool>,
    play_viewport: Arc<AtomicBool>,
    stage_viewport: Arc<AtomicBool>,
    ticket_viewport: Arc<AtomicBool>,
    theater_viewport: Arc<AtomicBool>,
    viewer_viewport: Arc<AtomicBool>,
    vticket_viewport: Arc<AtomicBool>,
    
    // views: Views,
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
            theater: Arc::new(Vec::new()),
            ticket: Arc::new(Vec::new()),
            viewer: Arc::new(Vec::new()),
            vticket: Arc::new(Vec::new()),


            poster_vieport: Arc::new(AtomicBool::new(false)),
            performance_viewport: Arc::new(AtomicBool::new(false)),
            actors_viewport: Arc::new(AtomicBool::new(false)),
            performance_actors_viewport: Arc::new(AtomicBool::new(false)),
            play_viewport:  Arc::new(AtomicBool::new(false)),
            stage_viewport: Arc::new(AtomicBool::new(false)),
            theater_viewport:  Arc::new(AtomicBool::new(false)),
            ticket_viewport: Arc::new(AtomicBool::new(false)),
            viewer_viewport: Arc::new(AtomicBool::new(false)),
            vticket_viewport: Arc::new(AtomicBool::new(false)),
        }
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    let mut font = FontDefinitions::default();
    font.font_data.insert(
        String::from("cfonts"),
        FontData::from_static(include_bytes!("../cfonts.ttf")),
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
                let m =  egui::Button::new("Click me");
                ui.add(m).clicked();
               
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
            let mut stage_viewport: bool = self.stage_viewport.load(Ordering::Relaxed);
            let mut theater_viewport: bool = self.theater_viewport.load(Ordering::Relaxed);
            let mut ticket_viewport: bool = self.ticket_viewport.load(Ordering::Relaxed);
            let mut viewer_viewport: bool = self.viewer_viewport.load(Ordering::Relaxed);
            let mut vticket_viewport: bool = self.vticket_viewport.load(Ordering::Relaxed);


            
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
                                self.performance =  Arc::new(dbdriver::performance().await.unwrap())
                            }

                    if ui.add(egui::Button::new(self.tables.get(2).unwrap()).min_size(Vec2{x: 200.0, y:50.0})).clicked() {

                                // ui.checkbox(&mut actors_viewport, "Show deferred child viewport");
                                performance_actors_viewport=true;
                                self.performance_actors_viewport
                                    .store(performance_actors_viewport, Ordering::Relaxed);
                                self.performance_actors =  Arc::new(dbdriver::performance_actors().await.unwrap());
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
                        theater_viewport=true;
                        self.theater_viewport
                            .store(theater_viewport, Ordering::Relaxed);
                        self.theater = Arc::new(dbdriver::theater().await.unwrap());
                        
                      
                    }
                    if ui.add(egui::Button::new(self.tables.get(7).unwrap()).min_size(Vec2{x: 200.0, y:50.0})).clicked() {
                        ticket_viewport=true;
                        self.ticket_viewport
                            .store(ticket_viewport, Ordering::Relaxed);
                        self.theater = Arc::new(dbdriver::theater().await.unwrap());
                        
                      
                    }
                    if ui.add(egui::Button::new(self.tables.get(8).unwrap()).min_size(Vec2{x: 200.0, y:50.0})).clicked() {
                        viewer_viewport=true;
                        self.viewer_viewport
                            .store(viewer_viewport, Ordering::Relaxed);
                        self.viewer = Arc::new(dbdriver::viewer().await.unwrap());

                      
                    }
                    if ui.add(egui::Button::new(self.tables.get(9).unwrap()).min_size(Vec2{x: 200.0, y:50.0})).clicked() {
                        vticket_viewport=true;
                        self.vticket_viewport
                            .store(vticket_viewport, Ordering::Relaxed);
                        self.vticket = Arc::new(dbdriver::viewer_ticket().await.unwrap());
                      
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
                        egui::ScrollArea::vertical().show(ui, |ui| {

                        TableBuilder::new(ui).columns(Column::auto().resizable(true),4usize)
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
                            for i in &*vticket{
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.get_vvid().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_ttid().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_bdate());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_vtid().to_string());
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

                        TableBuilder::new(ui).columns(Column::auto().resizable(true), 4usize)
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
                            for i in &*viewer{
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.get_vid().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_name());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_email());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_phone());
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
                            for i in &*ticket{
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.get_tid().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_pid().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_snum().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_date());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_cost().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_status());
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
                            for i in &*theater{
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(&i.get_tid().to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_name());
                                    });
                                    row.col(|ui| {
                                        ui.label(&i.get_address());
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
