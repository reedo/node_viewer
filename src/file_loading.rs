use anyhow::anyhow;

/// Details of a loaded file.
#[derive(Debug, Clone)]
pub struct FileDetails {
    /// The filename.
    pub file_name: String,

    /// The bytes of the file.
    pub file_content: Vec<u8>,
}

/// State of file loading operation
#[derive(Debug, Clone)]
pub enum FileLoadingState {
    Idle,
    Loading,
    Loaded(FileDetails),
    Error(String),
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
pub async fn open_web_file_dialog_async() -> anyhow::Result<FileDetails> {
    let file_handle = rfd::AsyncFileDialog::new()
        .add_filter("All files", &["*"])
        .pick_file()
        .await
        .ok_or_else(|| anyhow!("No file selected"))?;

    let file_name = file_handle.file_name();
    let file_content = file_handle.read().await;

    Ok(FileDetails {
        file_name,
        file_content,
    })
}
