mod animal;
mod eye;
mod food;
mod simulation;
mod world;

pub use self::{animal::*, eye::*, food::*, simulation::*, world::*};
use nalgebra as na;
use neural_network as nn;
use rand::{Rng, RngCore};
