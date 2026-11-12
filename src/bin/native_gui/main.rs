//▀▄█▌▐░▒▓
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;
use gui::AppGUI;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([425.0, 250.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Artify 1.0.0 Made By xDragonDev",
        options,
        Box::new(|cc| Ok(Box::new(AppGUI::new(cc)))),
    )
}
