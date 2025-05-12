use eframe::egui;

#[derive(Default)]
pub struct BitboardDemoApp {}

impl eframe::App for BitboardDemoApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("hi");
            let square = egui::Rect {
                min: egui::Pos2::new(50., 50.),
                max: egui::Pos2::new(150., 150.),
            };
            let shape = egui::Shape::rect_filled(square, 0, egui::Color32::WHITE);
            ui.painter().add(shape);
        });
    }
}
