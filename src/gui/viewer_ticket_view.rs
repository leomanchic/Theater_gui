use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use egui_extras::{Column, TableBuilder};

use crate::{
    dbworker::dbdriver,
    entity::{self, theater},
};

pub fn viewer_ticket_view(
    ctx: &egui::Context,
    vticket: &Vec<entity::viewer_ticket::Model>,
    vticket_viewport: &mut Arc<AtomicBool>,
) {
    let vticket_viewport = vticket_viewport.clone();
    let vticket = vticket.clone();
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
