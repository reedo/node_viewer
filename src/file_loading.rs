/// Details of a loaded file.
#[derive(Debug)]
pub struct FileDetails {
    /// The filename.
    pub file_name: String,

    /// The bytes of the file.
    pub file_content: Vec<u8>,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn open_native_file_dialog() -> anyhow::Result<FileDetails> {
    use anyhow::{Context, anyhow};

    let Some(path) = rfd::FileDialog::new()
        .add_filter("All files", &["*"])
        .pick_file()
    else {
        return Err(anyhow!("No file selected"));
    };

    let file_content = std::fs::read(&path).context("Reading the file contents")?;

    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .context("Getting the filename from the path")?;

    Ok(FileDetails {
        file_name,
        file_content,
    })
}

#[cfg(target_arch = "wasm32")]
pub fn open_web_file_dialog() -> anyhow::Result<FileDetails> {
    use wasm_bindgen::JsCast;
    use web_sys::HtmlInputElement;

    let file_name = "test.txt".to_string();
    let file_content = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let document = web_sys::window()
        .expect("No window found")
        .document()
        .expect("No document found");

    // Create a file input element
    let input: HtmlInputElement = document
        .create_element("input")
        .expect("Failed to create input element")
        .dyn_into()
        .expect("Failed to cast to HtmlInputElement");

    // Configure the input to accept only one file
    input.set_type("file");
    input.set_multiple(false);

    // TODO - Read the file.

    Ok(FileDetails {
        file_name,
        file_content,
    })
}
