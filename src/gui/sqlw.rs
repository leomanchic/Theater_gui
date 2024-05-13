use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use crate::dbworker::dbdriver;

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
                ui.label("Raw SQL querry writer");
                egui::CollapsingHeader::new("querry").show(ui, |ui| {
                    let response = ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut *sql_querry_buffer.lock().unwrap()),
                    );

                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        // …available_size()
                        let runtime = tokio::runtime::Runtime::new();
                        runtime.unwrap().block_on(async {
                            *result.lock().unwrap() = dbdriver::rsql_executor(
                                (*sql_querry_buffer.lock().unwrap().clone()).to_string(),
                            )
                            .await
                            .unwrap();
                        });
                    }
                });
                egui::CollapsingHeader::new("result").show(ui, |ui| {
                    let response = ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut *result.lock().unwrap()),
                    );

                    // if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {

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
