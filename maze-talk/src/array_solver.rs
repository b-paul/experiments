use eframe::egui;

#[derive(Default)]
pub struct ArraySolverApp {
    maze: crate::maze::Maze<17>,
}

impl eframe::App for ArraySolverApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Array solver");

            let grid = self.maze.grid().map(|a| {
                a.map(|b| match b {
                    true => egui::Color32::BLACK,
                    false => egui::Color32::WHITE,
                })
            });
            let maze_display = crate::board::BoardDisplay { size: 256., grid };
            ui.add(maze_display);
        });
    }
}
