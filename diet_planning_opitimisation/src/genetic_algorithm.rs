use genevo::operator::prelude::*;
use genevo::population::ValueEncodedGenomeBuilder;
use genevo::prelude::*;
use crate::dish::{Dish, get_dishes};
use crate::dynamic_selector::{DynamicSelector, GenevoSelector};
use crate::inversion_mutator::InversionMutator;

/// The phenotype
#[derive(Debug)]
struct Diet {
    dishes: Vec<(Dish, u32)>,
    total_calories: u64,
    total_carbs: u64,
    total_fats: u64,
    total_proteins: u64,
    price: u64,
}

/// The genotype
type Selection = Vec<u32>;

/// How do the genes of the genotype show up in the phenotype
trait AsPhenotype {
    fn as_diet(&self, all_dishes: &[Dish]) -> Diet;
}

impl AsPhenotype for Selection {
    fn as_diet(&self, all_dishes: &[Dish]) -> Diet {
        let dishes: Vec<(Dish, u32)> = self
            .iter()
            .enumerate()
            .filter_map(|(index, dish_count)| {
                if *dish_count > 0 {
                    Some((all_dishes[index].clone(), *dish_count))
                } else {
                    None
                }
            })
            .collect();

        let total_calories = dishes.iter().map(|(dish, count)| (dish.calories * count) as u64).sum::<u64>();
        let price = dishes.iter().map(|(dish, count)| (dish.price * count) as u64).sum::<u64>();
        let total_carbs = dishes.iter().map(|(dish, count)| (dish.carbs * count) as u64).sum::<u64>();
        let total_fats = dishes.iter().map(|(dish, count)| (dish.fats * count) as u64).sum::<u64>();
        let total_proteins = dishes.iter().map(|(dish, count)| (dish.proteins * count) as u64).sum::<u64>();

        Diet {
            dishes,
            total_calories,
            price,
            total_carbs,
            total_fats,
            total_proteins,
        }
    }
}

/// The problem definition
#[derive(Debug, Clone)]
struct Problem<'a> {
    all_dishes: &'a Vec<Dish>,
    target_calories: u64,
    target_carbs: u64,
    target_fats: u64,
    target_proteins: u64,
}

impl<'a> Problem<'a> {
    pub fn new(target_calories: u64, target_carbs: u64, target_fats: u64, target_proteins: u64, all_dishes: &'a Vec<Dish>) -> Self {
        Self {
            all_dishes,
            target_calories,
            target_carbs,
            target_fats,
            target_proteins,
        }
    }
}

/// The fitness function for `Selection`
impl<'a, 'b> FitnessFunction<Selection, i64> for &'b Problem<'a> {
    fn fitness_of(&self, selection: &Selection) -> i64 {
        let (total_calories, total_carbs, total_fats, total_proteins, total_price) = selection
            .iter()
            .enumerate()
            .filter_map(|(index, dish_count)| {
                if *dish_count > 0 {
                    let dish = &self.all_dishes[index];
                    Some(((u64::from(dish.calories * dish_count)),
                          (u64::from(dish.carbs * dish_count)),
                          (u64::from(dish.fats * dish_count)),
                          (u64::from(dish.proteins * dish_count)),
                          i64::from(dish.price * dish_count)))
                } else {
                    None
                }
            })
            .fold((0, 0, 0, 0, 0), |(total_calories, total_carbs, total_fats, total_proteins, total_price), (calories, carbs, fats, proteins, price)| {
                (total_calories + calories, total_carbs + carbs, total_fats + fats, total_proteins + proteins, total_price + price)
            });

        let nil = -99999;
        let mut sum: i64 = 9999999;
        if total_calories.abs_diff(self.target_calories) > 200 {
            sum += nil;
        }
        if total_carbs.abs_diff(self.target_carbs) > 25 {
            sum += nil
        }
        if total_fats.abs_diff(self.target_fats) > 10 {
            sum += nil
        }
        if total_proteins.abs_diff(self.target_proteins) > 20 {
            sum += nil
        }

        sum - total_price.pow(2) + (total_calories + total_carbs + total_fats + total_proteins) as i64
    }

    fn average(&self, values: &[i64]) -> i64 {
        (values.iter().sum::<i64>() as f64 / values.len() as f64 + 0.5).floor() as i64
    }

    fn highest_possible_fitness(&self) -> i64 {
        0
    }

    fn lowest_possible_fitness(&self) -> i64 { -9999999 }
}

fn run(selector: DynamicSelector, all_dishes: &Vec<Dish>, generation_count: u64) -> (i64, usize) {
    let problem = Problem::new(2250, 275, 50, 120, all_dishes);
    let max_dish_count = 10;

    let initial_population: Population<Selection> = build_population()
        .with_genome_builder(ValueEncodedGenomeBuilder::new(
            problem.all_dishes.len(), 0, max_dish_count,
        ))
        .of_size(15)
        .uniform_at_random();

    let mut diet_sim = simulate(
        genetic_algorithm()
            .with_evaluation(&problem)
            .with_selection(selector)
            .with_crossover(SinglePointCrossBreeder::new())
            .with_mutation(InversionMutator { mutation_rate: 0.1 })
            .with_reinsertion(ElitistReinserter::new(&problem, false, 0.85))
            // .with_reinsertion(UniformReinserter::new(0.85))
            .with_initial_population(initial_population)
            .build(),
    )
        .until(GenerationLimit::new(generation_count))
        .build();

    let mut fitness: i64 = 0;
    let mut unique_dishes: usize = 0;

    'sim: loop {
        let result = diet_sim.step();

        match result {
            Ok(SimResult::Intermediate(step)) => {
                // println!("{:?}", &step.result.evaluated_population);
                // let evaluated_population = step.result.evaluated_population;
                // let best_solution = step.result.best_solution;
                // println!(
                //     "step: generation: {}, average_fitness: {}, \
                //      best fitness: {}, duration: {}, processing_time: {}",
                //     step.iteration,
                //     evaluated_population.average_fitness(),
                //     best_solution.solution.fitness,
                //     step.duration.fmt(),
                //     step.processing_time.fmt(),
                // );
                // let diet = best_solution
                //     .solution
                //     .genome
                //     .as_diet(&problem.all_dishes);
                // println!(
                //     "      Diet: number of unique dishes: {}, total calories: {}, total price: {}",
                //     diet.dishes.len(),
                //     diet.total_calories,
                //     diet.price,
                // );
            }
            Ok(SimResult::Final(step, processing_time, duration, stop_reason)) => {
                let best_solution = step.result.best_solution;
                // println!("{}", stop_reason);
                // println!(
                //     "Final result after {}: generation: {}, \
                //      best solution with fitness {} found in generation {}, processing_time: {}",
                //     duration.fmt(),
                //     step.iteration,
                //     best_solution.solution.fitness,
                //     best_solution.generation,
                //     processing_time.fmt(),
                // );
                // let diet = best_solution
                //     .solution
                //     .genome
                //     .as_diet(problem.all_dishes);
                // println!(
                //     "      Diet: number of unique dishes: {}, total calories: {}, total price: {}",
                //     diet.dishes.len(),
                //     diet.total_calories,
                //     diet.price,
                // );
                // println!("{:?}", diet.dishes.into_iter().map(|(dish, count)| (dish.name, count)).collect::<Vec<(String, u32)>>());
                fitness = best_solution.solution.fitness;
                unique_dishes = best_solution.solution.genome.as_diet(problem.all_dishes).dishes.len();
                // println!("fitness: {}, unique dishes: {}", fitness, unique_dishes);
                break 'sim;
            }
            Err(error) => {
                println!("{}", error);
                break 'sim;
            }
        }
    }

    (fitness, unique_dishes)
}

pub fn gen_algorithm()
{
    let dishes = get_dishes();

    // let selector = DynamicSelector::new(GenevoSelector::Maximize(MaximizeSelector::new(0.85, 12)));
    // let selector = DynamicSelector::new(GenevoSelector::Roulette(RouletteWheelSelector::new(0.85, 12)));
    let selector = DynamicSelector::new(GenevoSelector::Tournament(TournamentSelector::new(0.85, 12, 3, 1.0, false)));

    let gen_fitness_dishes: Vec<(u64, i64, usize)> = [1, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 10000].into_iter().map(|generation_count| {
        let i = 5;
        let (fitness_sum, dish_count_sum) = (0..i)
            .map(|_| {
                run(selector.clone(), &dishes, generation_count)
            })
            .fold((0, 0), |(fitness_sum, dish_count_sum), (fitness, dish_count)| (fitness_sum + fitness, dish_count_sum + dish_count));

        (generation_count, fitness_sum / i, dish_count_sum / i as usize)
    }).collect();

    for (generation, average_fitness, average_dishes) in gen_fitness_dishes {
        // println!("|{}|{}|{}|", generation, average_fitness, average_dishes)
        println!("{}", average_fitness - 9000000)
    }
}