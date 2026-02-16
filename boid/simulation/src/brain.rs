use crate::*;

#[derive(Debug)]
pub struct Brain {
    pub(crate) nn: nn::Network,
}

impl Brain {
    pub fn random<R: RngCore>(rng: &mut R, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::random(rng, &Self::topology(eye)).unwrap(),
        }
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.nn.weights().collect()
    }

    pub(crate) fn from_chromosome(chromosome: ga::Chromosome, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::from_weights(&Self::topology(eye), chromosome),
        }
    }

    fn topology(eye: &Eye) -> [nn::LayerTopology; 3] {
        [
            nn::LayerTopology {
                neurons: eye.cells(),
            },
            nn::LayerTopology {
                neurons: 2 * eye.cells(),
            },
            nn::LayerTopology { neurons: 2 },
        ]
    }
}
