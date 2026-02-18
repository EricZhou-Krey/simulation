use crate::*;
use genetic_algorithm::Chromosome;
use neural_network::{LayerTopology, Network};

#[derive(Debug)]
pub struct Brain {
    pub(crate) nn: Network,
}

impl Brain {
    pub fn random<R: RngCore>(rng: &mut R, eye: &Eye) -> Self {
        Self {
            nn: Network::random(rng, &Self::topology(eye)).unwrap(),
        }
    }

    pub(crate) fn as_chromosome(&self) -> Chromosome {
        self.nn.weights().collect()
    }

    pub(crate) fn from_chromosome(chromosome: Chromosome, eye: &Eye) -> Self {
        Self {
            nn: Network::from_weights(&Self::topology(eye), chromosome),
        }
    }

    fn topology(eye: &Eye) -> [LayerTopology; 3] {
        [
            LayerTopology {
                neurons: eye.cells(),
            },
            LayerTopology {
                neurons: 2 * eye.cells(),
            },
            LayerTopology { neurons: 2 },
        ]
    }
}
