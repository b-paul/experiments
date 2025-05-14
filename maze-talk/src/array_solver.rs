use eframe::egui;

use std::collections::{BTreeSet, VecDeque};

const N: usize = 4;

pub struct ArraySolverApp {
    pub gui_scale: f32,

    maze: crate::maze::Maze,
    grid: Vec<Vec<bool>>,
    visited: Vec<Vec<bool>>,
    enqueued: BTreeSet<(usize, usize)>,
    // The last point we checked
    next: Option<(usize, usize)>,

    // (point, distance)
    queue: VecDeque<((usize, usize), usize)>,

    pushes: usize,
    pops: usize,
    contains: usize,
    inserts: usize,

    auto: bool,
    dfs: bool,

    len: Option<usize>,

    start_time: f64,
}

impl ArraySolverApp {
    fn reset(&mut self, time: f64) {
        self.maze = crate::maze::Maze::random(N);
        self.grid = self.maze.grid();
        self.visited = self
            .maze
            .grid()
            .iter()
            .map(|r| r.iter().map(|_| false).collect())
            .collect();
        self.enqueued = BTreeSet::new();
        self.next = Some((1, 0));
        self.queue = VecDeque::new();
        self.queue.push_back(((1, 0), 0));
        self.enqueued.insert((1, 0));
        self.pushes = 0;
        self.pops = 0;
        self.contains = 0;
        self.inserts = 0;
        self.len = None;
        self.start_time = time;
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
        if self.done() {
            return;
        }
        self.pops += 1;
        // oh god what have i done
        let Some((p, dist)) = (if self.dfs {
            self.queue.pop_back()
        } else {
            self.queue.pop_front()
        }) else {
            return;
        };
        self.visited[p.1][p.0] = true;

        self.contains += 1;
        if self.visited[2 * N][2 * N - 1] {
            self.next = None;
            self.len = Some(dist);
            self.queue = VecDeque::new();
            return;
        }

        let enqueued = self.enqueued.clone();
        for next in super::maze::DIRS
            .into_iter()
            .flat_map(|d| d.translate(p, 2 * N + 1))
            .filter(|&(x, y)| {
                self.contains += 1;
                !self.grid[y][x] && !enqueued.contains(&(x, y))
            })
        {
            self.pushes += 1;
            self.queue.push_back((next, dist + 1));
            self.inserts += 1;
            self.enqueued.insert(next);
        }
        self.next = if self.dfs {
            self.queue.back()
        } else {
            self.queue.front()
        }
        .map(|&(p, _)| p);
    }

    fn done(&self) -> bool {
        self.visited[2 * N][2 * N - 1]
    }
}

impl Default for ArraySolverApp {
    fn default() -> Self {
        let maze = crate::maze::Maze::random(N);
        let mut app = Self {
            gui_scale: 1.,

            maze,
            grid: Vec::new(),
            visited: Vec::new(),
            enqueued: BTreeSet::new(),
            next: Some((1, 0)),
            queue: VecDeque::new(),
            pushes: 0,
            pops: 0,
            contains: 0,
            inserts: 0,

            auto: false,
            dfs: false,
            len: None,
            start_time: 0.,
        };
        app.reset(0.);
        app
    }
}

impl eframe::App for ArraySolverApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        let time = ctx.input(|i| i.time);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Array solver");

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
            ui.checkbox(&mut self.dfs, "DFS");

            ui.label(format!("Queue pushes: {}", self.pushes));
            ui.label(format!("Queue pops: {}", self.pops));
            ui.label(format!("Set contains: {}", self.contains));
            ui.label(format!("Set insertions: {}", self.inserts));

            if let Some(dist) = self.len {
                ui.label(format!("Length: {dist}"));
            }
        });
    }
}
