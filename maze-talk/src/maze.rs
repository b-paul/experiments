use std::collections::HashSet;

pub struct Maze {
    // Walls are on the bottom of the square
    vertical_walls: Vec<Vec<bool>>,
    // Walls are on the right of the square
    horizontal_walls: Vec<Vec<bool>>,
    size: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub const DIRS: [Direction; 4] = [
    Direction::Left,
    Direction::Right,
    Direction::Up,
    Direction::Down,
];

impl Direction {
    pub fn offset(self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }

    pub fn translate(self, p: (usize, usize), n: usize) -> Option<(usize, usize)> {
        let (dx, dy) = self.offset();
        let (x, y) = (p.0 as i32 + dx, p.1 as i32 + dy);
        x.try_into()
            .ok()
            .filter(|&x| x < n)
            .and_then(|x| y.try_into().ok().filter(|&y| y < n).map(|y| (x, y)))
    }
}

impl Maze {
    pub fn random(n: usize) -> Self {
        use rand::prelude::*;

        let vertical_walls = (0..n).map(|_| vec![true; n]).collect::<Vec<_>>();
        let horizontal_walls = (0..n).map(|_| vec![true; n]).collect::<Vec<_>>();

        let mut maze = Self {
            vertical_walls,
            horizontal_walls,
            size: n,
        };

        // Wilson's algorithm (read from wikipedia)

        let mut visited = HashSet::new();
        let mut to_visit = (0..n)
            .flat_map(|a| (0..n).map(move |b| (a, b)))
            .collect::<HashSet<_>>();

        let mut rng = rand::rng();

        let &initial = to_visit.iter().choose(&mut rng).unwrap();
        to_visit.remove(&initial);
        visited.insert(initial);

        while let Some(&p) = to_visit.iter().choose(&mut rng) {
            let mut path = vec![p];
            let mut dirs = Vec::new();
            let mut path_set = HashSet::new();
            path_set.insert(p);
            // Perform a random walk, resetting the path if we collide our walk, and ending when we
            // reach the visited part of the maze.
            while !visited.contains(path.last().unwrap()) {
                let (dir, next) = DIRS
                    .iter()
                    .flat_map(|d| d.translate(*path.last().unwrap(), n).map(|p| (d, p)))
                    .choose(&mut rng)
                    .unwrap();

                if path_set.contains(&next) {
                    while *path.last().unwrap() != next {
                        let tail = path.pop().unwrap();
                        dirs.pop();
                        path_set.remove(&tail);
                    }
                } else {
                    path.push(next);
                    dirs.push(dir);
                    path_set.insert(next);
                    if visited.contains(&next) {
                        // Remove all of the walls we came into in the path!
                        path.pop();
                        while let Some(&dir) = dirs.pop() {
                            let p = path.pop().unwrap();
                            maze.remove_wall(p, dir);
                            to_visit.remove(&p);
                            visited.insert(p);
                        }
                        break;
                    }
                }
            }
        }
        maze
    }

    fn remove_wall(&mut self, (x, y): (usize, usize), dir: Direction) {
        match dir {
            Direction::Left => self.horizontal_walls[y][x - 1] = false,
            Direction::Right => self.horizontal_walls[y][x] = false,
            Direction::Up => self.vertical_walls[y - 1][x] = false,
            Direction::Down => self.vertical_walls[y][x] = false,
        }
    }

    pub fn grid(&self) -> Vec<Vec<bool>> {
        let mut grid = (0..self.size * 2 + 1)
            .map(|_| vec![true; self.size * 2 + 1])
            .collect::<Vec<_>>();

        grid[0][1] = false;
        grid[self.size * 2][self.size * 2 - 1] = false;

        for x in 0..self.size {
            for y in 0..self.size {
                grid[2 * y + 1][2 * x + 1] = false;
                if !self.vertical_walls[y][x] {
                    grid[2 * y + 2][2 * x + 1] = false;
                }
                if !self.horizontal_walls[y][x] {
                    grid[2 * y + 1][2 * x + 2] = false;
                }
            }
        }

        grid
    }
}
