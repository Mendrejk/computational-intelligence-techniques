use genevo::{operator::prelude::*, population::*, prelude::*, types::fmt::Display};
use smallvec::SmallVec;

#[derive(Debug, Clone)]
struct Dish {
    name: String,
    calories: u32,
    price: u32,
    carbs: u32,
    fats: u32,
    proteins: u32,
}

#[derive(Debug, Clone)]
struct AllDishes {
    dishes: Vec<Dish>,
}

impl From<Vec<Dish>> for AllDishes {
    fn from(items: Vec<Dish>) -> Self {
        AllDishes { dishes: items }
    }
}

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
type Selection = SmallVec<[u32; 16]>;

/// How do the genes of the genotype show up in the phenotype
trait AsPhenotype {
    fn as_diet(&self, all_dishes: &AllDishes) -> Diet;
}

impl AsPhenotype for Selection {
    fn as_diet(&self, all_dishes: &AllDishes) -> Diet {
        let dishes: Vec<(Dish, u32)> = self
            .into_iter()
            .enumerate()
            .filter_map(|(index, dish_count)| {
                if *dish_count > 0 {
                    Some((all_dishes.dishes[index].clone(), *dish_count))
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
struct Problem {
    all_dishes: AllDishes,
    target_calories: u64,
    target_carbs: u64,
    target_fats: u64,
    target_proteins: u64,
}

impl Problem {
    pub fn new(target_calories: u64, target_carbs: u64, target_fats: u64, target_proteins: u64, all_dishes: AllDishes) -> Self {
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
impl<'a> FitnessFunction<Selection, i64> for &'a Problem {
    fn fitness_of(&self, selection: &Selection) -> i64 {
        let (total_calories, total_carbs, total_fats, total_proteins, total_price) = selection
            .iter()
            .enumerate()
            .filter_map(|(index, dish_count)| {
                if *dish_count > 0 {
                    let dish = &self.all_dishes.dishes[index];
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

        let nil = -9999999999;
        let mut sum: i64 = 0;
        if total_calories.abs_diff(self.target_calories) > 200 {
            sum += nil;
        }
        if total_carbs.abs_diff(self.target_carbs) > 25 {
            sum += nil
        }
        if total_fats.abs_diff(self.target_fats) > 75 {
            sum += nil
        }
        if total_proteins.abs_diff(self.target_proteins) > 50 {
            sum += nil
        }

        sum - total_price
    }

    fn average(&self, values: &[i64]) -> i64 {
        (values.iter().sum::<i64>() as f64 / values.len() as f64 + 0.5).floor() as i64
    }

    fn highest_possible_fitness(&self) -> i64 {
        0
    }

    fn lowest_possible_fitness(&self) -> i64 { -9999999999 }
}

fn main() {
    run(get_dishes());
}

fn run(all_dishes: AllDishes) {
    let problem = Problem::new(2250, 275, 50, 120, all_dishes);

    let initial_population: Population<Selection> = build_population()
        .with_genome_builder(ValueEncodedGenomeBuilder::new(
            problem.all_dishes.dishes.len(), 0, 5,
        ))
        .of_size(150)
        .uniform_at_random();

    let mut diet_sim = simulate(
        genetic_algorithm()
            .with_evaluation(&problem)
            .with_selection(MaximizeSelector::new(0.85, 12))
            // .with_selection(  RouletteWheelSelector::new(0.85, 12))
            .with_crossover(SinglePointCrossBreeder::new())
            .with_mutation(RandomValueMutator::new(0.1, 0, 5))
            .with_reinsertion(ElitistReinserter::new(&problem, false, 0.85))
            // .with_reinsertion(UniformReinserter::new(0.85))
            .with_initial_population(initial_population)
            .build(),
    )
        .until(GenerationLimit::new(1000))
        .build();

    'sim: loop {
        let result = diet_sim.step();

        match result {
            Ok(SimResult::Intermediate(step)) => {
                // println!("{:?}", &step.result.evaluated_population);
                let evaluated_population = step.result.evaluated_population;
                let best_solution = step.result.best_solution;
                println!(
                    "step: generation: {}, average_fitness: {}, \
                     best fitness: {}, duration: {}, processing_time: {}",
                    step.iteration,
                    evaluated_population.average_fitness(),
                    best_solution.solution.fitness,
                    step.duration.fmt(),
                    step.processing_time.fmt(),
                );
                let diet = best_solution
                    .solution
                    .genome
                    .as_diet(&problem.all_dishes);
                println!(
                    "      Diet: number of unique dishes: {}, total calories: {}, total price: {}",
                    diet.dishes.len(),
                    diet.total_calories,
                    diet.price,
                );
            }
            Ok(SimResult::Final(step, processing_time, duration, stop_reason)) => {
                let best_solution = step.result.best_solution;
                println!("{}", stop_reason);
                println!(
                    "Final result after {}: generation: {}, \
                     best solution with fitness {} found in generation {}, processing_time: {}",
                    duration.fmt(),
                    step.iteration,
                    best_solution.solution.fitness,
                    best_solution.generation,
                    processing_time.fmt(),
                );
                let diet = best_solution
                    .solution
                    .genome
                    .as_diet(&problem.all_dishes);
                println!(
                    "      Diet: number of unique dishes: {}, total calories: {}, total price: {}",
                    diet.dishes.len(),
                    diet.total_calories,
                    diet.price,
                );
                println!("{:?}", diet.dishes.into_iter().map(|(dish, count)| (dish.name, count)).collect::<Vec<(String, u32)>>());
                break 'sim;
            }
            Err(error) => {
                println!("{}", error);
                break 'sim;
            }
        }
    }
}

fn get_dishes() -> AllDishes {
// a list of 15 dishes with polish zloty * 10 as price
    vec![
        Dish {
            name: "potatoes, 100g".into(),
            calories: 66,
            price: 2,
            carbs: 15,
            fats: 0,
            proteins: 2,
        },
        Dish {
            name: "brown rice, 100g".into(),
            calories: 111,
            price: 5,
            carbs: 23,
            fats: 1,
            proteins: 2,
        },
        Dish {
            name: "chicken breast, 100g".into(),
            calories: 165,
            price: 35,
            carbs: 0,
            fats: 3,
            proteins: 31,
        },
        Dish {
            name: "salmon fillet, 100g".into(),
            calories: 206,
            price: 65,
            carbs: 0,
            fats: 13,
            proteins: 22,
        },
        Dish {
            name: "spinach, 100g".into(),
            calories: 23,
            price: 5,
            carbs: 3,
            fats: 0,
            proteins: 3,
        },
        Dish {
            name: "broccoli, 100g".into(),
            calories: 34,
            price: 5,
            carbs: 7,
            fats: 0,
            proteins: 3,
        },
        Dish {
            name: "carrots, 100g".into(),
            calories: 41,
            price: 3,
            carbs: 10,
            fats: 0,
            proteins: 1,
        },
        Dish {
            name: "sweet potato, 100g".into(),
            calories: 86,
            price: 4,
            carbs: 20,
            fats: 0,
            proteins: 1,
        },
        Dish {
            name: "oatmeal, per serving (40g)".into(),
            calories: 68,
            price: 4,
            carbs: 12,
            fats: 1,
            proteins: 2,
        },
        Dish {
            name: "eggs, per piece".into(),
            calories: 78,
            price: 3,
            carbs: 0,
            fats: 5,
            proteins: 6,
        },
        Dish {
            name: "cottage cheese, per serving (200g)".into(),
            calories: 232,
            price: 25,
            carbs: 8,
            fats: 10,
            proteins: 28,
        },
        Dish {
            name: "yogurt, per serving (200g)".into(),
            calories: 120,
            price: 12,
            carbs: 16,
            fats: 2,
            proteins: 6,
        },
        Dish {
            name: "tuna can, drained (150g)".into(),
            calories: 150,
            price: 45,
            carbs: 0,
            fats: 2,
            proteins: 33,
        },
        Dish {
            name: "chickpeas can, drained (240g)".into(),
            calories: 360,
            price: 25,
            carbs: 60,
            fats: 6,
            proteins: 20,
        },
        Dish {
            name: "apple".into(),
            calories: 52,
            price: 12,
            carbs: 14,
            fats: 0,
            proteins: 0,
        },
    ].into()
}