use rand::Rng;

use crate::world::{HEIGHT, WIDTH};

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
pub enum EntityType {
    Corpse,

    Oak,

    Deer,
    Boar,
    Wolf,
    Raven,
}

pub struct Entity {
    pub entity_type: EntityType,
    pub x: f32,
    pub y: f32,
    pub energy: f32,
}

impl Entity {
    pub fn new(entity_type: EntityType, x: f32, y: f32, energy: f32) -> Self {
        Entity {
            entity_type,
            x,
            y,
            energy,
        }
    }

    pub fn oak() -> Self {
        let mut rng = rand::rng();

        Entity::new(
            EntityType::Oak,
            rng.random_range(0.0..WIDTH),
            rng.random_range(0.0..HEIGHT),
            100.0,
        )
    }
    pub fn deer() -> Self {
        let mut rng = rand::rng();

        Entity::new(
            EntityType::Deer,
            rng.random_range(0.0..WIDTH),
            rng.random_range(0.0..HEIGHT),
            100.0,
        )
    }
    pub fn wolf() -> Self {
        let mut rng = rand::rng();

        Entity::new(
            EntityType::Wolf,
            rng.random_range(0.0..WIDTH),
            rng.random_range(0.0..HEIGHT),
            100.0,
        )
    }
    pub fn raven() -> Self {
        let mut rng = rand::rng();

        Entity::new(
            EntityType::Raven,
            rng.random_range(0.0..WIDTH),
            rng.random_range(0.0..HEIGHT),
            100.0,
        )
    }
    pub fn boar() -> Self {
        let mut rng = rand::rng();

        Entity::new(
            EntityType::Boar,
            rng.random_range(0.0..WIDTH),
            rng.random_range(0.0..HEIGHT),
            100.0,
        )
    }
    // pub fn corpse() -> Self {
    //     let mut rng = rand::rng();

    //     Entity::new(
    //         EntityType::Corpse,
    //         rng.random_range(0.0..WIDTH),
    //         rng.random_range(0.0..HEIGHT),
    //         100.0,
    //     )
    // }

    fn in_range(&mut self, entity: &Entity) -> bool {
        ((self.x - entity.x).powi(2) + (self.y - entity.y).powi(2)).sqrt() < 10.0
    }

    pub fn eat(&mut self, entities: &mut [&mut Entity]) {
        let _nearby: Vec<&Entity> = entities
            .iter()
            .filter(|e| self.in_range(e))
            .map(|e| &**e)
            .collect();

        match self.entity_type {
            EntityType::Oak | EntityType::Corpse => return,
            EntityType::Deer => {
                if let Some(_target) = entities
                    .iter_mut()
                    .find(|e| e.entity_type == EntityType::Oak)
                {
                    self.energy += 10.0;
                }
            }
            EntityType::Boar => {
                if let Some(_target) = entities
                    .iter_mut()
                    .find(|e| e.entity_type == EntityType::Oak)
                {
                    self.energy += 15.0;
                } else if let Some(target) = entities
                    .iter_mut()
                    .find(|e| e.entity_type == EntityType::Deer)
                {
                    self.kill(target);
                }
            }
            EntityType::Wolf => {
                if let Some(target) = entities.iter_mut().find(|e| {
                    e.entity_type == EntityType::Deer || e.entity_type == EntityType::Boar
                }) {
                    self.kill(target);
                }
            }
            EntityType::Raven => {
                if let Some(_target) = entities
                    .iter_mut()
                    .find(|e| e.entity_type == EntityType::Corpse)
                {
                    self.energy += 5.0;
                }
            }
        }
    }

    pub fn reproduce(&mut self, new_entities: &mut Vec<Entity>, entities: &mut [&mut Entity]) {
        let _nearby: Vec<&Entity> = entities
            .iter()
            .filter(|e| self.in_range(e))
            .map(|e| &**e)
            .collect();

        match self.entity_type {
            EntityType::Oak | EntityType::Corpse => return,
            EntityType::Deer | EntityType::Boar | EntityType::Wolf | EntityType::Raven => {
                if let Some(partner) = entities
                    .iter_mut()
                    .find(|e| e.entity_type == self.entity_type)
                {
                    if self.mate(partner) {
                        new_entities.push(Entity::new(self.entity_type, self.x, self.y, 40.0));
                    }
                }
            }
        }
    }

    fn mate(&mut self, partner: &mut Entity) -> bool {
        if self.energy < 80.0 || partner.energy < 80.0 {
            return false;
        }

        self.energy -= 30.0;
        partner.energy -= 30.0;

        true
    }

    fn kill(&mut self, prey: &mut Entity) {
        prey.energy -= 100.0;
        self.energy += 100.0;
        prey.entity_type = EntityType::Corpse;
        prey.energy = 100.0;
    }

    pub fn random_move(&mut self) {
        if self.entity_type == EntityType::Oak {
            return;
        }

        self.energy -= 10.0;

        if self.energy < 0.0 && self.entity_type != EntityType::Corpse {
            self.entity_type = EntityType::Corpse;
            self.energy = 100.0;
            return;
        }

        let mut rng = rand::rng();

        self.x += rng.random_range(-10.0..10.0);
        self.y += rng.random_range(-10.0..10.0);

        if self.x < 0.0 {
            self.x = 0.0;
        }
        if self.x > WIDTH {
            self.x = WIDTH;
        }
        if self.y < 0.0 {
            self.y = 0.0;
        }
        if self.y > HEIGHT {
            self.y = HEIGHT;
        }
    }
}
