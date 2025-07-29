### Plan to Use quick-xml for Reading XML Root Nodes

Based on the current implementation of the Node Viewer application, here's a comprehensive plan to integrate the
`quick-xml` package to read and display the root node of XML files:

#### 1. Add the quick-xml Dependency

Update the `Cargo.toml` file to include the `quick-xml` package:

```toml
[dependencies]
anyhow = "1"
egui = "0.32"
log = "0.4"
rfd = "0.15"
serde = { version = "1", features = ["derive"] }
quick-xml = "0.30"  # Add this line
```

#### 2. Create an XML Parsing Module

Create a new file `src/xml_parsing.rs` to handle XML parsing functionality:

```rust
use anyhow::Result;
use quick_xml::Reader;
use quick_xml::events::Event;

/// Attempts to extract the root element name from XML content
pub fn get_xml_root_node_name(content: &[u8]) -> Result<Option<String>> {
    let mut reader = Reader::from_reader(content);
    reader.trim_text(true);

    let mut buf = Vec::new();

    // Look for the first start element event
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                // Found the root element
                return Ok(Some(std::str::from_utf8(e.name().as_ref())?.to_string()));
            }
            Event::Eof => {
                // Reached end of file without finding a root element
                return Ok(None);
            }
            _ => {
                // Skip other events (comments, processing instructions, etc.)
                buf.clear();
            }
        }
    }
}

/// Checks if content appears to be XML
pub fn is_likely_xml(content: &[u8]) -> bool {
    if let Ok(start) = std::str::from_utf8(&content[..content.len().min(1000)]) {
        // Simple heuristic: check for XML declaration or root tag
        start.trim_start().starts_with("<?xml") ||
            start.trim_start().starts_with("<") && start.contains(">")
    } else {
        false
    }
}
```

#### 3. Update the App Module

Modify `src/app.rs` to:

1. Import the new XML parsing module
2. Add XML root node information to the UI display

```rust
// Add this import at the top
use crate::xml_parsing::{get_xml_root_node_name, is_likely_xml};

// Then modify the display_file_content method to include XML info
fn display_file_content(&self, ui: &mut egui::Ui, content: &[u8]) {
    ui.label(format!("File size: {} bytes", content.len()));

    // Add XML detection and root node display
    if is_likely_xml(content) {
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
                ui.colored_label(
                    egui::Color32::YELLOW,
                    format!("Error parsing XML: {}", e)
                );
            }
        }
        ui.separator();
    }

    // Existing code for displaying hex content
    if content.len() <= 1024 {
        ui.collapsing("File content (hex)", |ui| {
            // ... existing hex display code ...
        });
    }

    // Existing code for displaying text content
    if let Ok(mut text_content) = std::str::from_utf8(content) {
        // ... existing text display code ...
    }
}
```

#### 4. Update the lib.rs File

Modify `src/lib.rs` to include the new module:

```rust
mod app;
mod file_loading;
mod xml_parsing;  // Add this line
mod init;

pub use app::App;
```

#### 5. Testing Plan

1. Test with various XML files:
    - Well-formed XML with different root elements
    - XML with comments or processing instructions before the root
    - XML with namespaces
    - Malformed XML
    - Non-XML files

2. Test on both platforms:
    - Native desktop application
    - Web application

#### 6. Future Enhancements

After implementing the basic functionality, consider these enhancements:

1. Display more XML metadata (encoding, version, etc.)
2. Show attributes of the root node
3. Implement the node tree view as mentioned in the project overview
4. Add XML validation capabilities
5. Implement caching for large XML files to improve performance

This plan provides a straightforward approach to integrate the `quick-xml` package into the existing application to read
and display the root node of XML files, while maintaining compatibility with both native and web platforms.
