use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use eframe;
use egui::{FontData, FontDefinitions, Vec2};

// use egui_extras;
use serde;
use tokio;

use crate::dbworker::dbdriver;

use super::{
    actor_view::{self, ActorView},
    performance_actors_view::{self, Performance_Actors_View},
    performance_view::{self, Performance_View},
    play_view::{self, Plays_View},
    poster_view::{self, Poster_View},
    sqlw,
    stage_view::{self, Stage_View},
    theater_view::{self, Theater_View},
    ticket_view::{self, Ticket_View},
    viewer_ticket_view::{self, Viewer_Ticket_View},
    viewer_view::{self, Viewer_view},
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    // #[serde(skip)] // This how you opt-out of serialization of a field
    ticket_viewport: Ticket_View,
    stage_viewport: Stage_View,
    // texts: Arc<Mutex<String>>,
    // actr: Arc<Vec<entity::actor::Model>>,
    actview: ActorView,
    performance_actor_viewport: Performance_Actors_View,
    performance: Performance_View,
    theater_viewport: Theater_View,
    viewer_viewport: Viewer_view,
    poster_viewport: Poster_View,
    plays_viewport: Plays_View,
    viewer_ticket_viewport: Viewer_Ticket_View,
    // ticket: Arc<Vec<entity::ticket::Model>>,
    // poster: Arc<Vec<entity::poster::Model>>,
    // performance_actors: Arc<Vec<entity::performance_actors::Model>>,

    // actors_viewport: Arc<AtomicBool>,
    // performance_viewport: Arc<AtomicBool>,
    // performance_actors_viewport: Arc<AtomicBool>,
    // stage_viewport: Arc<AtomicBool>,
    // ticket_viewport: Arc<AtomicBool>,
    // theater_viewport: Arc<AtomicBool>,
    // viewer_viewport: Arc<AtomicBool>,

    // views: Views,
    tables: Vec<String>,
    rsql: Arc<AtomicBool>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Init stuff:
            actview: ActorView::new(),
            performance_actor_viewport: Performance_Actors_View::new(),
            performance: Performance_View::new(),
            ticket_viewport: Ticket_View::new(),
            theater_viewport: Theater_View::new(),
            viewer_viewport: Viewer_view::new(),
            plays_viewport: Plays_View::new(),
            poster_viewport: Poster_View::new(),
            stage_viewport: Stage_View::new(),
            viewer_ticket_viewport: Viewer_Ticket_View::new(),
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
                    sqlw::sqlwtool(ctx, &mut self.rsql);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Database");
            // let v: Vec<String> = vec!["actor".to_owned(),"performance".to_owned(),"performance_actors".to_owned(),"
            // play".to_owned(),"posters".to_owned(),"stage".to_owned(),"theater".to_owned(),"ticket".to_owned(),
            // "viewer".to_owned(),"viewer_ticket".to_owned()];

            // let mut actors_viewport = self.actors_viewport.load(Ordering::Relaxed);
            let mut av = self.actview.view_enabled.load(Ordering::Relaxed);
            let mut pav = self
                .performance_actor_viewport
                .view_enabled
                .load(Ordering::Relaxed);

            let mut performance_view = self.performance.view_enabled.load(Ordering::Relaxed);
            // let mut performance_actors_viewport =
            //     self.performance_actors_viewport.load(Ordering::Relaxed);
            let mut play_view = self.plays_viewport.view_enabled.load(Ordering::Relaxed);
            let mut poster_view = self.poster_viewport.view_enabled.load(Ordering::Relaxed);
            let mut stage_view = self.stage_viewport.view_enabled.load(Ordering::Relaxed);
            let mut theater_view = self.theater_viewport.view_enabled.load(Ordering::Relaxed);
            let mut ticket_view = self.ticket_viewport.view_enabled.load(Ordering::Relaxed);
            let mut viewer_view = self.viewer_viewport.view_enabled.load(Ordering::Relaxed);
            let mut vticket_view = self
                .viewer_ticket_viewport
                .view_enabled
                .load(Ordering::Relaxed);

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
                            av = true;
                            self.actview.view_enabled.store(av, Ordering::Relaxed);
                            let mut act = self.actview.content.lock().unwrap();
                            *act = dbdriver::get_actors().await.unwrap();

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
                            performance_view = true;
                            self.performance
                                .view_enabled
                                .store(performance_view, Ordering::Relaxed);
                            let mut pvec = self.performance.content.lock().unwrap();
                            *pvec = dbdriver::get_performances().await.unwrap()
                        }

                        if ui
                            .add(
                                egui::Button::new(self.tables.get(2).unwrap())
                                    .min_size(Vec2 { x: 200.0, y: 50.0 }),
                            )
                            .clicked()
                        {
                            // ui.checkbox(&mut actors_viewport, "Show deferred child viewport");
                            pav = true;
                            self.performance_actor_viewport
                                .view_enabled
                                .store(pav, Ordering::Relaxed);
                            let mut pac = self.performance_actor_viewport.content.lock().unwrap();
                            *pac = dbdriver::get_per_actors().await.unwrap();
                        }

                        if ui
                            .add(
                                egui::Button::new(self.tables.get(3).unwrap())
                                    .min_size(Vec2 { x: 200.0, y: 50.0 }),
                            )
                            .clicked()
                        {
                            // ui.checkbox(&mut actors_viewport, "Show deferred child viewport");
                            play_view = true;
                            self.plays_viewport
                                .view_enabled
                                .store(play_view, Ordering::Relaxed);
                            let mut binding = self.plays_viewport.content.lock().unwrap();
                            *binding = dbdriver::get_plays().await.unwrap();
                        }
                        if ui
                            .add(
                                egui::Button::new(self.tables.get(4).unwrap())
                                    .min_size(Vec2 { x: 200.0, y: 50.0 }),
                            )
                            .clicked()
                        {
                            poster_view = true;
                            self.poster_viewport
                                .view_enabled
                                .store(poster_view, Ordering::Relaxed);
                            let mut binding = self.poster_viewport.content.lock().unwrap();
                            *binding = dbdriver::get_poster().await.unwrap();
                        }
                        if ui
                            .add(
                                egui::Button::new(self.tables.get(5).unwrap())
                                    .min_size(Vec2 { x: 200.0, y: 50.0 }),
                            )
                            .clicked()
                        {
                            stage_view = true;
                            self.stage_viewport
                                .view_enabled
                                .store(stage_view, Ordering::Relaxed);
                            let mut binding = self.stage_viewport.content.lock().unwrap();
                            *binding = dbdriver::get_stage().await.unwrap();
                        }
                        if ui
                            .add(
                                egui::Button::new(self.tables.get(6).unwrap())
                                    .min_size(Vec2 { x: 200.0, y: 50.0 }),
                            )
                            .clicked()
                        {
                            theater_view = true;
                            self.theater_viewport
                                .view_enabled
                                .store(theater_view, Ordering::Relaxed);
                            let mut binding = self.theater_viewport.content.lock().unwrap();
                            *binding = dbdriver::get_theaters().await.unwrap();
                        }
                        if ui
                            .add(
                                egui::Button::new(self.tables.get(7).unwrap())
                                    .min_size(Vec2 { x: 200.0, y: 50.0 }),
                            )
                            .clicked()
                        {
                            ticket_view = true;
                            self.ticket_viewport
                                .view_enabled
                                .store(ticket_view, Ordering::Relaxed);
                            let mut binding = self.ticket_viewport.content.lock().unwrap();
                            *binding = dbdriver::get_tickets().await.unwrap();
                        }
                        if ui
                            .add(
                                egui::Button::new(self.tables.get(8).unwrap())
                                    .min_size(Vec2 { x: 200.0, y: 50.0 }),
                            )
                            .clicked()
                        {
                            viewer_view = true;
                            self.viewer_viewport
                                .view_enabled
                                .store(viewer_view, Ordering::Relaxed);
                            let mut binding = self.viewer_viewport.content.lock().unwrap();
                            *binding = dbdriver::get_viewer().await.unwrap();
                        }
                        if ui
                            .add(
                                egui::Button::new(self.tables.get(9).unwrap())
                                    .min_size(Vec2 { x: 200.0, y: 50.0 }),
                            )
                            .clicked()
                        {
                            vticket_view = true;
                            self.viewer_ticket_viewport
                                .view_enabled
                                .store(vticket_view, Ordering::Relaxed);
                            let mut binding = self.viewer_ticket_viewport.content.lock().unwrap();
                            *binding = dbdriver::get_viewer_ticket().await.unwrap();
                        }
                    });
                });
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                // powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });

        if self
            .viewer_ticket_viewport
            .view_enabled
            .load(Ordering::Relaxed)
        {
            viewer_ticket_view::viewer_ticket_view(
                ctx,
                &self.viewer_ticket_viewport.content.lock().unwrap(),
                &mut self.viewer_ticket_viewport.view_enabled,
            )
        }

        if self.viewer_viewport.view_enabled.load(Ordering::Relaxed) {
            viewer_view::viewer_view(
                ctx,
                &self.viewer_viewport.content.lock().unwrap(),
                &mut self.viewer_viewport.view_enabled,
            )
        }

        if self.ticket_viewport.view_enabled.load(Ordering::Relaxed) {
            ticket_view::ticket_view(
                ctx,
                &self.ticket_viewport.content.lock().unwrap(),
                &mut self.ticket_viewport.view_enabled,
            )
        }

        if self.theater_viewport.view_enabled.load(Ordering::Relaxed) {
            theater_view::theater_view(
                ctx,
                &self.theater_viewport.content.lock().unwrap(),
                &mut self.theater_viewport.view_enabled,
            )
        }

        if self.stage_viewport.view_enabled.load(Ordering::Relaxed) {
            stage_view::stage_view(
                ctx,
                &self.stage_viewport.content.lock().unwrap(),
                &mut self.stage_viewport.view_enabled,
            )
        }

        if self.poster_viewport.view_enabled.load(Ordering::Relaxed) {
            poster_view::poster_view(
                ctx,
                &self.poster_viewport.content.lock().unwrap(),
                &mut self.poster_viewport.view_enabled,
            )
        }

        if self
            .performance_actor_viewport
            .view_enabled
            .load(Ordering::Relaxed)
        {
            performance_actors_view::performance_actors_view(
                ctx,
                &self.performance_actor_viewport.content.lock().unwrap(),
                &mut self.performance_actor_viewport.view_enabled,
            )
        }

        if self.actview.view_enabled.load(Ordering::Relaxed) {
            actor_view::actor_view(
                ctx,
                &self.actview.content.lock().unwrap(),
                &mut self.actview.view_enabled,
            );
        }

        if self.plays_viewport.view_enabled.load(Ordering::Relaxed) {
            play_view::play_view(
                ctx,
                &self.plays_viewport.content.lock().unwrap(),
                &mut self.plays_viewport.view_enabled,
            )
        }

        if self.performance.view_enabled.load(Ordering::Relaxed) {
            performance_view::performance_view(
                ctx,
                &self.performance.content.lock().unwrap(),
                &mut self.performance.view_enabled,
            )
        }
    }
}
