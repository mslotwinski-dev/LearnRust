use rand::Rng;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
enum State {
    Dead,
    Alive,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            State::Alive => write!(f, "██"),
            State::Dead => write!(f, "  "),
        }
    }
}

pub struct Game {
    cells: Vec<Vec<State>>,
}

impl Game {
    pub fn new(width: u32, height: u32, density: f32, chaos: bool) {
        let mut cells = vec![vec![State::Dead; width as usize]; height as usize];

        let mut rng = rand::rng();
        for y in 0..height {
            for x in 0..width {
                if rng.random::<f32>() < density {
                    cells[y as usize][x as usize] = State::Alive;
                }
            }
        }

        Game { cells }.run(chaos);
    }

    fn display(&self) {
        print!("\x1B[2J\x1B[1;1H");

        let mut buffer = String::new();
        for row in &self.cells {
            for cell in row {
                buffer.push_str(&cell.to_string());
            }
            buffer.push('\n');
        }
        print!("{}", buffer);
    }

    fn neighbors(&self, x: usize, y: usize) -> u8 {
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut count = 0;

        for (dx, dy) in directions.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0
                && ny >= 0
                && (nx as usize) < self.cells[0].len()
                && (ny as usize) < self.cells.len()
            {
                if let State::Alive = self.cells[ny as usize][nx as usize] {
                    count += 1;
                }
            }
        }

        count
    }

    fn chaos(&mut self) {
        for y in 0..self.cells.len() {
            for x in 0..self.cells[0].len() {
                let alive_neighbors = self.neighbors(x, y);
                self.cells[y][x] = match (self.cells[y][x].clone(), alive_neighbors) {
                    (State::Alive, 2) | (State::Alive, 3) => State::Alive,
                    (State::Dead, 3) => State::Alive,
                    _ => State::Dead,
                };
            }
        }
    }

    fn step(&mut self) {
        let mut new_cells = self.cells.clone();

        for y in 0..self.cells.len() {
            for x in 0..self.cells[0].len() {
                let alive_neighbors = self.neighbors(x, y);
                new_cells[y][x] = match (self.cells[y][x].clone(), alive_neighbors) {
                    (State::Alive, 2) | (State::Alive, 3) => State::Alive,
                    (State::Dead, 3) => State::Alive,
                    _ => State::Dead,
                };
            }
        }

        self.cells = new_cells;
    }

    fn run(&mut self, chaos: bool) {
        loop {
            (if chaos { self.chaos() } else { self.step() });

            self.display();
            thread::sleep(Duration::from_millis(100));
        }
    }
}
