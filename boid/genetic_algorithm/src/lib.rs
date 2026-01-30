use rand::{seq::IndexedRandom, Rng, RngCore};
use std::ops::Index;

pub trait Individual {
    fn create(chromosome: Chromosome) -> Self;
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

#[derive(Debug)]
pub struct GeneticAlgorithm<S, C, M> {
    selection_method: S,
    crossover_method: C,
    mutation_method: M,
}
impl<S: SelectionMethod, C: CrossoverMethod, M: MutationMethod> GeneticAlgorithm<S, C, M> {
    pub fn new(selection_method: S, crossover_method: C, mutation_method: M) -> Self {
        Self {
            selection_method,
            crossover_method,
            mutation_method,
        }
    }
    pub fn evolve<I: Individual, R: RngCore>(
        &self,
        rng: &mut R,
        population: &[I],
    ) -> Result<Vec<I>, String> {
        if population.is_empty() {
            return Err("Cannot evolve empty population".to_string());
        }
        Ok((0..population.len())
            .map(|_| {
                let chromosome_a = self
                    .selection_method
                    .select(rng, population)
                    .unwrap()
                    .chromosome();
                let chromosome_b = self
                    .selection_method
                    .select(rng, population)
                    .unwrap()
                    .chromosome();

                let mut cross_chromosome = self
                    .crossover_method
                    .crossover(rng, chromosome_a, chromosome_b)
                    .unwrap();

                self.mutation_method.mutate(rng, &mut cross_chromosome);

                I::create(cross_chromosome)
            })
            .collect())
    }
}

pub trait SelectionMethod {
    fn select<'a, I: Individual, R: RngCore>(
        &self,
        rng: &mut R,
        population: &'a [I],
    ) -> Result<&'a I, String>;
}

pub struct RouletteWheelSelection;
impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I: Individual, R: RngCore>(
        &self,
        rng: &mut R,
        population: &'a [I],
    ) -> Result<&'a I, String> {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .map_err(|_| "Cannot select from empty population".to_string())
    }
}

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.genes.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<f32>;
    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

pub trait CrossoverMethod {
    fn crossover<R: RngCore>(
        &self,
        rng: &mut R,
        chromosome_a: &Chromosome,
        chromosome_b: &Chromosome,
    ) -> Result<Chromosome, String>;
}

#[derive(Clone, Debug)]
pub struct UniformCrossover;
impl CrossoverMethod for UniformCrossover {
    fn crossover<R: RngCore>(
        &self,
        rng: &mut R,
        chromosome_a: &Chromosome,
        chromosome_b: &Chromosome,
    ) -> Result<Chromosome, String> {
        if chromosome_a.len() != chromosome_b.len() {
            return Err(format!(
                "Chromosome a of length {}, not compatible with chromosome b of length {}",
                chromosome_a.len(),
                chromosome_b.len()
            ));
        }
        Ok(chromosome_a
            .iter()
            .zip(chromosome_b.iter())
            .map(|(&a, &b)| if rng.random_bool(0.5) { a } else { b })
            .collect())
    }
}

pub trait MutationMethod {
    fn mutate<R: RngCore>(&self, rng: &mut R, cross_chromosome: &mut Chromosome);
}

#[derive(Clone, Debug)]
pub struct GaussianMutation {
    chance: f32,
    coefficient: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coefficient: f32) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&chance) {
            return Err(format!(
                "Probabilty has to be between 1.0 - 0.0 but was {}",
                chance
            ));
        }
        Ok(Self {
            chance,
            coefficient,
        })
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate<R: RngCore>(&self, rng: &mut R, cross_chromosome: &mut Chromosome) {
        cross_chromosome.iter_mut().for_each(|gene| {
            if rng.random_bool(self.chance as f64) {
                let sign = if rng.random_bool(0.5) { -1.0 } else { 1.0 };
                *gene += sign * self.coefficient * rng.random::<f32>();
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use std::collections::BTreeMap;
    use std::iter::FromIterator;

    #[derive(Clone, Debug, PartialEq)]
    enum TestIndividual {
        WithChromosome { chromosome: Chromosome },
        WithFitness { fitness: f32 },
    }

    impl TestIndividual {
        fn new(fitness: f32) -> Self {
            Self::WithFitness { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn create(chromosome: Chromosome) -> Self {
            Self::WithChromosome { chromosome }
        }

        fn chromosome(&self) -> &Chromosome {
            match self {
                Self::WithChromosome { chromosome } => chromosome,

                Self::WithFitness { .. } => {
                    panic!("not supported for TestIndividual::WithFitness")
                }
            }
        }

        fn fitness(&self) -> f32 {
            match self {
                Self::WithChromosome { chromosome } => chromosome.iter().sum(),

                Self::WithFitness { fitness } => *fitness,
            }
        }
    }

    impl PartialEq for Chromosome {
        fn eq(&self, other: &Self) -> bool {
            approx::relative_eq!(self.genes.as_slice(), other.genes.as_slice())
        }
    }

    #[test]
    fn uniform_crossover() {
        let mut rng = StdRng::seed_from_u64(42);
        let chromosome_a: Chromosome = (1..=100).map(|n| n as f32).collect();
        let chromosome_b: Chromosome = (1..=100).map(|n| -n as f32).collect();

        let cross_chromosome = UniformCrossover
            .crossover(&mut rng, &chromosome_a, &chromosome_b)
            .unwrap();

        let diff_a = cross_chromosome
            .iter()
            .zip(chromosome_a)
            .filter(|(c, p)| *c != p)
            .count();
        let diff_b = cross_chromosome
            .iter()
            .zip(chromosome_b)
            .filter(|(c, p)| *c != p)
            .count();

        assert_eq!(diff_a, 48);
        assert_eq!(diff_b, 52);
    }

    #[test]
    fn roulette_wheel_selection() {
        let mut rng = StdRng::seed_from_u64(42);
        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let mut actual_histogram = BTreeMap::new();
        for _ in 0..1000 {
            let Ok(individual) = RouletteWheelSelection
                .select(&mut rng, &population)
                .map_err(|e| eprintln!("Error: {e}"))
            else {
                continue;
            };
            let fitness = individual.fitness() as i32;
            *actual_histogram.entry(fitness).or_insert(0) += 1;
        }
        let expected_histogram = BTreeMap::from_iter([(1, 112), (2, 196), (3, 304), (4, 388)]);

        assert_eq!(actual_histogram, expected_histogram);
    }

    mod gaussian_mutation {
        use super::*;
        use approx::assert_relative_eq;

        fn actual(chance: f32, coeff: f32) -> Vec<f32> {
            let mut rng = StdRng::seed_from_u64(42);
            let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();

            GaussianMutation::new(chance, coeff)
                .unwrap()
                .mutate(&mut rng, &mut child);

            child.into_iter().collect()
        }

        mod given_zero_chance {
            use super::*;

            fn actual(coeff: f32) -> Vec<f32> {
                super::actual(0.0, coeff)
            }

            mod and_zero_coefficient {
                use super::*;

                #[test]
                fn does_not_change_the_original_chromosome() {
                    let actual = actual(0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }

            mod and_nonzero_coefficient {
                use super::*;

                #[test]
                fn does_not_change_the_original_chromosome() {
                    let actual = actual(0.5);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }

        mod given_fifty_fifty_chance {
            use super::*;

            mod and_zero_coefficient {
                use super::*;

                #[test]
                fn does_not_change_the_original_chromosome() {
                    let actual = actual(0.5, 0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }

            mod and_nonzero_coefficient {
                use super::*;

                #[test]
                fn slightly_changes_the_original_chromosome() {
                    let actual = actual(0.5, 0.5);
                    let expected = vec![1.0, 2.0, 3.0, 3.691288, 4.575374];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }

        mod given_max_chance {
            use super::*;

            mod and_zero_coefficient {
                use super::*;

                #[test]
                fn does_not_change_the_original_chromosome() {
                    let actual = actual(1.0, 0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }

            mod and_nonzero_coefficient {
                use super::*;

                #[test]
                fn entirely_changes_the_original_chromosome() {
                    let actual = actual(1.0, 0.5);
                    let expected = vec![1.1243691, 2.3182325, 2.5154905, 4.2074785, 5.0928855];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }
    }
}
