use crate::*;

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: nn::Network,
}
impl Animal {
    pub fn random<R: RngCore>(rng: &mut R) -> Self {
        let eye = Eye::default();
        let brain = nn::Network::random(
            rng,
            &[
                nn::LayerTopology {
                    neurons: eye.cells(),
                },
                nn::LayerTopology {
                    neurons: 2 * eye.cells(),
                },
                nn::LayerTopology { neurons: 2 },
            ],
        )
        .unwrap();
        Self {
            position: rng.random(),
            rotation: rng.random(),
            speed: 0.002,
            eye,
            brain,
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}
