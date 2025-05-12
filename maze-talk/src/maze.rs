pub struct Maze<const N: usize> {
    walls: [[bool; N]; N],
}

impl<const N: usize> Default for Maze<N> {
    fn default() -> Self {
        // this is a bit of an insane definition but idc
        Self::random()
    }
}

impl<const N: usize> Maze<N> {
    pub fn random() -> Self {
        // blah blah yeah this isn't uniform over all mazes sorry it was easy to implement i'll fix
        // it later
        use rand::prelude::*;

        const OFFSETS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let mut walls = [[true; N]; N];
        let mut visited = [[false; N]; N];
        walls[0][1] = false;

        let mut rng = rand::rng();

        let mut stack = vec![(1, 1)];
        while let Some((x, y)) = stack.pop() {
            walls[y as usize][x as usize] = false;
            visited[y as usize][x as usize] = true;

            if let Some(((wx, wy), next)) = OFFSETS
                .map(|(dx, dy)| ((x + dx, y + dy), (x + 2 * dx, y + 2 * dy)))
                .iter()
                .filter(|(_, (x, y))| 0 <= *x && *x < N as i32 && 0 <= *y && *y < N as i32)
                .filter(|(_, (x, y))| !visited[*y as usize][*x as usize])
                .choose(&mut rng)
            {
                stack.push((x, y));
                stack.push(*next);
                walls[*wy as usize][*wx as usize] = false;
            }
        }
        walls[N-1][N-2] = false;

        Self { walls }
    }

    pub fn grid(&self) -> [[bool; N]; N] {
        self.walls
    }
}
