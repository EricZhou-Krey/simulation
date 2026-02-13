mod animal;
mod food;
mod world;

pub use self::{animal::*, food::*, world::*};
use nalgebra as na;
use rand::{Rng, RngCore};

#[derive(Debug)]
pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random<R: RngCore>(rng: &mut R) -> Self {
        Self {
            world: World::random(rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step<R: RngCore>(&mut self, rng: &mut R) {
        self.process_collisions(rng);
        self.process_movements();
    }

    fn process_collisions<R: RngCore>(&mut self, rng: &mut R) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);

                if distance <= 0.01 {
                    food.position = rng.random();
                }
            }
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }
}
