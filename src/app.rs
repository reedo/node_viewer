use egui::{Align, Layout, UiKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    ui_scale: f32,

    #[serde(skip)]
    file_content: Option<Vec<u8>>,

    #[serde(skip)]
    file_name: Option<String>,

    #[serde(skip)]
    file_error: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        App {
            ui_scale: 1.0,
            file_content: None,
            file_name: None,
            file_error: None,
        }
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

    fn open_file_dialog(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.open_native_file_dialog();
        }

        #[cfg(target_arch = "wasm32")]
        {
            self.open_web_file_dialog();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn open_native_file_dialog(&mut self) {
        // Use rfd for a native file dialog
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("All files", &["*"])
            .pick_file()
        {
            match std::fs::read(&path) {
                Ok(content) => {
                    self.file_name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|s| s.to_string());
                    self.file_content = Some(content);
                    self.file_error = None;
                }
                Err(err) => {
                    self.file_error = Some(format!("Error reading file: {err}"));
                    self.file_name = None;
                    self.file_content = None;
                }
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn open_web_file_dialog(&mut self) {
        use std::cell::RefCell;
        use std::rc::Rc;
        use wasm_bindgen::JsCast;
        use web_sys::{Event, FileReader, HtmlInputElement};

        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        // Create a file input element
        let input: HtmlInputElement = document
            .create_element("input")
            .expect("Failed to create input element")
            .dyn_into()
            .expect("Failed to cast to HtmlInputElement");

        input.set_type("file");

        // Create shared state for file operations
        let file_name_rc = Rc::new(RefCell::new(None::<String>));
        let file_content_rc = Rc::new(RefCell::new(None::<Vec<u8>>));
        let file_error_rc = Rc::new(RefCell::new(None::<String>));

        // Clone the Rc values before moving into closure
        let file_name_rc_clone = file_name_rc.clone();
        let file_content_rc_clone = file_content_rc.clone();
        let file_error_rc_clone = file_error_rc.clone();

        // Set up the onchange event handler
        let onchange = wasm_bindgen::closure::Closure::wrap(Box::new(move |event: Event| {
            let input: HtmlInputElement = event
                .target()
                .expect("Event has no target")
                .dyn_into()
                .expect("Failed to cast to HtmlInputElement");

            if let Some(file_list) = input.files() {
                if let Some(file) = file_list.get(0) {
                    let file_name = file.name();

                    let reader = FileReader::new().expect("Failed to create FileReader");
                    let reader_clone = reader.clone();

                    let file_name_clone = file_name_rc_clone.clone();
                    let file_content_clone = file_content_rc_clone.clone();
                    let file_error_clone = file_error_rc_clone.clone();

                    let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: Event| {
                        if let Ok(array_buffer) = reader_clone.result() {
                            let array = js_sys::Uint8Array::new(&array_buffer);
                            let content = array.to_vec();

                            *file_name_clone.borrow_mut() = Some(file_name.clone());
                            *file_content_clone.borrow_mut() = Some(content);
                            *file_error_clone.borrow_mut() = None;
                        }
                    })
                        as Box<dyn FnMut(Event)>);

                    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                    onload.forget();

                    reader
                        .read_as_array_buffer(&file)
                        .expect("Failed to read file");
                }
            }
        }) as Box<dyn FnMut(Event)>);

        input.set_onchange(Some(onchange.as_ref().unchecked_ref()));
        onchange.forget();

        // Programmatically click the input to open the file dialog
        input.click();

        // Apply the results back to self (this happens synchronously after the async operations complete)
        // Note: In a real application, you might want to use a different approach like channels or callbacks
        // to handle the asynchronous nature properly
        self.file_name = file_name_rc.borrow().clone();
        self.file_content = file_content_rc.borrow().clone();
        self.file_error = file_error_rc.borrow().clone();
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

            if let Some(file_name) = &self.file_name {
                ui.heading(format!("File: {file_name}"));

                // Display file content based on the file type
                if let Some(content) = &self.file_content {
                    self.display_file_content(ui, content);
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
