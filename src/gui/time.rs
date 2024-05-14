use chrono::{NaiveDate, NaiveDateTime};
use egui::{Response, Ui};

pub fn time_piker(ui: &mut Ui) {
    let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2016, 7, 8)
        .unwrap()
        .and_hms_opt(9, 10, 11)
        .unwrap();

    ui.label("hile");
    ui.label("hit;e");
    ui.label("kkeke");
}
