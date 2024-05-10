use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use eframe::{glow::Context, WindowBuilder};
use egui::{FontData, FontDefinitions, ScrollArea, Vec2, Window};

use egui_extras::{Column, DatePickerButton, TableBuilder};
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

use crate::{dbworker::dbdriver, entity};

use super::{
    actor_view, performance_actors_view, performance_view, play_view, poster_view, sqlw,
    stage_view, theater_view, ticket_view, viewer_ticket_view, viewer_view,
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
            viewer_ticket_view::viewer_ticket_view(ctx, &self.vticket, &mut self.vticket_viewport)
        }

        if self.viewer_viewport.load(Ordering::Relaxed) {
            viewer_view::viewer_view(ctx, &self.viewer, &mut self.viewer_viewport)
        }

        if self.ticket_viewport.load(Ordering::Relaxed) {
            ticket_view::ticket_view(ctx, &self.ticket, &mut self.ticket_viewport)
        }

        if self.theater_viewport.load(Ordering::Relaxed) {
            theater_view::theater_view(ctx, &self.theater, &mut self.theater_viewport)
        }

        if self.stage_viewport.load(Ordering::Relaxed) {
            stage_view::stage_view(ctx, &self.stage, &mut self.stage_viewport)
        }

        if self.poster_vieport.load(Ordering::Relaxed) {
            poster_view::poster_view(ctx, &self.poster, &mut self.poster_vieport)
        }

        if self.performance_actors_viewport.load(Ordering::Relaxed) {
            performance_actors_view::performance_actors_view(
                ctx,
                &self.performance_actors,
                &mut self.performance_actors_viewport,
            )
        }

        if self.actors_viewport.load(Ordering::Relaxed) {
            actor_view(ctx, &self.actr, &mut self.actors_viewport);
        }

        if self.play_viewport.load(Ordering::Relaxed) {
            play_view(ctx, &self.plays, &mut self.play_viewport)
        }

        if self.performance_viewport.load(Ordering::Relaxed) {
            performance_view(ctx, &self.performance, &mut self.performance_viewport)
        }
    }
}
