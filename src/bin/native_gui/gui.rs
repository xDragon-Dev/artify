use artify::prelude::*;

pub struct AppGUI {
    text: String,
    align: Align,
    size_kind: SizeKind,
}
impl Default for AppGUI {
    fn default() -> Self {
        Self {
            text: String::from("GG WP"),
            align: Align::Center,
            size_kind: SizeKind::Short,
        }
    }
}

impl AppGUI {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_zoom_factor(1.5);
        cc.egui_ctx.set_theme(egui::Theme::Dark);
        Self::default()
    }
}

impl eframe::App for AppGUI {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                let label = ui.label("Text");
                let line = ui.text_edit_singleline(&mut self.text);
                line.labelled_by(label.id);
                if ui.button("clear").clicked() {
                    self.text.clear();
                }
            });
            ui.add_space(10.0);

            //Select Used characters
            ui.horizontal(|ui| {
                ui.label("Text Size");

                ui.selectable_value(&mut self.size_kind, SizeKind::Full, "Full");
                ui.selectable_value(&mut self.size_kind, SizeKind::Compact, "Compact");
                ui.selectable_value(&mut self.size_kind, SizeKind::Short, "Short");
            });
            ui.add_space(10.0);

            //Select used align
            ui.horizontal(|ui| {
                ui.label("Text Align");
                ui.selectable_value(&mut self.align, Align::Left, "Left");
                ui.selectable_value(&mut self.align, Align::Center, "Center");
                ui.selectable_value(&mut self.align, Align::Right, "Right");
            });
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                char_set::set_charset(self.size_kind);

                let formatted = Formatter::from(self.text.as_str())
                    .with_align(self.align)
                    .fmt();
                let art_str = ArtString::from(formatted.as_str());

                ui.code(&art_str.as_readable());
                if ui.button("Copy").clicked() {
                    ui.ctx().copy_text(art_str.as_usable());
                }
            });
        });
    }
}
