use std::time::Duration;

const WIDTH: usize = 80;
const HEIGHT: usize = 24;
const GRAVITY: f64 = 0.0;

struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

impl Particle {
    fn new(x: f64, y: f64, vx: f64, vy: f64) -> Self {
        Particle { x, y, vx, vy }
    }

    fn update(&mut self, dt: f64, width: usize, height: usize) {
        self.vy += GRAVITY * dt;
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        if self.x < 0.0 {
            self.x = 0.0;
            self.vx = -self.vx;
        }
        if self.x > width as f64 - 1.0 {
            self.x = width as f64 - 1.0;
            self.vx = -self.vx;
        }
        if self.y < 0.0 {
            self.y = 0.0;
            self.vy = -self.vy;
        }
        if self.y > height as f64 - 1.0 {
            self.y = height as f64 - 1.0;
            self.vy = -self.vy;
        }
    }
}

fn render(particles: &Vec<Particle>) {
    let mut buf = vec![vec![' '; WIDTH]; HEIGHT];

    for p in particles {
        let x = p.x as usize;
        let y = p.y as usize;

        if x < WIDTH && y < HEIGHT {
            buf[y][x] = '*';
        }
    }

    for row in buf {
        let line: String = row.iter().collect();
        println!("{}", line);
    }
}

fn handle_collisions(particles: &mut Vec<Particle>) {
    for i in 0..particles.len() {
        for j in i + 1..particles.len() {
            let dx = (particles[i].x - particles[j].x).abs();
            let dy = (particles[i].y - particles[j].y).abs();
            if dx < 1.0 && dy < 1.0 {
                particles[i].vx = -particles[i].vx;
                particles[i].vy = -particles[i].vy;
                particles[j].vx = -particles[j].vx;
                particles[j].vy = -particles[j].vy;
            }
        }
    }
}

fn main() {
    let dt = 3.0;
    let mut particles = vec![
        Particle::new(10.0, 5.0, 1.0, 0.0),
        Particle::new(30.0, 10.0, -0.5, 0.2),
        Particle::new(50.0, 15.0, 0.7, -0.3),
        Particle::new(20.0, 5.0, -0.35, 0.3),
        Particle::new(30.0, 25.0, -0.35, -0.3),
        Particle::new(40.0, 35.0, 0.15, -0.15),
    ];

    loop {
        print!("\x1b[2J\x1b[H");

        for p in &mut particles {
            p.update(dt, WIDTH, HEIGHT);
        }

        handle_collisions(&mut particles);

        render(&particles);

        std::thread::sleep(Duration::from_millis(100));
    }
}
