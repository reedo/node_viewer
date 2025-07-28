#[cfg(not(target_arch = "wasm32"))]
use crate::file_loading::open_native_file_dialog;

use crate::file_loading::FileLoadingState;
use egui::{Align, Layout, UiKind};
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use std::sync::mpsc;

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    ui_scale: f32,

    #[serde(skip)]
    file_loading_state: FileLoadingState,

    #[serde(skip)]
    #[cfg(target_arch = "wasm32")]
    file_receiver: Option<mpsc::Receiver<FileLoadingState>>,
}

impl Default for App {
    fn default() -> Self {
        App {
            ui_scale: 1.0,
            file_loading_state: FileLoadingState::Idle,
            #[cfg(target_arch = "wasm32")]
            file_receiver: None,
        }
    }
}

impl App {
    pub fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state.
        if let Some(storage) = _creation_context.storage {
            let mut app: App = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();

            // Initialize the receiver for WASM builds
            #[cfg(target_arch = "wasm32")]
            {
                let (_, receiver) = mpsc::channel();
                app.file_receiver = Some(receiver);
            }

            return app;
        }

        let mut app = Self::default();

        #[cfg(target_arch = "wasm32")]
        {
            let (_, receiver) = mpsc::channel();
            app.file_receiver = Some(receiver);
        }

        app
    }

    fn open_file_dialog(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            match open_native_file_dialog() {
                Ok(file_details) => {
                    self.file_loading_state = FileLoadingState::Loaded(file_details);
                }
                Err(e) => {
                    self.file_loading_state =
                        FileLoadingState::Error(format!("Error opening file: {e}"));
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            use crate::file_loading::open_web_file_dialog_async;
            use wasm_bindgen_futures::spawn_local;

            // Set loading state
            self.file_loading_state = FileLoadingState::Loading;

            // Create a new channel for this file loading operation
            let (sender, receiver) = mpsc::channel();
            self.file_receiver = Some(receiver);

            // Spawn the async file loading operation
            spawn_local(async move {
                let result = match open_web_file_dialog_async().await {
                    Ok(file_details) => FileLoadingState::Loaded(file_details),
                    Err(e) => FileLoadingState::Error(format!("Error opening file: {e}")),
                };

                // Send the result back to the main thread
                if let Err(e) = sender.send(result) {
                    log::error!("Failed to send file loading result: {e}");
                }
            });
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn check_file_loading_result(&mut self) {
        if let Some(receiver) = &self.file_receiver {
            // Check if there's a new result from the async operation
            if let Ok(new_state) = receiver.try_recv() {
                self.file_loading_state = new_state;
            }
        }
    }

    fn display_file_content(&self, ui: &mut egui::Ui, content: &[u8]) {
        ui.label(format!("File size: {} bytes", content.len()));

        // Display first few bytes as hex if it's a small file
        if content.len() <= 1024 {
            ui.collapsing("File content (hex)", |ui| {
                let hex_string = content
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" ");

                ui.add(
                    egui::TextEdit::multiline(&mut hex_string.as_str())
                        .desired_width(f32::INFINITY)
                        .code_editor(),
                );
            });
        }

        // Try to display as text if it looks like text
        if let Ok(mut text_content) = std::str::from_utf8(content) {
            if text_content
                .chars()
                .all(|c| c.is_ascii() && !c.is_control() || c.is_whitespace())
            {
                ui.collapsing("File content (text)", |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut text_content)
                            .desired_width(f32::INFINITY)
                            .code_editor(),
                    );
                });
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for async file loading results on WASM
        #[cfg(target_arch = "wasm32")]
        self.check_file_loading_result();

        egui::TopBottomPanel::top("top_panel").show(context, |ui| {
            context.set_pixels_per_point(self.ui_scale);

            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    let open_button = ui.add_enabled(
                        !matches!(self.file_loading_state, FileLoadingState::Loading),
                        egui::Button::new("Open..."),
                    );

                    if open_button.clicked() {
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
            match &self.file_loading_state {
                FileLoadingState::Idle => {
                    ui.heading("Node Viewer");
                    ui.label("No file opened. Use File > Open to select a file.");
                }
                FileLoadingState::Loading => {
                    ui.heading("Loading file...");
                    ui.spinner();
                    ui.label("Please wait while the file is being loaded.");
                }
                FileLoadingState::Loaded(file_details) => {
                    ui.heading(format!("File: {}", &file_details.file_name));
                    self.display_file_content(ui, &file_details.file_content);
                }
                FileLoadingState::Error(error) => {
                    ui.heading("Error");
                    ui.colored_label(egui::Color32::RED, error);
                    ui.separator();
                    ui.label("Use File > Open to try loading another file.");
                }
            }

            ui.with_layout(Layout::bottom_up(Align::RIGHT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });

        // Request repaint to keep checking for async results
        #[cfg(target_arch = "wasm32")]
        context.request_repaint();
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
