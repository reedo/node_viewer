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
    use anyhow::anyhow;

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
    use wasm_bindgen::JsCast;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Event, FileReader, HtmlInputElement};

    let document = web_sys::window()
        .ok_or_else(|| anyhow::anyhow!("No window found"))?
        .document()
        .ok_or_else(|| anyhow::anyhow!("No document found"))?;

    // Create a file input element
    let input: HtmlInputElement = document
        .create_element("input")
        .map_err(|_| anyhow::anyhow!("Failed to create input element"))?
        .dyn_into()
        .map_err(|_| anyhow::anyhow!("Failed to cast to HtmlInputElement"))?;

    input.set_type("file");
    input.set_multiple(false);

    // Create a promise for file selection and reading
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let resolve = resolve.clone();
        let reject = reject.clone();

        let resolve_clone = resolve.clone();
        let reject_clone = reject.clone();

        let closure = Closure::wrap(Box::new(move |event: Event| {
            let input = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                .unwrap();

            let files = input.files().unwrap();

            if files.length() == 0 {
                reject_clone
                    .call1(&JsValue::NULL, &JsValue::from_str("No file selected"))
                    .unwrap();
                return;
            }

            let file = files.get(0).unwrap();
            let file_name = file.name();

            let file_reader = FileReader::new().unwrap();
            let file_reader_clone = file_reader.clone();

            // Handle successful file reading
            let resolve_clone2 = resolve_clone.clone();
            let file_name_clone = file_name.clone();
            let onload_closure = Closure::wrap(Box::new(move |_event: Event| {
                let array_buffer = file_reader_clone.result().unwrap();
                let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                let mut file_content = vec![0; uint8_array.length() as usize];
                uint8_array.copy_to(&mut file_content);

                // Create a JavaScript object to return both filename and content
                let result = js_sys::Object::new();
                js_sys::Reflect::set(&result, &"fileName".into(), &file_name_clone.clone().into()).unwrap();
                js_sys::Reflect::set(
                    &result,
                    &"content".into(),
                    &serde_wasm_bindgen::to_value(&file_content).unwrap(),
                )
                .unwrap();

                resolve_clone2.call1(&JsValue::NULL, &result).unwrap();
            }) as Box<dyn FnMut(_)>);

            file_reader.set_onload(Some(onload_closure.as_ref().unchecked_ref()));
            onload_closure.forget();

            // Handle file reading errors
            let reject_clone2 = reject_clone.clone();
            let onerror_closure = Closure::wrap(Box::new(move |_event: Event| {
                reject_clone2
                    .call1(&JsValue::NULL, &JsValue::from_str("Failed to read file"))
                    .unwrap();
            }) as Box<dyn FnMut(_)>);

            file_reader.set_onerror(Some(onerror_closure.as_ref().unchecked_ref()));
            onerror_closure.forget();

            // Start reading the file as ArrayBuffer
            file_reader.read_as_array_buffer(&file).unwrap();
        }) as Box<dyn FnMut(_)>);

        input.set_onchange(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        // Trigger the file dialog
        input.click();
    });

    // Wait for the promise to resolve
    let result = JsFuture::from(promise)
        .await
        .map_err(|e| anyhow::anyhow!("File selection failed: {:?}", e))?;

    // Extract filename and content from the result
    let file_name: String = js_sys::Reflect::get(&result, &"fileName".into())
        .map_err(|_| anyhow::anyhow!("Failed to get filename"))?
        .as_string()
        .ok_or_else(|| anyhow::anyhow!("Filename is not a string"))?;

    let content_value = js_sys::Reflect::get(&result, &"content".into())
        .map_err(|_| anyhow::anyhow!("Failed to get file content"))?;

    let file_content: Vec<u8> = serde_wasm_bindgen::from_value(content_value)
        .map_err(|e| anyhow::anyhow!("Failed to deserialize file content: {:?}", e))?;

    Ok(FileDetails {
        file_name,
        file_content,
    })
}
