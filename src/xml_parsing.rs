use anyhow::Result;
use quick_xml::Reader;
use quick_xml::events::Event;

/// Checks if content appears to be XML.
pub fn is_likely_xml(content: &[u8]) -> bool {
    if let Ok(start) = std::str::from_utf8(&content[..content.len().min(1000)]) {
        // Simple heuristic: check for XML declaration or root tag.
        start.trim_start().starts_with("<?xml")
            || start.trim_start().starts_with("<") && start.contains(">")
    } else {
        false
    }
}

pub fn get_all_xml_element_names(content: &[u8]) -> Result<Vec<String>> {
    let mut reader = Reader::from_reader(content);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut element_names = Vec::new();

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
