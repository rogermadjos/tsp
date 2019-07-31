#[path = "tsp.rs"]
mod tsp;

use crate::tsp::City;
use crate::tsp::total_distance;
use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::cmp::Ordering;

struct Individual {
  solution: Vec<usize>,
  fitness: f64,
}

impl Ord for Individual {
    fn cmp(&self, other: &Self) -> Ordering {
        other.fitness.partial_cmp(&self.fitness).unwrap()
    }
}

impl PartialOrd for Individual {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.fitness.partial_cmp(&self.fitness)
    }
}

impl PartialEq for Individual {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

impl Eq for Individual {}

fn generate_random_individual(cities: &Vec<City>, rng: &mut ThreadRng) -> Individual {
  let mut solution: Vec<usize> = (0..cities.len()).collect();
  solution.shuffle(rng);
  let fitness = 1. / total_distance(cities, &solution);

  Individual {
    solution,
    fitness,
  }
}

fn crossover(parent1: &Individual, parent2: &Individual, rng: &mut ThreadRng) {
  let _mark: usize = rng.gen();
}

pub fn solve(cities: &Vec<City>) -> Vec<usize> {
  let mut rng = thread_rng();
  let population_size = 10;
  let mut population = Vec::new();
  for _ in 0..population_size {
    population.push(generate_random_individual(cities, &mut rng));
  }

  population.sort();

  let best = population.first().unwrap();
  best.fitness;

  // Selection
  let survivors: Vec<&Individual> = population.iter().take(5).collect();
  let _parents = survivors.choose_multiple(&mut rng, 2);

  vec![]
}
