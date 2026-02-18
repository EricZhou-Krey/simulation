use crate::*;
use genetic_algorithm::Chromosome;
use nalgebra::{Point2, Rotation2};

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: Point2<f32>,
    pub(crate) rotation: Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: Brain,
    pub(crate) satiation: usize,
}
impl Animal {
    fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.random(),
            rotation: rng.random(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0,
        }
    }
    pub fn random<R: RngCore>(rng: &mut R) -> Self {
        let eye = Eye::default();
        let brain = Brain::random(rng, &eye);
        Self::new(eye, brain, rng)
    }

    pub(crate) fn as_chromosome(&self) -> Chromosome {
        self.brain.as_chromosome()
    }

    pub(crate) fn from_chromosome(chromosome: Chromosome, rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> Rotation2<f32> {
        self.rotation
    }
}
