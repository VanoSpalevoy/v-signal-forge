use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Signal Forge",
        eframe::NativeOptions::default(),
        Box::new(|_creation_context| Ok(Box::<SignalForgeApp>::default())),
    )
}

struct SignalForgeApp {
    name: String,
    age: u64,
}

impl Default for SignalForgeApp {
    fn default() -> Self {
        Self {
            name: "Ivan".to_owned(),
            age: 20,
        }
    }
}

impl eframe::App for SignalForgeApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Signal Forge");
        ui.text_edit_singleline(&mut self.name);
        ui.add(
            egui::DragValue::new(&mut self.age)
                .range(0..=120)
                .suffix(" years"),
        );
        if ui.button("Increment").clicked() {
            self.age += 1;
        }

        ui.label(format!("{} is {}", self.name, self.age));
    }

}
