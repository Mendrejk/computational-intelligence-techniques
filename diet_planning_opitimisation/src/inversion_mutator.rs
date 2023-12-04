use std::fmt::Debug;
use genevo::operator::{GeneticOperator, MutationOp};
use genevo::prelude::Rng;
use genevo::random::random_cut_points;

#[derive(Clone, Debug, PartialEq)]
pub struct InversionMutator {
    pub mutation_rate: f64,
}

impl GeneticOperator for InversionMutator {
    fn name() -> String {
        "Inversion-Mutation".to_string()
    }
}

impl<V> MutationOp<Vec<V>> for InversionMutator
    where
        V: Clone + Debug + PartialEq + Send + Sync,
{
    fn mutate<R>(&self, mut genome: Vec<V>, rng: &mut R) -> Vec<V>
        where
            R: Rng + Sized,
    {
        if rng.gen::<f64>() > self.mutation_rate {
            return genome;
        }

        let (locus1, locus2) = random_cut_points(rng, genome.len());
        let _ = &genome[locus1..=locus2].reverse();
        genome
    }
}
