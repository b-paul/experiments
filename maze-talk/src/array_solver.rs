use eframe::egui;

const N: usize = 4;

pub struct ArraySolverApp {
    maze: crate::maze::Maze,
}

impl Default for ArraySolverApp {
    fn default() -> Self {
        Self {
            maze: crate::maze::Maze::random(N),
        }
    }
}

impl eframe::App for ArraySolverApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Array solver");

            let grid = self.maze.grid().into_iter().map(|a| {
                a.into_iter().map(|b| match b {
                    true => egui::Color32::BLACK,
                    false => egui::Color32::WHITE,
                }).collect()
            }).collect();
            let maze_display = crate::board::BoardDisplay { size: 256., grid, n: 2 * N + 1 };
            ui.add(maze_display);

            if ui.button("Generate maze").clicked() {
                self.maze = crate::maze::Maze::random(N);
            }
        });
    }
}
