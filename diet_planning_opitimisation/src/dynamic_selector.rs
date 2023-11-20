use genevo::algorithm::EvaluatedPopulation;
use genevo::genetic::{AsScalar, Parents};
use genevo::operator::{GeneticOperator, MultiObjective, SelectionOp, SingleObjective};
use genevo::operator::prelude::{MaximizeSelector, RouletteWheelSelector, TournamentSelector};
use genevo::prelude::{*};

#[derive(Clone, Debug, PartialEq)]
pub enum GenevoSelector {
    Maximize(MaximizeSelector),
    Roulette(RouletteWheelSelector),
    Tournament(TournamentSelector),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DynamicSelector {
    selector: GenevoSelector,
}

impl DynamicSelector {
    pub fn new(selector: GenevoSelector) -> DynamicSelector {
        DynamicSelector { selector }
    }
}

impl SingleObjective for DynamicSelector {}

impl MultiObjective for DynamicSelector {}

impl GeneticOperator for DynamicSelector {
    fn name() -> String {
        "variable_selector".to_string()
    }
}

impl<G, F> SelectionOp<G, F> for DynamicSelector
    where
        G: Genotype,
        F: Fitness + AsScalar,
{
    fn select_from<R>(&self, evaluated: &EvaluatedPopulation<G, F>, rng: &mut R) -> Vec<Parents<G>>
        where
            R: Rng + Sized,
    {
        match &self.selector {
            GenevoSelector::Maximize(selector) => selector.select_from(evaluated, rng),
            GenevoSelector::Roulette(selector) => selector.select_from(evaluated, rng),
            GenevoSelector::Tournament(selector) => selector.select_from(evaluated, rng)
        }
    }
}
