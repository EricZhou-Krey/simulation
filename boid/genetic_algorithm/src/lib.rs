pub struct GeneticAlgorithm {}
impl GeneticAlgorithm {
    pub fn evolve<I>(&self, population: &[I]) -> Result<Vec<I>, String> {
        if population.is_empty() {
            return Err("Cannot evolve empty population".to_string());
        }
        (0..population.len()).map(|_| todo!()).collect()
    }
}
