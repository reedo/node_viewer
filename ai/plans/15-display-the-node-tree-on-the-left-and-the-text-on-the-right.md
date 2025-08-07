### Plan to Re-arrange the Main UI Panel

Based on the current implementation in the Node Viewer application, I'll outline a plan to modify the UI layout
according to the requirements: creating two equal-sized panels with a vertical split, placing the XML elements list on
the left and the text content on the right, while removing the hex view.

### Current Implementation

The application currently:

1. Uses a single central panel to display all content
2. Shows file information, XML elements list, hex view, and text content in a vertical layout
3. Places the XML elements and text content in collapsible sections

### Modification Plan

#### 1. Modify the Central Panel Layout

Replace the current central panel implementation with a split panel layout using `egui::SidePanel` for the left side and
`egui::CentralPanel` for the right side:

```rust
fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
    // Top panel with menu remains unchanged
    egui::TopBottomPanel::top("top_panel").show(context, |ui| {
        // Existing menu code...
    });

    match &self.file_loading_state {
        FileLoadingState::Idle => {
            // Show welcome message in central panel
            egui::CentralPanel::default().show(context, |ui| {
                ui.heading("Node Viewer");
                ui.label("No file opened. Use File > Open to select a file.");
            });
        }
        FileLoadingState::Loading => {
            // Show loading message in central panel
            egui::CentralPanel::default().show(context, |ui| {
                ui.heading("Loading file...");
                ui.spinner();
                ui.label("Please wait while the file is being loaded.");
            });
        }
        FileLoadingState::Loaded(file_details) => {
            // Show file name in a top panel
            egui::TopBottomPanel::bottom("file_info").show(context, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(format!("File: {}", &file_details.file_name));
                    ui.label(format!("Size: {} bytes", file_details.file_content.len()));
                });
            });

            // Left panel for XML elements
            egui::SidePanel::left("xml_elements_panel")
                .resizable(true)
                .default_width(context.available_rect().width() / 2.0) // Set to half the available width
                .show(context, |ui| {
                    self.display_xml_elements(ui, &file_details.file_content);
                });

            // Right panel for text content
            egui::CentralPanel::default().show(context, |ui| {
                self.display_text_content(ui, &file_details.file_content);
            });
        }
        FileLoadingState::Error(error) => {
            // Show error message in central panel
            egui::CentralPanel::default().show(context, |ui| {
                ui.heading("Error");
                ui.colored_label(egui::Color32::RED, error);
                ui.separator();
                ui.label("Use File > Open to try loading another file.");
            });
        }
    }
}
```

#### 2. Split the `display_file_content` Method

Divide the current `display_file_content` method into two separate methods:

```rust
// Method for displaying XML elements in the left panel
fn display_xml_elements(&self, ui: &mut egui::Ui, content: &[u8]) {
    ui.heading("All XML Elements");

    if is_likely_xml(content) {
        match get_all_xml_element_names(content) {
            Ok(element_names) => {
                if element_names.is_empty() {
                    ui.label("No elements found in the XML document.");
                } else {
                    ui.label(format!("Found {} elements:", element_names.len()));

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (index, name) in element_names.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}.", index + 1));
                                ui.label(name);
                            });
                        }
                    });
                }
            }
            Err(e) => {
                ui.colored_label(
                    egui::Color32::YELLOW,
                    format!("Error extracting element names: {e}"),
                );
            }
        }
    } else {
        ui.label("Not an XML file or unable to parse XML elements.");
    }
}

// Method for displaying text content in the right panel
fn display_text_content(&self, ui: &mut egui::Ui, content: &[u8]) {
    ui.heading("File Content");

    // Try to display as text if it looks like text
    if let Ok(text_content) = std::str::from_utf8(content)
        && text_content
        .chars()
        .all(|c| c.is_ascii() && !c.is_control() || c.is_whitespace())
    {
        // Use a scrollable text editor that fills the entire panel
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut text_content.to_string())
                        .desired_width(ui.available_width())
                        .desired_height(ui.available_height())
                        .code_editor(),
                );
            });
    } else {
        ui.label("File content cannot be displayed as text.");
    }
}
```

#### 3. Remove the Hex View

The hex view is completely removed from the implementation as requested. The code that previously displayed the hex
view (lines 136-151 in app.rs) will not be included in the new methods.

#### 4. UI Improvements

1. Make both panels fill their available space:
    - Use `auto_shrink([false, false])` for scroll areas
    - Set desired width and height to available space

2. Make the divider between panels resizable:
    - Use `resizable(true)` on the `SidePanel`
    - Set a default width of half the screen

3. Improve the display of file information:
    - Move file name and size to a bottom panel for better visibility

### Implementation Steps

1. Modify the `update` method in `app.rs` to use the split panel layout
2. Create the two new methods `display_xml_elements` and `display_text_content`
3. Remove the existing `display_file_content` method
4. Test the changes with various XML files to ensure proper display
5. Adjust panel sizes and spacing as needed for optimal user experience

This implementation maintains all the required functionality while providing a cleaner, more organized UI that follows
the specified layout requirements.
