use plotters::prelude::*;
use rayon::prelude::*;
use rand::Rng;
use std::sync::Mutex;
use crate::dish::{Dish, get_dishes};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DishCount {
    pub(crate) dish: usize,
    pub(crate) count: usize,
}

#[derive(Debug, Clone)]
struct Ant {
    current_dish: usize,
    path: Vec<DishCount>,
    total_price: u32,
    total_calories: u32,
    total_carbs: u32,
    total_fats: u32,
    total_proteins: u32,
    path_cost: f64,
}

impl Ant {
    fn new() -> Self {
        Self {
            current_dish: 0,
            path: vec![],
            total_price: 0,
            total_calories: 0,
            total_carbs: 0,
            total_fats: 0,
            total_proteins: 0,
            path_cost: 0.0,
        }
    }

    fn construct_path(&mut self, dishes: &[Dish], pheromone: &[f64]) {
        const PRICE_LIMIT: u32 = 200; // 20 zloty
        const TARGET_CALORIES: u32 = 2250;
        const TARGET_CARBS: u32 = 275;
        const TARGET_FATS: u32 = 50;
        const TARGET_PROTEINS: u32 = 120;

        while self.path.len() < dishes.len() * 10 && self.total_price <= PRICE_LIMIT {
            let next_dish = self.select_next_dish(dishes, pheromone);
            let dish_exists = self.path.iter_mut().find(|x| x.dish == next_dish);
            match dish_exists {
                Some(dish_count) => dish_count.count += 1,
                None => self.path.push(DishCount { dish: next_dish, count: 1 }),
            }
            self.total_price += dishes[next_dish].price;
            self.total_calories += dishes[next_dish].calories;
            self.total_carbs += dishes[next_dish].carbs;
            self.total_fats += dishes[next_dish].fats;
            self.total_proteins += dishes[next_dish].proteins;
            // self.path_cost = self.total_price as f64 - (self.total_price.pow(2) as f64) + (self.total_calories + self.total_carbs + self.total_fats + self.total_proteins) as f64;
            let normalized_price = self.total_price as f64 / PRICE_LIMIT as f64;
            let normalized_calories = self.total_calories as f64 / TARGET_CALORIES as f64;
            let normalized_carbs = self.total_carbs as f64 / TARGET_CARBS as f64;
            let normalized_fats = self.total_fats as f64 / TARGET_FATS as f64;
            let normalized_proteins = self.total_proteins as f64 / TARGET_PROTEINS as f64;
            self.path_cost = (normalized_calories + normalized_carbs + normalized_fats + normalized_proteins) - normalized_price;
            if self.total_calories < 2050 || self.total_calories > 2450 {
                self.path_cost -= (self.total_calories as i32 - 2250).abs() as f64;
            }
            if self.total_carbs < 250 || self.total_carbs > 300 {
                self.path_cost -= (self.total_carbs as i32 - 275).abs() as f64;
            }
            if self.total_fats < 40 || self.total_fats > 60 {
                self.path_cost -= (self.total_fats as i32 - 50).abs() as f64;
            }
            if self.total_proteins < 100 || self.total_proteins > 140 {
                self.path_cost -= (self.total_proteins as i32 - 120).abs() as f64;
            }
            self.current_dish = next_dish;
        }
    }

    fn select_next_dish(&self, dishes: &[Dish], pheromone: &[f64]) -> usize {
        let mut rng = rand::thread_rng();
        let total: f64 = dishes.iter().enumerate()
            .filter(|&(i, _)| self.path.iter().find(|&x| x.dish == i).map_or(true, |x| x.count < 10))
            .map(|(i, _)| pheromone[i])
            .sum();
        let mut prob: f64 = rng.gen();
        for (i, dish) in dishes.iter().enumerate() {
            if self.path.iter().find(|&x| x.dish == i).map_or(true, |x| x.count < 10) {
                prob -= pheromone[i] / total;
                if prob <= 0.0 {
                    return i;
                }
            }
        }
        self.path[self.path.len() - 1].dish
    }
}

#[derive(Debug, Clone)]
pub struct AntColony {
    pub(crate) dishes: Vec<Dish>,
    pheromone: Vec<f64>,
    ants: Vec<Ant>,
    pub(crate) best_path: Vec<DishCount>,
    best_ant: Option<Ant>
}

impl AntColony {
    pub(crate) fn new(dishes: Vec<Dish>, num_ants: usize) -> Self {
        let pheromone = vec![1.0; dishes.len()];
        let ants = vec![Ant::new(); num_ants];
        Self {
            dishes,
            pheromone,
            ants,
            best_path: vec![],
            best_ant: None,
        }
    }

    fn reset_ants(&mut self) {
        for ant in &mut self.ants {
            *ant = Ant::new();
        }
    }

    fn update_pheromones(&mut self) {
        const EVAPORATION_RATE: f64 = 0.9; // 10% of the pheromone evaporates in each iteration
        // Pheromone evaporation
        for pheromone_level in &mut self.pheromone {
            *pheromone_level *= EVAPORATION_RATE;
        }

        // Find the best ant
        let best_ant = self.ants.iter().max_by(|a, b| a.path_cost.partial_cmp(&b.path_cost).unwrap()).unwrap();

        // Pheromone deposit
        for ant in &self.ants {
            let fitness_ratio = ant.path_cost / best_ant.path_cost;
            let random_number: f64 = rand::random();
            if random_number <= fitness_ratio {
                for &dish in &ant.path {
                    self.pheromone[dish.dish] += 1.0 / ant.path_cost;
                }
            }
        }
    }

    pub(crate) fn run(&mut self, iterations: usize) -> Vec<f64> {
        let mut best_fitnesses = Vec::new();
        for _ in 0..iterations {
            self.reset_ants();
            self.ants.par_iter_mut().for_each(|ant| {
                ant.construct_path(&self.dishes, &self.pheromone);
            });
            self.update_pheromones();
            if let Some(best_ant) = self.ants.iter().max_by(|a, b| a.path_cost.partial_cmp(&b.path_cost).unwrap()) {
                if self.best_ant.is_none() || best_ant.path_cost > self.best_ant.as_ref().unwrap().path_cost {
                    self.best_path = best_ant.path.clone();
                    self.best_ant = Some(best_ant.clone());
                }
            }
            best_fitnesses.push(self.best_ant.as_ref().unwrap().path_cost);
        }
        best_fitnesses
    }
}

pub fn ant_colony_algorithm() {
    sample_run();

    let dishes = get_dishes();
    let results = Mutex::new(Vec::new());
    let num_ants_values: Vec<usize> = (0..=10).map(|i| 5 * 2usize.pow(i)).collect();
    num_ants_values.par_iter().for_each(|&num_ants| {
        let num_iterations_values: Vec<usize> = (0..=10).map(|i| 5 * 2usize.pow(i)).collect();
        num_iterations_values.par_iter().for_each(|&num_iterations| {
            let mut ant_colony = AntColony::new(dishes.clone(), num_ants);
            let best_fitnesses = ant_colony.run(num_iterations);
            let best_fitness = *best_fitnesses.last().unwrap();
            results.lock().unwrap().push((num_ants, num_iterations, best_fitness));
        });
    });

    for (num_ants, num_iterations, best_fitness) in results.lock().unwrap().iter() {
        println!("Number of ants: {}, Number of iterations: {}, Best fitness: {}", num_ants, num_iterations, best_fitness);
    }

    let results = results.lock().unwrap();

    plot_for_each_num_ants(&results);
    plot_for_each_num_iterations(&results);
    plot_joint(&results);
}

fn sample_run() {
    let dishes = get_dishes();
    let mut ant_colony = AntColony::new(dishes, 1000);
    ant_colony.run(5000);
    let best_path_names: Vec<String> = ant_colony.best_path
        .iter()
        .map(|dish_count| format!("dish: {}, count: {}", ant_colony.dishes[dish_count.dish].name, dish_count.count))
        .collect();
    println!("Best path: {:?}", best_path_names);

    let total_calories: u32 = ant_colony.best_path
        .iter()
        .map(|dish_count| ant_colony.dishes[dish_count.dish].calories * dish_count.count as u32)
        .sum();
    let total_carbs: u32 = ant_colony.best_path
        .iter()
        .map(|dish_count| ant_colony.dishes[dish_count.dish].carbs * dish_count.count as u32)
        .sum();
    let total_fats: u32 = ant_colony.best_path
        .iter()
        .map(|dish_count| ant_colony.dishes[dish_count.dish].fats * dish_count.count as u32)
        .sum();
    let total_proteins: u32 = ant_colony.best_path
        .iter()
        .map(|dish_count| ant_colony.dishes[dish_count.dish].proteins * dish_count.count as u32)
        .sum();
    let total_price: u32 = ant_colony.best_path
        .iter()
        .map(|dish_count| ant_colony.dishes[dish_count.dish].price * dish_count.count as u32)
        .sum();
    let best_ant = ant_colony.best_ant.unwrap();

    println!("Goal function value: {}", best_ant.path_cost);
    println!("Total price: {}", total_price);
    println!("Total calories: {}", total_calories);
    println!("Total carbs: {}", total_carbs);
    println!("Total fats: {}", total_fats);
    println!("Total proteins: {}", total_proteins);
}

pub fn plot_best_fitness(best_fitnesses: &[f64]) {
    let root = BitMapBackend::new("best_fitness.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Best Fitness Over Generations", ("sans-serif", 20).into_font())
        .margin(25)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..best_fitnesses.len() as f64, best_fitnesses.iter().cloned().fold(f64::MAX, f64::min)..best_fitnesses.iter().cloned().fold(f64::MIN, f64::max))
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart.draw_series(LineSeries::new(
        best_fitnesses.iter().enumerate().map(|(i, fitness)| (i as f64, *fitness)),
        &RED,
    )).unwrap();
}

fn plot_for_each_num_ants(results: &[(usize, usize, f64)]) {
    let num_ants_values: Vec<usize> = (0..=10).map(|i| 5 * 2usize.pow(i)).collect();
    for &num_ants in &num_ants_values {
        let filename = format!("plot_for_{}_ants.png", num_ants);
        let root = BitMapBackend::new(&filename, (640, 480)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let data: Vec<(f64, f64)> = results.iter()
            .filter(|&&(na, _, _)| na == num_ants)
            .map(|&(_, ni, bf)| (ni as f64, bf))
            .collect();

        let fitness_values: Vec<f64> = data.iter().map(|&(_, y)| y).collect();
        let min_fitness = *fitness_values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let max_fitness = *fitness_values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption(format!("Best Fitness Over Generations for {} Ants", num_ants), ("sans-serif", 20).into_font())
            .margin(25)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0f64..(results.len() as f64) * 1.1, min_fitness*0.9..max_fitness*1.1)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart.draw_series(LineSeries::new(data, &RED)).unwrap();
    }
}

fn plot_for_each_num_iterations(results: &[(usize, usize, f64)]) {
    let num_iterations_values: Vec<usize> = (0..=10).map(|i| 5 * 2usize.pow(i)).collect();
    for &num_iterations in &num_iterations_values {
        let filename = format!("plot_for_{}_iterations.png", num_iterations);
        let root = BitMapBackend::new(&filename, (640, 480)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption(format!("Best Fitness Over Generations for {} Iterations", num_iterations), ("sans-serif", 20).into_font())
            .margin(25)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0f64..results.len() as f64, 0.0..1.0)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        let data: Vec<(f64, f64)> = results.iter()
            .filter(|&&(_, ni, _)| ni == num_iterations)
            .map(|&(_, na, bf)| (na as f64, bf))
            .collect();

        chart.draw_series(LineSeries::new(data, &RED)).unwrap();
    }
}

fn plot_joint(results: &[(usize, usize, f64)]) {
    let filename = "joint_plot.png";
    let root = BitMapBackend::new(&filename, (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Best Fitness Over Generations", ("sans-serif", 20).into_font())
        .margin(25)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..results.len() as f64, 0.0..1.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    let data: Vec<(f64, f64)> = results.iter()
        .filter(|&&(na, ni, _)| na == ni)
        .map(|&(_, _, bf)| bf)
        .enumerate()
        .map(|(i, bf)| (i as f64, bf))
        .collect();

    chart.draw_series(LineSeries::new(data, &RED)).unwrap();
}