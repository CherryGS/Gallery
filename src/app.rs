use egui::Label;

pub struct TemplateApp {
    folder: Path,
}

impl Default for TemplateApp {
    fn default() -> Self {
        return TemplateApp { folder: "/" };
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Choice").clicked() {
                        self.folder = rfd::FileDialog::new()
                            .set_directory("/")
                            .pick_folder()
                            .unwrap()
                            .to_str()
                            .unwrap();
                    }
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });
        // egui::CentralPanel::default().show(ctx, |ui| {
        //     // ui.heading("Test CentralPanel");
        //     // ui.label(format!("Path: {}", folder));
        // });
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        return Default::default();
    }
}
