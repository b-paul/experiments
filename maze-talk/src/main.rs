use eframe::egui;

use maze_talk::*;

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
    #[default]
    IntroPage,
    ArraySolver,
    BitboardDemo,
    BitboardSolver,
}

struct App {
    slide: Slide,
    show_settings: bool,

    gui_scale: f32,

    intro_page: intro::IntroApp,
    array_solver: array_solver::ArraySolverApp,
    bitboard_demo: bitboard_demo::BitboardDemoApp,
    bitboard_solver: bitboard_solver::BitboardSolverApp,
}

impl Default for App {
    fn default() -> Self {
        Self {
            slide: Default::default(),
            show_settings: Default::default(),
            gui_scale: 1.,
            intro_page: Default::default(),
            array_solver: Default::default(),
            bitboard_demo: Default::default(),
            bitboard_solver: Default::default(),
        }
    }
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
            Slide::BitboardSolver => self.bitboard_solver.update(ctx, frame),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let default_styles = egui::Style::default().text_styles;
        ctx.style_mut(|style| {
            for (key, font_id) in style.text_styles.iter_mut() {
                font_id.size = default_styles[key].size * self.gui_scale;
            }
        });

        if self.show_settings {
            egui::Window::new("Settings").show(ctx, |ui| {
                ui.add(egui::Slider::new(&mut self.gui_scale, 0.5..=3.0).text("Gui scale"));
            });
        }

        self.array_solver.gui_scale = self.gui_scale;
        self.bitboard_demo.gui_scale = self.gui_scale;
        self.bitboard_solver.gui_scale = self.gui_scale;

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
                if ui
                    .selectable_label(self.slide == Slide::BitboardSolver, "Bitboard solver")
                    .clicked()
                {
                    self.slide = Slide::BitboardSolver;
                }

                ui.separator();

                if ui
                    .selectable_label(self.show_settings, "Settings")
                    .clicked()
                {
                    self.show_settings = !self.show_settings;
                }
            })
        });

        self.show_slide(ctx, frame);
    }
}
