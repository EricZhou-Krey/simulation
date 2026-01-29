use rand::{seq::IndexedRandom, Rng, RngCore};
use std::ops::Index;

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

#[derive(Debug)]
pub struct GeneticAlgorithm<S, C> {
    selection_method: S,
    crossover_method: C,
}
impl<S: SelectionMethod, C: CrossoverMethod> GeneticAlgorithm<S, C> {
    pub fn new(selection_method: S, crossover_method: C) -> Self {
        Self {
            selection_method,
            crossover_method,
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
        if chance < 0.0 || chance > 1.0 {
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

    #[derive(Clone, Debug)]
    struct TestIndividual {
        fitness: f32,
    }

    impl TestIndividual {
        fn new(fitness: f32) -> Self {
            Self { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            self.fitness
        }
        fn chromosome(&self) -> &Chromosome {
            panic!("Chromosome not supported for TestIndividual")
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
}
