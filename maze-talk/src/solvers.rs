use super::maze::DIRS;

use std::collections::{BTreeSet, VecDeque};

pub fn min_dist_array_bfs(walls: Vec<Vec<bool>>, size: usize) -> usize {
    let mut queue = VecDeque::new();
    let mut enqueued = BTreeSet::new();

    queue.push_back(((1, 0), 0));
    enqueued.insert((1, 0));

    while let Some((p, dist)) = queue.pop_front() {
        if p == (size - 2, size - 1) {
            return dist;
        }

        for dir in DIRS {
            if let Some(next) = dir.translate(p, size) {
                if !walls[next.1][next.0] && !enqueued.contains(&next) {
                    queue.push_back((next, dist + 1));
                    enqueued.insert(next);
                }
            }
        }
    }

    panic!("No solution found!");
}

pub fn min_dist_array_dfs(walls: Vec<Vec<bool>>, size: usize) -> usize {
    let mut stack = Vec::new();
    let mut enqueued = BTreeSet::new();

    stack.push(((1, 0), 0));
    enqueued.insert((1, 0));

    while let Some((p, dist)) = stack.pop() {
        if p == (size - 2, size - 1) {
            return dist;
        }

        for dir in DIRS {
            if let Some(next) = dir.translate(p, size) {
                if !walls[next.1][next.0] && !enqueued.contains(&next) {
                    stack.push((next, dist + 1));
                    enqueued.insert(next);
                }
            }
        }
    }

    panic!("No solution found!");
}

pub fn min_dist_bitboards(walls: u64) -> usize {
    const L: u64 = 0b100000010000001000000100000010000001000000100000010000001000000;
    const R: u64 = 0b000000100000010000001000000100000010000001000000100000010000001;

    let mut visited = 1 << 62;
    let mut dist = 0;

    // While we haven't reached the endpoint...
    while visited & 1 == 0 {
        visited =
            (visited | (!L & visited) << 1 | (!R & visited) >> 1 | visited << 7 | visited >> 7)
                & !walls;
        dist += 1;
    }
    dist
}
