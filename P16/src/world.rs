use crate::entity::{Entity, EntityType};

pub const WIDTH: f32 = 5000.0;
pub const HEIGHT: f32 = 5000.0;

pub struct World {
    entities: Vec<Entity>,
}

impl World {
    pub fn new() -> Self {
        let mut entities = Vec::new();

        for _ in 0..5000 {
            entities.push(Entity::oak());
        }

        for _ in 0..200 {
            entities.push(Entity::deer());
        }
        for _ in 0..10 {
            entities.push(Entity::wolf());
        }
        for _ in 0..30 {
            entities.push(Entity::raven());
        }
        for _ in 0..25 {
            entities.push(Entity::boar());
        }

        World { entities }
    }

    fn cleanup(&mut self) {
        self.entities.retain(|e| e.energy > 0.0);
    }

    pub fn simulation_step(&mut self) {
        // faza 1 – ruch
        for entity in self.entities.iter_mut() {
            entity.random_move();
        }

        // faza 2 – jedzenie
        let len = self.entities.len();
        for i in 0..len {
            let (left, right) = self.entities.split_at_mut(i);
            let (entity_slot, rest) = right.split_at_mut(1);

            let entity = &mut entity_slot[0];

            // zbierz wszystkie inne encje w slice
            let mut others: Vec<&mut Entity> = left.iter_mut().chain(rest.iter_mut()).collect();

            // jedzenie
            entity.eat(&mut others[..]);
        }

        // faza 3 – reprodukcja
        let mut new_entities: Vec<Entity> = Vec::new();
        let len = self.entities.len();
        for i in 0..len {
            let (left, right) = self.entities.split_at_mut(i);
            let (entity_slot, rest) = right.split_at_mut(1);

            let entity = &mut entity_slot[0];

            // inne encje jako slice
            let mut others: Vec<&mut Entity> = left.iter_mut().chain(rest.iter_mut()).collect();

            // zamiast pushować bezpośrednio, dodajemy do new_entities
            entity.reproduce(&mut new_entities, &mut others[..]);
        }

        // dodaj nowe encje do świata
        self.entities.append(&mut new_entities);

        // faza 4 – sprzątanie
        self.cleanup();
    }

    pub fn get_counts(&self) -> (usize, usize, usize, usize, usize) {
        // print how many are there deers wolves etc
        let mut counts = std::collections::HashMap::new();
        for entity in &self.entities {
            *counts.entry(&entity.entity_type).or_insert(0) += 1;
        }

        (
            *counts.get(&EntityType::Deer).unwrap_or(&0),
            *counts.get(&EntityType::Boar).unwrap_or(&0),
            *counts.get(&EntityType::Wolf).unwrap_or(&0),
            *counts.get(&EntityType::Raven).unwrap_or(&0),
            *counts.get(&EntityType::Corpse).unwrap_or(&0),
        )
    }
}
