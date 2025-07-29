use anyhow::bail;

pub fn start_app() -> anyhow::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    let start_result = eframe::run_native(
        "Node Viewer",
        native_options,
        Box::new(|cc| Ok(Box::new(crate::App::new(cc)))),
    );

    if start_result.is_ok() {
        Ok(())
    } else {
        bail!("Failed to start app")
    }
}
