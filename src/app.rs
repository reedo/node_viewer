use egui::{Align, Layout};

#[derive(Default)]
pub struct App;

impl App {
    /// Called once before the first frame.
    pub fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        // This is where you can customise the look and feel of egui using
        // `creation_context.egui_ctx.set_visuals` and `creation_context.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(context, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    egui::widgets::global_theme_preference_buttons(ui);
                });
            });
        });

        egui::CentralPanel::default().show(context, |ui| {
            ui.heading("Node Viewer");

            ui.with_layout(Layout::bottom_up(Align::RIGHT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
