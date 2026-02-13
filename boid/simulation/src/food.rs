use crate::*;

#[derive(Debug)]
pub struct Food {
    pub(crate) position: na::Point2<f32>,
}

impl Food {
    pub fn random<R: RngCore>(rng: &mut R) -> Self {
        Self {
            position: rng.random(),
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}
