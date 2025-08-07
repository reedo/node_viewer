### Plan to Display All XML Element Names

Based on the current implementation, the app already parses XML files and displays the root element name. To extend this
functionality to display a list of all element names in the document, I'll outline a comprehensive plan:

### 1. Modify XML Parsing Module

First, we need to enhance the XML parsing functionality in `src/xml_parsing.rs`:

```rust
/// Extracts all element names from an XML document
pub fn get_all_xml_element_names(content: &[u8]) -> Result<Vec<String>> {
    let mut reader = Reader::from_reader(content);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut element_names = Vec::new();

    // Iterate through all XML events
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) | Event::Empty(e) => {
                // Found an element, add its name to the list
                element_names.push(std::str::from_utf8(e.name().as_ref())?.to_string());
            }
            Event::Eof => {
                // Reached end of file
                break;
            }
            _ => {
                // Skip other events (comments, processing instructions, etc.)
                buf.clear();
            }
        }
    }

    Ok(element_names)
}
```

### 2. Update UI Display in App Module

Next, modify the `display_file_content` method in `src/app.rs` to use the new function and display all element names:

```rust
fn display_file_content(&self, ui: &mut egui::Ui, content: &[u8]) {
    ui.label(format!("File size: {} bytes", content.len()));

    if is_likely_xml(content) {
        // Display root node name (existing functionality)
        match get_xml_root_node_name(content) {
            Ok(Some(root_name)) => {
                ui.horizontal(|ui| {
                    ui.label("XML Root Node:");
                    ui.strong(root_name);
                });
            }
            Ok(None) => {
                ui.label("XML file detected, but no root node found.");
            }
            Err(e) => {
                ui.colored_label(egui::Color32::YELLOW, format!("Error parsing XML: {e}"));
            }
        }

        // Display all element names (new functionality)
        match get_all_xml_element_names(content) {
            Ok(element_names) => {
                ui.collapsing("All XML Elements", |ui| {
                    if element_names.is_empty() {
                        ui.label("No elements found in the XML document.");
                    } else {
                        ui.label(format!("Found {} elements:", element_names.len()));

                        // Create a scrollable list for large XML files
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (index, name) in element_names.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.label(format!("{}.", index + 1));
                                    ui.label(name);
                                });
                            }
                        });
                    }
                });
            }
            Err(e) => {
                ui.colored_label(egui::Color32::YELLOW,
                                 format!("Error extracting element names: {e}"));
            }
        }

        ui.separator();
    }

    // Rest of the existing display code...
}
```

### 3. Optional Enhancements

1. **Element Frequency Counter**:
    - Count occurrences of each element name
    - Display a summary like "div: 15, span: 8, p: 12"

2. **Element Filtering**:
    - Add a search box to filter elements by name
    - Include checkboxes to show/hide specific element types

3. **Element Attributes**:
    - Extend the parsing to capture element attributes
    - Display attributes alongside element names

4. **Element Hierarchy Indicators**:
    - Add indentation or breadcrumb indicators to show parent-child relationships
    - Display the path to each element (e.g., "html > body > div > p")

### 4. Testing

1. Test with various XML files of different sizes and complexities
2. Ensure performance remains acceptable for large XML files
3. Verify correct handling of XML namespaces
4. Test edge cases like empty documents, malformed XML, etc.

### 5. Implementation Steps

1. Implement the new `get_all_xml_element_names` function in `xml_parsing.rs`
2. Update the `display_file_content` method in `app.rs`
3. Test with sample XML files
4. Implement optional enhancements as needed
5. Add error handling for edge cases

This plan extends the current functionality while maintaining the existing UI patterns and code structure, making it a
natural evolution of the application.
