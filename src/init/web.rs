use anyhow::{Context, bail};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlCanvasElement};

const CANVAS_ID: &str = "app_canvas";
const FEEDBACK_TEXT_ID: &str = "feedback_text";

pub fn start_app() -> anyhow::Result<()> {
    eframe::WebLogger::init(log::LevelFilter::Debug).context("Failed to initialize logger")?;

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        if let Err(e) = actually_start_app(web_options).await {
            set_dom_error_message(
                "<p>The app has crashed. See the developer console for details.</p>",
            );
            panic!("Application failed: {e:?}");
        }
    });

    Ok(())
}

async fn actually_start_app(web_options: eframe::WebOptions) -> anyhow::Result<()> {
    let canvas = get_canvas_element()?;

    let start_result = eframe::WebRunner::new()
        .start(
            canvas,
            web_options,
            Box::new(|cc| Ok(Box::new(crate::App::new(cc)))),
        )
        .await;

    if let Some(feedback_element) = get_feedback_text_element() {
        match start_result {
            Ok(_) => {
                feedback_element.remove();
            }
            Err(e) => {
                bail!("Failed to start the app. Reason: {e:?}")
            }
        }
    }

    Ok(())
}

fn get_canvas_element() -> anyhow::Result<HtmlCanvasElement> {
    let window = web_sys::window().context("Failed to get the window object")?;
    let document = window
        .document()
        .context("Failed to get the document object")?;

    let Ok(canvas) = document
        .get_element_by_id(CANVAS_ID)
        .context(format!(
            "Failed to find an element with id of `{CANVAS_ID}`"
        ))?
        .dyn_into::<HtmlCanvasElement>()
    else {
        bail!("Element with `{CANVAS_ID}` was found but is not a canvas");
    };

    Ok(canvas)
}

fn get_feedback_text_element() -> Option<Element> {
    web_sys::window()?
        .document()?
        .get_element_by_id(FEEDBACK_TEXT_ID)
}

fn set_dom_error_message(message: &str) {
    if let Some(feedback_element) = get_feedback_text_element() {
        feedback_element.set_inner_html(message);
    }
}
