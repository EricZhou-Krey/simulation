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
use genetic_algorithm as ga;
use nalgebra as na;
use neural_network as nn;
use rand::{Rng, RngCore};
