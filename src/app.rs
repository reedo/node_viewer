use egui::{Align, Layout};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    ui_scale: f32,
}

impl Default for App {
    fn default() -> Self {
        App { ui_scale: 1.0 }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        // This is where you can customise the look and feel of egui using
        // `creation_context.egui_ctx.set_visuals` and `creation_context.egui_ctx.set_fonts`.

        // Load previous app state.
        if let Some(storage) = _creation_context.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(context, |ui| {
            context.set_pixels_per_point(self.ui_scale);

            egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    egui::ComboBox::from_label("UI scale")
                        .selected_text(format!("{:?}", &self.ui_scale))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.ui_scale, 1.0, "1.0");
                            ui.selectable_value(&mut self.ui_scale, 1.5, "1.5");
                            ui.selectable_value(&mut self.ui_scale, 2.0, "2.0");
                        });

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

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
