use eframe::egui;

mod bitboard_demo;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Bitboard presentation",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

#[derive(Default)]
enum Slide {
    #[default]
    BitboardDemo,
}

#[derive(Default)]
struct App {
    slide: Slide,

    bitboard_demo: bitboard_demo::BitboardDemoApp,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn show_slide(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        use eframe::App;
        match self.slide {
            Slide::BitboardDemo => self.bitboard_demo.update(ctx, frame),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::widgets::global_theme_preference_switch(ui);
                ui.separator();
            })
        });

        self.show_slide(ctx, frame);
    }
}
