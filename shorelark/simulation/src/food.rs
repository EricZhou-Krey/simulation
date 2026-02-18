use crate::*;
use nalgebra::Point2;

#[derive(Debug)]
pub struct Food {
    pub(crate) position: Point2<f32>,
}

impl Food {
    pub fn random<R: RngCore>(rng: &mut R) -> Self {
        Self {
            position: rng.random(),
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }
}
