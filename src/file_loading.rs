use anyhow::{Context, anyhow};

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
