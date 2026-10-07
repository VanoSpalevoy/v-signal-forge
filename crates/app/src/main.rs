mod editor;

fn main() -> eframe::Result {
    eframe::run_native(
        "Signal Forge",
        eframe::NativeOptions::default(),
        Box::new(|creation_context| {
            creation_context
                .egui_ctx
                .set_visuals(eframe::egui::Visuals::dark());
            Ok(Box::<editor::SignalForgeApp>::default())
        }),
    )
}
