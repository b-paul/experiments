use eframe::egui;

#[derive(Default)]
pub struct BitboardDemoApp {}

impl eframe::App for BitboardDemoApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("todo");
        });
    }
}
