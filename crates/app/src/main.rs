mod editor;

fn main() -> eframe::Result {
    eframe::run_native(
        "Signal Forge",
        eframe::NativeOptions::default(),
        Box::new(|_creation_context| Ok(Box::<editor::SignalForgeApp>::default())),
    )
}
