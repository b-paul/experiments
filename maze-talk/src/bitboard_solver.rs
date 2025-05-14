use eframe::egui;

const N: usize = 4;

pub struct BitboardSolverApp {
    pub gui_scale: f32,

    maze: crate::maze::Maze,
    // 7 wide 9 high grid.
    grid: u64,
    visited: u64,

    shifts: usize,
    ors: usize,
    ands: usize,

    auto: bool,

    steps: usize,
    len: Option<usize>,

    start_time: f64,
}

impl BitboardSolverApp {
    fn reset(&mut self, time: f64) {
        self.maze = crate::maze::Maze::random(N);
        self.grid = 0;
        self.visited = 0;
        self.shifts = 0;
        self.ors = 0;
        self.ands = 0;

        for (y, row) in self.maze.grid().into_iter().enumerate() {
            for (x, b) in row.into_iter().enumerate() {
                if x == 0 || x == 8 {
                    continue;
                }
                if b {
                    // should probably put this indexing stuff in a function or something
                    self.grid |= 1 << (7 * (8 - y) + (6 - (x - 1)));
                }
            }
        }

        self.visited |= 1 << 62;

        self.steps = 0;
        self.len = None;
        self.start_time = time;
    }

    fn colour_grid(&self) -> Vec<Vec<egui::Color32>> {
        (0..9)
            .map(|y| {
                (0..9)
                    .map(|x| {
                        if x == 0 || x == 8 {
                            egui::Color32::BLACK
                        } else {
                            let shift = 7 * (8 - y) + (6 - (x - 1));
                            if self.grid & 1 << shift != 0 {
                                egui::Color32::BLACK
                            } else if self.visited & 1 << shift != 0 {
                                egui::Color32::BLUE
                            } else {
                                egui::Color32::WHITE
                            }
                        }
                    })
                    .collect()
            })
            .collect()
    }

    fn done(&self) -> bool {
        self.visited & (1 << 0) != 0
    }

    fn step(&mut self) {
        if self.done() {
            return;
        }

        let v = self.visited;
        let m = self.grid;
        const L: u64 = 0b100000010000001000000100000010000001000000100000010000001000000;
        const R: u64 = 0b000000100000010000001000000100000010000001000000100000010000001;

        self.shifts += 4;
        self.ors += 4;
        self.ands += 4;
        self.visited = (v | (!L & v) << 1 | (!R & v) >> 1 | v << 7 | v >> 7) & !m & !(1 << 63);
        self.steps += 1;

        if self.done() {
            self.len = Some(self.steps);
        }

        println!("{:064b}", self.visited);
    }
}

impl Default for BitboardSolverApp {
    fn default() -> Self {
        let maze = super::maze::Maze::random(N);
        let mut app = Self {
            gui_scale: 1.,
            maze,
            grid: 0,
            visited: 0,
            shifts: 0,
            ors: 0,
            ands: 0,
            auto: false,
            steps: 0,
            len: None,
            start_time: 0.,
        };
        app.reset(0.);
        app
    }
}

impl eframe::App for BitboardSolverApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        let time = ctx.input(|i| i.time);

        egui::CentralPanel::default().show(ctx, |ui| {
            let grid = self.colour_grid();
            let maze_display = crate::board::BoardDisplay {
                size: 256. * self.gui_scale,
                grid,
                n: 2 * N + 1,
            };
            ui.add(maze_display);

            if ui.button("Generate maze").clicked() {
                self.reset(time);
            }
            let mut stepped = false;
            if ui.button("Step").clicked() {
                self.step();
                stepped = true;
            }

            ui.checkbox(&mut self.auto, "Autosolve");
            if self.auto && !stepped && !self.done() && time - self.start_time > 0.05 {
                self.step();
                self.start_time = time;
            }

            ui.label(format!("Shift operations: {}", self.shifts));
            ui.label(format!("Or operations: {}", self.ors));
            ui.label(format!("And operationsg: {}", self.ands));

            if let Some(dist) = self.len {
                ui.label(format!("Length: {dist}"));
            }
        });
    }
}
