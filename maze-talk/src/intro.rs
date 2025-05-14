use eframe::egui;

#[derive(Default)]
pub struct IntroApp {}

impl eframe::App for IntroApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("\"Fast maze solving using epic bit manipulation tricks\" by bpaul");
            ui.label("Today we will be searching through 9x9 mazes really quickly!");
            
            ui.add_space(12.0);

            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 0.;
                ui.label("This ui was made with ");
                ui.hyperlink_to("egui", "https://crates.io/crates/egui");
                ui.label(", it's super cool you should all try it.");
            });
        });
    }
}
