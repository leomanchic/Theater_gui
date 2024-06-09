use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use crate::dbworker::dbdriver;
/// Raw sql querry builder
pub fn sqlwtool(ctx: &egui::Context, refrsql: &mut Arc<AtomicBool>) {
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
                ui.heading("Raw SQL querry executor");
                egui::CollapsingHeader::new("querry").show(ui, |ui| {
                    ui.add_sized(
                        [ui.available_width() / 2f32, 40f32],
                        egui::TextEdit::multiline(&mut *sql_querry_buffer.lock().unwrap()),
                    );
                    ui.add_space(10f32);
                    // ui.centered_and_justified(add_contents)
                    if ui.add(egui::Button::new("run")).clicked() {
                        let runtime = tokio::runtime::Runtime::new();
                        runtime.unwrap().block_on(async {
                            *result.lock().unwrap() = dbdriver::rsql_executor(
                                (*sql_querry_buffer.lock().unwrap().clone()).to_string(),
                            )
                            .await
                            .unwrap();
                        });
                    }
                    // if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    // …available_size()
                    // }
                });
                egui::CollapsingHeader::new("result").show(ui, |ui| {
                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::multiline(&mut *result.lock().unwrap()),
                    );
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
