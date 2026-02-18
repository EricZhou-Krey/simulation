use rand::{Rng, RngCore};
use std::iter::once;

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn random<R: RngCore>(rng: &mut R, layers: &[LayerTopology]) -> Result<Self, String> {
        if layers.len() <= 1 {
            return Err(format!(
                "Cannot make neural network of {} layers",
                layers.len()
            ));
        }
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();
        Ok(Self { layers })
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Result<Vec<f32>, String> {
        self.layers
            .iter()
            .try_fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .copied()
    }
    pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("got too many weights");
        }

        Self { layers }
    }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn random<R: RngCore>(rng: &mut R, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();
        Self { neurons }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Result<Vec<f32>, String> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self { neurons }
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    fn random<R: RngCore>(rng: &mut R, input_size: usize) -> Self {
        let bias = rng.random_range(-1.0..=1.0);
        let weights = (0..input_size)
            .map(|_| rng.random_range(-1.0..=1.0))
            .collect();
        Self { bias, weights }
    }

    fn propagate(&self, inputs: &[f32]) -> Result<f32, String> {
        if inputs.len() != self.weights.len() {
            return Err(format!(
                "Got {} inputs, have {} weights",
                inputs.len(),
                self.weights.len(),
            ));
        }

        Ok(inputs
            .iter()
            .zip(&self.weights)
            .fold(self.bias, |output, (input, weight)| {
                output + (input * weight)
            })
            .max(0.0))
    }

    fn from_weights(input_size: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().expect("got not enough weights");

        let weights = (0..input_size)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();

        Self { bias, weights }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    mod network {
        use super::*;

        #[test]
        fn random() {
            let mut rng = StdRng::seed_from_u64(42);
            let topology = vec![LayerTopology { neurons: 3 }, LayerTopology { neurons: 2 }];
            let res = Network::random(&mut rng, &topology);

            assert!(res.is_ok());
            let network = res.unwrap();
            assert_eq!(network.layers.len(), 1);
            assert_eq!(network.layers[0].neurons.len(), 2);
        }

        #[test]
        fn propagate() {
            let mut rng = StdRng::seed_from_u64(42);
            let topology = vec![
                LayerTopology { neurons: 2 },
                LayerTopology { neurons: 3 },
                LayerTopology { neurons: 1 },
            ];
            let network = Network::random(&mut rng, &topology).unwrap();

            let result = network.propagate(vec![0.5, 0.5]);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().len(), 1);
        }

        #[test]
        fn fail_on_invalid_topology() {
            let mut rng = rand::rng();
            let res = Network::random(&mut rng, &[LayerTopology { neurons: 3 }]);
            assert!(res.is_err());
        }
        #[test]
        fn weights() {
            let network = Network {
                layers: vec![
                    Layer {
                        neurons: vec![Neuron {
                            bias: 0.1,
                            weights: vec![0.2, 0.3, 0.4],
                        }],
                    },
                    Layer {
                        neurons: vec![Neuron {
                            bias: 0.5,
                            weights: vec![0.6, 0.7, 0.8],
                        }],
                    },
                ],
            };

            let actual: Vec<_> = network.weights().collect();
            let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

            assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }
        #[test]
        fn from_weights() {
            let layers = &[LayerTopology { neurons: 3 }, LayerTopology { neurons: 2 }];

            let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
            let network = Network::from_weights(layers, weights.clone());
            let actual: Vec<_> = network.weights().collect();

            assert_relative_eq!(actual.as_slice(), weights.as_slice());
        }
    }

    mod layer {
        use super::*;

        #[test]
        fn random() {
            let mut rng = StdRng::seed_from_u64(42);
            let layer = Layer::random(&mut rng, 4, 2);
            assert_eq!(layer.neurons.len(), 2);
            assert_eq!(layer.neurons[0].weights.len(), 4);
        }

        #[test]
        fn propagate() {
            let neuron1 = Neuron {
                bias: 0.1,
                weights: vec![1.0, 0.0],
            };
            let neuron2 = Neuron {
                bias: 0.2,
                weights: vec![0.0, 1.0],
            };
            let layer = Layer {
                neurons: vec![neuron1, neuron2],
            };

            let inputs = vec![0.5, 0.5];
            let outputs = layer.propagate(inputs).unwrap();

            assert_relative_eq!(outputs[0], 0.6); // 0.5 * 1.0 + 0.1
            assert_relative_eq!(outputs[1], 0.7); // 0.5 * 1.0 + 0.2
        }
    }

    mod neuron {
        use super::*;

        #[test]
        fn random() {
            let mut rng = StdRng::seed_from_u64(4);
            let neuron = Neuron::random(&mut rng, 4);

            assert_relative_eq!(neuron.bias, 0.3885615);
            assert_relative_eq!(
                neuron.weights.as_slice(),
                [0.06600618, -0.8327689, 0.31393528, 0.3456552].as_ref()
            );
        }

        #[test]
        fn propagate() {
            let neuron = Neuron {
                bias: 0.5,
                weights: vec![-0.3, 0.8],
            };

            assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]).unwrap(), 0.0);

            assert_relative_eq!(
                neuron.propagate(&[0.5, 1.0]).unwrap(),
                (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
            );
        }

        #[test]
        fn fail_on_wrong_input_size() {
            let neuron = Neuron {
                bias: 0.0,
                weights: vec![1.0],
            };
            assert!(neuron.propagate(&[1.0, 2.0]).is_err());
        }
    }
}
