mod animal;
mod animal_individual;
mod brain;
mod eye;
mod food;
mod simulation;
mod world;

pub use self::{
    animal::*, animal_individual::*, brain::*, eye::*, food::*, simulation::*, world::*,
};
use rand::{Rng, RngCore};
