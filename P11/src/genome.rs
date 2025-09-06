use crate::board::Board;
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Moves {
    Left,
    Right,
    Up,
    Down,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
}

#[derive(Clone)]
pub struct Genome {
    pub genes: Vec<Moves>,
}

impl Genome {
    pub fn new(length: usize) -> Self {
        let mut rng = rand::rng();
        let genes = (0..length)
            .map(|_| match rng.random_range(0..8) {
                0 => Moves::Left,
                1 => Moves::Right,
                2 => Moves::Up,
                3 => Moves::Down,
                4 => Moves::LeftUp,
                5 => Moves::LeftDown,
                6 => Moves::RightUp,
                _ => Moves::RightDown,
            })
            .collect();
        Genome { genes }
    }

    pub fn crossover(parent1: &Genome, parent2: &Genome) -> Genome {
        let mut rng = rand::rng();
        let len = parent1.genes.len().min(parent2.genes.len()).max(1);
        let point = rng.random_range(0..len);
        let mut child_genes = parent1.genes[..point].to_vec();
        child_genes.extend_from_slice(&parent2.genes[point..]);
        Genome { genes: child_genes }
    }

    pub fn from_path(path: &[(usize, usize)]) -> Genome {
        let mut genes = Vec::with_capacity(path.len().saturating_sub(1));
        for pair in path.windows(2) {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];
            let mv = if x2 == x1 + 1 && y2 == y1 {
                Moves::Right
            } else if x2 + 1 == x1 && y2 == y1 {
                Moves::Left
            } else if y2 == y1 + 1 && x2 == x1 {
                Moves::Down
            } else if y2 + 1 == y1 && x2 == x1 {
                Moves::Up
            } else if x2 == x1 + 1 && y2 == y1 + 1 {
                Moves::RightDown
            } else if x2 == x1 + 1 && y2 + 1 == y1 {
                Moves::RightUp
            } else if x2 + 1 == x1 && y2 == y1 + 1 {
                Moves::LeftDown
            } else if x2 + 1 == x1 && y2 + 1 == y1 {
                Moves::LeftUp
            } else {
                panic!("Invalid path step: ({},{}) -> ({},{})", x1, y1, x2, y2);
            };
            genes.push(mv);
        }
        Genome { genes }
    }

    pub fn remove_loops(&mut self, start: (usize, usize)) -> bool {
        let mut pos = start;
        let mut seen: HashMap<(usize, usize), usize> = HashMap::new();
        seen.insert(pos, 0);
        let mut positions: Vec<(usize, usize)> = vec![pos];

        for mv in &self.genes {
            pos = match mv {
                Moves::Left => (pos.0.saturating_sub(1), pos.1),
                Moves::Right => (pos.0 + 1, pos.1),
                Moves::Up => (pos.0, pos.1.saturating_sub(1)),
                Moves::Down => (pos.0, pos.1 + 1),
                Moves::LeftUp => (pos.0.saturating_sub(1), pos.1.saturating_sub(1)),
                Moves::LeftDown => (pos.0.saturating_sub(1), pos.1 + 1),
                Moves::RightUp => (pos.0 + 1, pos.1.saturating_sub(1)),
                Moves::RightDown => (pos.0 + 1, pos.1 + 1),
            };
            positions.push(pos);
        }

        for i in 0..positions.len() {
            if let Some(&first_idx) = seen.get(&positions[i]) {
                if first_idx < i {
                    self.genes.drain(first_idx..i);
                    return true;
                }
            } else {
                seen.insert(positions[i], i);
            }
        }
        false
    }

    pub fn smart_mutate(&mut self, board: &Board) {
        let mut rng = rand::rng();
        if self.genes.is_empty() {
            return;
        }
        let cut = rng.random_range(0..self.genes.len());
        self.genes.truncate(cut);
        board.repair_and_fill(self, 50 /* target len - dopasuj */);
    }
}
