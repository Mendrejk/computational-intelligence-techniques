use genevo::{operator::prelude::*, population::*, prelude::*, types::fmt::Display};
use smallvec::SmallVec;

#[derive(Debug, Clone)]
struct Dish {
    name: String,
    calories: u32,
    price: u32,
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
        let price = dishes.iter().map(|(dish, count)| (dish.price as u32 * count) as u64).sum::<u64>();

        Diet {
            dishes,
            total_calories,
            price,
        }
    }
}

/// The problem definition
#[derive(Debug, Clone)]
struct Problem {
    all_dishes: AllDishes,
    target_calories: u64,
}

impl Problem {
    pub fn new(target_calories: u64, all_dishes: AllDishes) -> Self {
        Self {
            all_dishes,
            target_calories,
        }
    }
}

/// The fitness function for `Selection`
impl<'a> FitnessFunction<Selection, i64> for &'a Problem {
    fn fitness_of(&self, selection: &Selection) -> i64 {
        let (total_calories, total_price) = selection
            .iter()
            .enumerate()
            .filter_map(|(index, dish_count)| {
                if *dish_count > 0 {
                    let dish = &self.all_dishes.dishes[index];
                    Some((u64::from(dish.calories * dish_count), i64::from(dish.price * dish_count)))
                } else {
                    None
                }
            })
            .fold((0, 0), |(total_clrs, total_prc), (calories, price)| {
                (total_clrs + calories, total_prc + price)
            });

        return if total_calories > self.target_calories + 250 || total_calories < self.target_calories - 250 {
            // println!("{}", -213742000);
            -213742000
        } else {
            // println!("{}", -total_price);
            -total_price
        }
    }

    fn average(&self, values: &[i64]) -> i64 {
        (values.iter().sum::<i64>() as f64 / values.len() as f64 + 0.5).floor() as i64
    }

    fn highest_possible_fitness(&self) -> i64 {
        0
    }

    fn lowest_possible_fitness(&self) -> i64 {
        -213742000
    }
}

fn main() {
    // a list of 15 dishes with polish zloty as price
    let all_dishes: AllDishes = vec![
        Dish {
            name: "potatoes, 100g".into(),
            calories: 66,
            price: 2,
        },
        Dish {
            name: "brown rice, 100g".into(),
            calories: 111,
            price: 5,
        },
        Dish {
            name: "chicken breast, 100g".into(),
            calories: 165,
            price: 35,
        },
        Dish {
            name: "salmon fillet, 100g".into(),
            calories: 206,
            price: 65,
        },
        Dish {
            name: "spinach, 100g".into(),
            calories: 23,
            price: 5,
        },
        Dish {
            name: "broccoli, 100g".into(),
            calories: 34,
            price: 5,
        },
        Dish {
            name: "carrots, 100g".into(),
            calories: 41,
            price: 3,
        },
        Dish {
            name: "sweet potato, 100g".into(),
            calories: 86,
            price: 4,
        },
        Dish {
            name: "oatmeal, 100g".into(),
            calories: 68,
            price: 4,
        },
        Dish {
            name: "eggs, per piece".into(),
            calories: 78,
            price: 3,
        },
        Dish {
            name: "cottage cheese, per serving (200g)".into(),
            calories: 232,
            price: 25,
        },
        Dish {
            name: "yogurt, per serving (200g)".into(),
            calories: 120,
            price: 12,
        },
        Dish {
            name: "tuna can, drained (150g)".into(),
            calories: 150,
            price: 45,
        },
        Dish {
            name: "chickpeas can, drained (240g)".into(),
            calories: 360,
            price: 25,
        },
        Dish {
            name:"apple".into(),
            calories :52 ,
            price : 12
        }
    ].into();

    let problem = Problem::new(2000, all_dishes);

    let initial_population: Population<Selection> = build_population()
        .with_genome_builder(ValueEncodedGenomeBuilder::new(
            problem.all_dishes.dishes.len(), 0, 5
        ))
        .of_size(100)
        .uniform_at_random();

    let mut diet_sim = simulate(
        genetic_algorithm()
            .with_evaluation(&problem)
            .with_selection( MaximizeSelector::new(0.85, 12))
            // .with_selection(  RouletteWheelSelector::new(0.85, 12))
            .with_crossover(SinglePointCrossBreeder::new())
            .with_mutation(RandomValueMutator::new(0.1, 0, 5))
            .with_reinsertion(ElitistReinserter::new(&problem, false, 0.85))
            // .with_reinsertion(UniformReinserter::new(0.85))
            .with_initial_population(initial_population)
            .build(),
    )
        .until(GenerationLimit::new(200))
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
            },
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
                break 'sim;
            },
            Err(error) => {
                println!("{}", error);
                break 'sim;
            },
        }
    }
}