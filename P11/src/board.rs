use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use rand::Rng;
use std::collections::VecDeque;
use std::io::stdout;

use crate::genome::{Genome, Moves};
use crate::terrain::Terrain;

pub struct Board {
    pos: (usize, usize),
    target: (usize, usize),
    terrain: Vec<Vec<Terrain>>,
    genomes: Vec<Genome>,
}

fn move_delta(mv: &Moves) -> (isize, isize) {
    match mv {
        Moves::Left => (-1, 0),
        Moves::Right => (1, 0),
        Moves::Up => (0, -1),
        Moves::Down => (0, 1),
        Moves::LeftUp => (-1, -1),
        Moves::LeftDown => (-1, 1),
        Moves::RightUp => (1, -1),
        Moves::RightDown => (1, 1),
    }
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let terrain = vec![vec![Terrain::Floor; width]; height];
        Board {
            pos: (0, 0),
            target: (width - 1, height - 1),
            terrain,
            genomes: vec![Genome::new(50); 5000],
        }
    }

    pub fn set_pos(&mut self, x: usize, y: usize) {
        if x < self.terrain[0].len() && y < self.terrain.len() {
            self.pos = (x, y);
            self.terrain[y][x] = Terrain::Player;
        } else {
            panic!("Position out of bounds");
        }
    }

    pub fn set_target(&mut self, x: usize, y: usize) {
        if x < self.terrain[0].len() && y < self.terrain.len() {
            self.target = (x, y);
            self.terrain[y][x] = Terrain::Target;
        } else {
            panic!("Target position out of bounds");
        }
    }

    pub fn print_board(&self) {
        print!("{}[2J", 27 as char);

        for row in &self.terrain {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }

    pub fn print_board_with_path(&self) {
        print!("{}[2J", 27 as char);

        let mut stdout = stdout();
        execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All)).unwrap();

        let mut display_terrain = self.terrain.clone();
        let mut x = self.pos.0 as isize;
        let mut y = self.pos.1 as isize;

        let gene = self.get_best();

        for mv in &gene.genes {
            let (dx, dy) = move_delta(mv);
            x += dx;
            y += dy;

            if x < 0
                || y < 0
                || x >= self.terrain[0].len() as isize
                || y >= self.terrain.len() as isize
            {
                break;
            }

            match self.terrain[y as usize][x as usize] {
                Terrain::Wall => break,
                Terrain::Player | Terrain::Target => {}
                _ => display_terrain[y as usize][x as usize] = Terrain::Player,
            }

            if (x as usize, y as usize) == self.target {
                break;
            }
        }

        for row in &display_terrain {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }

        println!("Best genome score: {}", self.evaluate_genome(gene));
    }

    pub fn generate_walls(&mut self, density: f64) {
        let mut rng = rand::rng();

        loop {
            for y in 0..self.terrain.len() {
                for x in 0..self.terrain[0].len() {
                    self.terrain[y][x] = Terrain::Floor;
                }
            }

            for y in 0..self.terrain.len() {
                for x in 0..self.terrain[0].len() {
                    if (x, y) == self.pos || (x, y) == self.target {
                        continue;
                    }
                    if rng.random::<f64>() < density {
                        self.terrain[y][x] = Terrain::Wall;
                    }
                }
            }

            self.terrain[self.pos.1][self.pos.0] = Terrain::Player;
            self.terrain[self.target.1][self.target.0] = Terrain::Target;

            if self.is_reachable(self.target) {
                break;
            }
        }
    }

    fn can_move_to(&self, x: isize, y: isize, mv: &Moves, allow_corner_cut: bool) -> bool {
        let w = self.terrain[0].len() as isize;
        let h = self.terrain.len() as isize;
        let (nx, ny) = (x + move_delta(mv).0, y + move_delta(mv).1);

        if nx < 0 || ny < 0 || nx >= w || ny >= h {
            return false;
        }
        if let Terrain::Wall = self.terrain[ny as usize][nx as usize] {
            return false;
        }

        if !allow_corner_cut {
            match mv {
                Moves::LeftUp => {
                    if let Terrain::Wall = self.terrain[y as usize][(x - 1) as usize] {
                        return false;
                    }
                    if let Terrain::Wall = self.terrain[(y - 1) as usize][x as usize] {
                        return false;
                    }
                }
                Moves::LeftDown => {
                    if let Terrain::Wall = self.terrain[y as usize][(x - 1) as usize] {
                        return false;
                    }
                    if let Terrain::Wall = self.terrain[(y + 1) as usize][x as usize] {
                        return false;
                    }
                }
                Moves::RightUp => {
                    if let Terrain::Wall = self.terrain[y as usize][(x + 1) as usize] {
                        return false;
                    }
                    if let Terrain::Wall = self.terrain[(y - 1) as usize][x as usize] {
                        return false;
                    }
                }
                Moves::RightDown => {
                    if let Terrain::Wall = self.terrain[y as usize][(x + 1) as usize] {
                        return false;
                    }
                    if let Terrain::Wall = self.terrain[(y + 1) as usize][x as usize] {
                        return false;
                    }
                }
                _ => {}
            }
        }

        true
    }

    fn is_reachable(&self, target: (usize, usize)) -> bool {
        let mut visited = vec![vec![false; self.terrain[0].len()]; self.terrain.len()];
        let mut queue = VecDeque::new();
        queue.push_back(self.pos);
        visited[self.pos.1][self.pos.0] = true;

        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some((x, y)) = queue.pop_front() {
            if (x, y) == target {
                return true;
            }
            for (dx, dy) in dirs {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && ny >= 0 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if ny < self.terrain.len() && nx < self.terrain[0].len() {
                        if !visited[ny][nx] {
                            match self.terrain[ny][nx] {
                                Terrain::Wall => {}
                                _ => {
                                    visited[ny][nx] = true;
                                    queue.push_back((nx, ny));
                                }
                            }
                        }
                    }
                }
            }
        }

        false
    }

    pub fn evaluate_genome(&self, g: &Genome) -> usize {
        let mut x = self.pos.0 as isize;
        let mut y = self.pos.1 as isize;
        let width = self.terrain[0].len() as isize;
        let height = self.terrain.len() as isize;

        let allow_corner_cut = false;

        for (i, mv) in g.genes.iter().enumerate() {
            let (dx, dy) = move_delta(mv);
            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || ny < 0 || nx >= width || ny >= height {
                break;
            }

            if !self.can_move_to(x, y, mv, allow_corner_cut) {
                break;
            }

            x = nx;
            y = ny;

            if (x as usize, y as usize) == self.target {
                return i;
            }
        }

        let dx = (x - self.target.0 as isize).abs() as usize;
        let dy = (y - self.target.1 as isize).abs() as usize;
        let manhattan = dx + dy;

        100 + manhattan + g.genes.len()
    }

    pub fn next_generation(&mut self) {
        let mut rng = rand::rng();

        let mut scored: Vec<(usize, Genome)> = self
            .genomes
            .iter()
            .enumerate()
            .map(|(_, g)| (self.evaluate_genome(g), g.clone()))
            .collect();

        scored.sort_by_key(|(score, _)| *score);

        let elite_n = (self.genomes.len() / 50).max(1);
        let mut new_pop: Vec<Genome> = scored
            .iter()
            .take(elite_n)
            .map(|(_, g)| g.clone())
            .collect();

        if let Some(path) = self.shortest_path_bfs() {
            let seed_genome = Genome::from_path(&path);
            new_pop.push(seed_genome.clone());
        }

        while new_pop.len() < self.genomes.len() {
            let mut parent = |k: usize| {
                let mut best_idx = rng.random_range(0..self.genomes.len());
                let mut best_score = self.evaluate_genome(&self.genomes[best_idx]);
                for _ in 1..k {
                    let j = rng.random_range(0..self.genomes.len());
                    let s = self.evaluate_genome(&self.genomes[j]);
                    if s < best_score {
                        best_score = s;
                        best_idx = j;
                    }
                }
                self.genomes[best_idx].clone()
            };

            let p1 = parent(5);
            let p2 = parent(5);

            let mut child = Genome::crossover(&p1, &p2);

            self.repair_and_fill(&mut child, p1.genes.len().max(p2.genes.len()));

            while child.remove_loops(self.pos) {}

            for _ in 0..5 {
                if rng.random::<f64>() < 0.3 {
                    child.smart_mutate(self);
                }
            }

            new_pop.push(child);
        }

        self.genomes = new_pop;
    }
    pub fn best_index(&self) -> usize {
        let mut best_i = 0usize;
        let mut best_score = usize::MAX;
        for (i, g) in self.genomes.iter().enumerate() {
            let s = self.evaluate_genome(g);
            if s < best_score {
                best_score = s;
                best_i = i;
            }
        }
        best_i
    }

    pub fn get_best(&self) -> &Genome {
        let i = self.best_index();
        &self.genomes[i]
    }

    pub fn shortest_path_bfs(&self) -> Option<Vec<(usize, usize)>> {
        let w = self.terrain[0].len();
        let h = self.terrain.len();
        let mut visited = vec![vec![false; w]; h];
        let mut parent: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; w]; h];
        let mut q = VecDeque::new();
        q.push_back(self.pos);
        visited[self.pos.1][self.pos.0] = true;

        let dirs: &[(isize, isize)] = &[
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        while let Some((x, y)) = q.pop_front() {
            if (x, y) == self.target {
                let mut path = Vec::new();
                let mut cur = (x, y);
                path.push(cur);
                while let Some(p) = parent[cur.1][cur.0] {
                    cur = p;
                    path.push(cur);
                }
                path.reverse();
                return Some(path);
            }
            for (dx, dy) in dirs {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && ny >= 0 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if nx < w && ny < h && !visited[ny][nx] {
                        if let Terrain::Wall = self.terrain[ny][nx] {
                            continue;
                        }
                        visited[ny][nx] = true;
                        parent[ny][nx] = Some((x, y));
                        q.push_back((nx, ny));
                    }
                }
            }
        }
        None
    }

    pub fn repair_and_fill(&self, g: &mut Genome, target_len: usize) {
        let mut x = self.pos.0 as isize;
        let mut y = self.pos.1 as isize;
        let w = self.terrain[0].len() as isize;
        let h = self.terrain.len() as isize;

        let mut valid_prefix = 0usize;
        for mv in &g.genes {
            let (dx, dy) = move_delta(mv);
            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || ny < 0 || nx >= w || ny >= h {
                break;
            }
            if let Terrain::Wall = self.terrain[ny as usize][nx as usize] {
                break;
            }
            x = nx;
            y = ny;
            valid_prefix += 1;
            if (x as usize, y as usize) == self.target {
                break;
            }
        }

        g.genes.truncate(valid_prefix);

        let mut rng = rand::rng();
        while g.genes.len() < target_len {
            let mut candidates = Vec::new();
            for mv in &[
                Moves::Left,
                Moves::Right,
                Moves::Up,
                Moves::Down,
                Moves::LeftUp,
                Moves::LeftDown,
                Moves::RightUp,
                Moves::RightDown,
            ] {
                let (dx, dy) = move_delta(mv);
                let nx = x + dx;
                let ny = y + dy;

                if nx < 0 || ny < 0 || nx >= w || ny >= h {
                    continue;
                }
                if let Terrain::Wall = self.terrain[ny as usize][nx as usize] {
                    continue;
                }

                let dist_now =
                    (x - self.target.0 as isize).abs() + (y - self.target.1 as isize).abs();
                let dist_next =
                    (nx - self.target.0 as isize).abs() + (ny - self.target.1 as isize).abs();
                let score = if dist_next < dist_now { 5 } else { 1 };
                candidates.push((mv.clone(), score));
            }

            if candidates.is_empty() {
                break;
            }

            let total: usize = candidates.iter().map(|(_, s)| *s).sum();
            let mut pick = rng.random_range(0..total);
            let mut chosen = candidates[0].0.clone();
            for (mv, s) in candidates {
                if pick < s {
                    chosen = mv;
                    break;
                }
                pick -= s;
            }

            let (dx, dy) = move_delta(&chosen);
            x += dx;
            y += dy;
            g.genes.push(chosen);

            if (x as usize, y as usize) == self.target {
                break;
            }
        }
    }
}
