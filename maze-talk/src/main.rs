use eframe::egui;

mod array_solver;
mod bitboard_demo;
mod board;
mod intro;
mod maze;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Maze talk",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

#[derive(Default, PartialEq, Eq)]
enum Slide {
    IntroPage,
    ArraySolver,
    #[default]
    BitboardDemo,
}

#[derive(Default)]
struct App {
    slide: Slide,

    intro_page: intro::IntroApp,
    array_solver: array_solver::ArraySolverApp,
    bitboard_demo: bitboard_demo::BitboardDemoApp,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn show_slide(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        use eframe::App;
        match self.slide {
            Slide::IntroPage => self.intro_page.update(ctx, frame),
            Slide::ArraySolver => self.array_solver.update(ctx, frame),
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
                if ui
                    .selectable_label(self.slide == Slide::IntroPage, "Intro")
                    .clicked()
                {
                    self.slide = Slide::IntroPage;
                }
                if ui
                    .selectable_label(self.slide == Slide::ArraySolver, "Array solver")
                    .clicked()
                {
                    self.slide = Slide::ArraySolver;
                }
                if ui
                    .selectable_label(self.slide == Slide::BitboardDemo, "Bitboard demo")
                    .clicked()
                {
                    self.slide = Slide::BitboardDemo;
                }
            })
        });

        self.show_slide(ctx, frame);
    }
}
