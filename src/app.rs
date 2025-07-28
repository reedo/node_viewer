#[cfg(not(target_arch = "wasm32"))]
use crate::file_loading::open_native_file_dialog;

#[cfg(target_arch = "wasm32")]
use crate::file_loading::open_web_file_dialog;

use crate::file_loading::FileDetails;
use egui::{Align, Layout, UiKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    ui_scale: f32,

    #[serde(skip)]
    loaded_file: Option<FileDetails>,

    file_error: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        App {
            ui_scale: 1.0,
            loaded_file: None,
            file_error: None,
        }
    }
}

impl App {
    pub fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        // This is where you can customise the look and feel of egui using
        // `creation_context.egui_ctx.set_visuals` and `creation_context.egui_ctx.set_fonts`.

        // Load previous app state.
        if let Some(storage) = _creation_context.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn open_file_dialog(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        let file_details_result = open_native_file_dialog();

        #[cfg(target_arch = "wasm32")]
        let file_details_result = open_web_file_dialog();

        match file_details_result {
            Ok(file_details) => {
                self.loaded_file = Some(file_details);
                self.file_error = None;
            }
            Err(e) => {
                self.file_error = Some(format!("Error opening file: {e}"));
                self.loaded_file = None;
            }
        }
    }

    fn display_file_content(&self, ui: &mut egui::Ui, content: &[u8]) {
        // This method would handle displaying the file content based on the file type
        // For example, if it's a text file, you could display it as text
        // If it's a binary file, you might display a hex view or a specialized viewer

        // For now, just display the file size
        ui.label(format!("File size: {} bytes", content.len()));

        // Implement actual file content display based on the application's requirements
        // For example, if this is a node viewer, you might parse the file as a node graph
        // and display it using egui's drawing capabilities
    }
}

impl eframe::App for App {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(context, |ui| {
            context.set_pixels_per_point(self.ui_scale);

            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open...").clicked() {
                        self.open_file_dialog();
                        ui.close_kind(UiKind::Menu);
                    }
                });

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
            if let Some(error) = &self.file_error {
                ui.colored_label(egui::Color32::RED, error);
            }

            if let Some(loaded_file) = &self.loaded_file {
                ui.heading(format!("File: {}", &loaded_file.file_name));

                if let Some(content) = &self.loaded_file {
                    self.display_file_content(ui, &content.file_content);
                }
            } else {
                ui.heading("Node Viewer");
                ui.label("No file opened. Use File > Open to select a file.");
            }

            ui.with_layout(Layout::bottom_up(Align::RIGHT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
