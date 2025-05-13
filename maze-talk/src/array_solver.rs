use eframe::egui;

use std::collections::{BTreeSet, VecDeque};

const N: usize = 4;

pub struct ArraySolverApp {
    maze: crate::maze::Maze,
    grid: Vec<Vec<bool>>,
    visited: Vec<Vec<bool>>,
    // The last point we checked
    next: Option<(usize, usize)>,

    // (point, distance)
    queue: VecDeque<((usize, usize), usize)>,
}

impl ArraySolverApp {
    fn reset(&mut self) {
        self.maze = crate::maze::Maze::random(N);
        self.grid = self.maze.grid();
        self.visited = self
            .maze
            .grid()
            .into_iter()
            .map(|r| r.into_iter().map(|_| false).collect())
            .collect();
        self.queue = VecDeque::new();
        self.queue.push_back(((1, 0), 0));
    }

    fn colour_grid(&self) -> Vec<Vec<egui::Color32>> {
        self.grid
            .iter()
            .enumerate()
            .map(|(y, a)| {
                a.iter()
                    .enumerate()
                    .map(
                        |(x, b)| match (b, self.visited[y][x], Some((x, y)) == self.next) {
                            (true, _, _) => egui::Color32::BLACK,
                            (_, _, true) => egui::Color32::GREEN,
                            (false, true, _) => egui::Color32::BLUE,
                            (false, false, _) => egui::Color32::WHITE,
                        },
                    )
                    .collect()
            })
            .collect()
    }

    fn step(&mut self) {
        let Some((p, dist)) = self.queue.pop_front() else {
            return;
        };
        self.visited[p.1][p.0] = true;

        if self.visited[2 * N][2 * N - 1] {
            self.next = None;
            self.queue = VecDeque::new();
            return;
        }

        let queued = self
            .queue
            .clone()
            .into_iter()
            .map(|(p, _)| p)
            .collect::<BTreeSet<_>>();
        for next in super::maze::DIRS
            .into_iter()
            .flat_map(|d| d.translate(p, 2 * N + 1))
            .filter(|&(x, y)| !self.grid[y][x] && !queued.contains(&(x, y)) && !self.visited[y][x])
        {
            self.queue.push_back((next, dist + 1));
        }
        self.next = self.queue.front().map(|&(p, _)| p);
    }
}

impl Default for ArraySolverApp {
    fn default() -> Self {
        let maze = crate::maze::Maze::random(N);
        let mut app = Self {
            maze,
            grid: Vec::new(),
            visited: Vec::new(),
            next: Some((1, 0)),
            queue: VecDeque::new(),
        };
        app.reset();
        app
    }
}

impl eframe::App for ArraySolverApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Array solver");

            let grid = self.colour_grid();
            let maze_display = crate::board::BoardDisplay {
                size: 256.,
                grid,
                n: 2 * N + 1,
            };
            ui.add(maze_display);

            if ui.button("Generate maze").clicked() {
                self.reset();
            }
            if ui.button("Step").clicked() {
                self.step();
            }
        });
    }
}
