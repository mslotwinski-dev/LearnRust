const DT: f32 = 0.05;
const G: f32 = 9.81;
const K: f32 = 0.3; // współczynnik oporu powietrza (dostosuj do efektu)

pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

impl Bullet {
    pub fn new(v: f32, angle_deg: f32) -> Self {
        let angle_rad = angle_deg.to_radians();
        Bullet {
            x: 0.0,
            y: 0.0,
            vx: v * angle_rad.cos(),
            vy: v * angle_rad.sin(),
        }
    }

    pub fn update(&mut self) -> (f32, f32) {
        // opór powietrza
        self.vx -= self.vx * K * DT;
        self.vy -= self.vy * K * DT;

        // grawitacja
        self.vy -= G * DT;

        // ruch
        self.x += self.vx * DT * 10.0;
        self.y += self.vy * DT * 10.0;

        // odbicie od ziemi
        if self.y < 0.0 {
            self.y = 0.0;
            if self.vy < 0.0 {
                self.vy = -0.3 * self.vy;
            }
        }

        if self.vy.abs() < 0.1 && self.y.abs() <= 0.1 {
            self.vy = 0.0;
            self.vx = 0.0;
        }

        (self.x, self.y)
    }
}
