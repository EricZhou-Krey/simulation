use crate::*;
use genetic_algorithm::{Chromosome, Individual};

pub struct AnimalIndividual {
    fitness: f32,
    chromosome: Chromosome,
}

impl Individual for AnimalIndividual {
    fn create(chromosome: Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }

    fn chromosome(&self) -> &Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl AnimalIndividual {
    pub fn from_animal(animal: &Animal) -> Self {
        Self {
            fitness: animal.satiation as f32,
            chromosome: animal.as_chromosome(),
        }
    }
    pub fn into_animal<R: RngCore>(self, rng: &mut R) -> Animal {
        Animal::from_chromosome(self.chromosome, rng)
    }
}
