### Plan for Implementing File Opening Functionality in Node Viewer

Based on the project structure, I'll outline a comprehensive approach to implement user-selected file opening
functionality in this egui-based Rust application that targets both native platforms and the web.

#### Prerequisites

First, we need to add the necessary dependencies to `Cargo.toml`:

```toml
[dependencies]
# Add these to the existing dependencies
rfd = "0.12"  # For native file dialogs
```

##### Result

Added `rfd` 0.15 to the native dependencies.

##### End of Result

For web support, we already have `web-sys` but we'll need to ensure it has the right features:

```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
web-sys = { version = "0.3", features = [
    "File",
    "FileList",
    "FileReader",
    "HtmlInputElement",
    "Blob",
    "BlobPropertyBag",
    "Url"
] }
```

##### Result

No deviation from the plan.

##### End of Result

#### Implementation Plan

##### 1. Update the App Structure

Modify the `App` struct in `app.rs` to include file-related state:

```rust
#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    ui_scale: f32,

    // New fields for file handling
    #[serde(skip)]
    file_content: Option<Vec<u8>>,
    #[serde(skip)]
    file_name: Option<String>,
    #[serde(skip)]
    file_error: Option<String>,
}
```

###### Result

No deviation from the plan.

###### End of Result

##### 2. Add File Opening UI Elements

Update the `update` method in the `eframe::App` implementation to add file opening buttons:

```rust
impl eframe::App for App {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        // Existing top panel code...

        egui::TopBottomPanel::top("top_panel").show(context, |ui| {
            // Existing UI scale code...

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open...").clicked() {
                        self.open_file_dialog();
                        ui.close_menu();
                    }
                });

                // Existing UI scale and theme code...
            });
        });

        // Central panel for displaying file content
        egui::CentralPanel::default().show(context, |ui| {
            if let Some(error) = &self.file_error {
                ui.colored_label(egui::Color32::RED, error);
            }

            if let Some(file_name) = &self.file_name {
                ui.heading(format!("File: {}", file_name));

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

    // Existing save method...
}
```

###### Result

Had to use `egui::MenuBar::new().ui` instead of `egui::menu::bar`.
This is because the egui version has been bumped and the existing method is now deprecated.

Used inline variable names in `format!`.

###### End of Result

##### 3. Implement Platform-Specific File Opening

Add methods to the `App` implementation for handling file opening:

```rust
impl App {
    // Existing methods...

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
                    self.file_name = path.file_name()
                        .and_then(|n| n.to_str())
                        .map(|s| s.to_string());
                    self.file_content = Some(content);
                    self.file_error = None;
                }
                Err(err) => {
                    self.file_error = Some(format!("Error reading file: {}", err));
                    self.file_name = None;
                    self.file_content = None;
                }
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn open_web_file_dialog(&mut self) {
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{HtmlInputElement, FileReader, Event};
        use std::rc::Rc;
        use std::cell::RefCell;

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

        // Create a clone of self that can be moved into the closure
        let app = Rc::new(RefCell::new(self));

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

                    let app_clone = app.clone();

                    let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: Event| {
                        if let Ok(array_buffer) = reader_clone.result() {
                            let array = js_sys::Uint8Array::new(&array_buffer);
                            let content = array.to_vec();

                            let mut app = app_clone.borrow_mut();
                            app.file_name = Some(file_name.clone());
                            app.file_content = Some(content);
                            app.file_error = None;
                        }
                    }) as Box<dyn FnMut(Event)>);

                    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                    onload.forget();

                    reader.read_as_array_buffer(&file).expect("Failed to read file");
                }
            }
        }) as Box<dyn FnMut(Event)>);

        input.set_onchange(Some(onchange.as_ref().unchecked_ref()));
        onchange.forget();

        // Programmatically click the input to open the file dialog
        input.click();
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
```

###### Result

No deviation from the plan.

###### End of Result

##### 4. Handling Different File Types

Depending on what kind of "nodes" this application is intended to view, you'll need to implement specific parsing and
rendering logic in the `display_file_content` method. This could involve:

- Parsing JSON, XML, or other structured data formats
- Building a visual representation of nodes and edges
- Implementing navigation and interaction with the node structure

#### Additional Considerations

1. **Error Handling**: Robust error handling for file operations, especially for web where browser permissions might be
   an issue.

2. **File Type Filtering**: Add appropriate file filters to the file dialog based on the supported file types.

3. **Progress Indicators**: For large files, implement progress indicators during loading.

4. **Recent Files**: Consider adding a "Recent Files" list in the File menu.

5. **Drag and Drop**: Implement drag and drop support for files.

6. **File Saving**: If the application needs to save modified files, implement save functionality as well.

#### Testing

Test the implementation on both native and web platforms:

1. For native: `cargo run --release`
2. For web: `trunk serve` and open `http://127.0.0.1:8080/index.html#dev`

This plan provides a comprehensive approach to implementing file opening functionality in the Node Viewer application,
taking into account the cross-platform nature of the project and the specific requirements of egui and eframe.
