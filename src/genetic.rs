#[path = "tsp.rs"]
mod tsp;

use crate::tsp::City;
use crate::tsp::total_distance;
use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::cmp::Ordering;

#[derive(Clone)]
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

fn crossover(parent1: &Individual, parent2: &Individual, mark: usize) {
}

fn invert(perm: &Vec<usize>) -> Vec<usize> {
  let mut inverse: Vec<usize> = vec![];
  for i in 0..perm.len() {
    let mut inv: usize = 0;
    for m in 0..perm.len() {
      if perm[m] == i {
        break;
      }
      if perm[m] > i {
        inv += 1;
      }
    }

    inverse.push(inv);
  }

  inverse
}

fn original(inv: &Vec<usize>) -> Vec<usize> {
  let mut perm = vec![0; inv.len()];
  let mut pos = vec![0; inv.len()];
  for i in (0..inv.len()).rev() {
    for m in (i+1)..inv.len() {
      if pos[m] >= inv[i] + 1 {
        pos[m] += 1;
      }
    }

    pos[i] = inv[i] + 1;
  }

  for i in 0..inv.len() {
    let j = pos[i] - 1;
    perm[j] = i;
  }

  perm
}

pub fn solve(cities: &Vec<City>) -> Vec<usize> {
  let mut rng = thread_rng();
  let population_size = 10;
  let survivors_size = population_size / 2;
  let mut population = Vec::new();
  for _ in 0..population_size {
    population.push(generate_random_individual(cities, &mut rng));
  }

  population.sort();

  // Selection
  let survivors: Vec<Individual> = population.iter().cloned().take(survivors_size).collect();
  let parents: Vec<&Individual> = survivors.choose_multiple(&mut rng, 2).collect();
  let mark: usize = rng.gen::<usize>() % cities.len();

  vec![]
}

#[cfg(test)]
mod tests {

  use super::*;

  const EXAMPLES: [([usize; 7], [usize; 7]); 2] = [
    ( [5, 1, 2, 3, 0, 6, 4], [4, 1, 1, 1, 2, 0, 0] ),
    ( [4, 1, 3, 0, 2, 6, 5], [3, 1, 2, 1, 0, 1, 0] )
  ];

  #[test]
  fn test_invert() {
    for (input, output) in EXAMPLES.iter() {
      assert_eq!(&invert(&input.to_vec()), output);
    }
  }

  #[test]
  fn test_original() {
    for (output, input) in EXAMPLES.iter() {
      assert_eq!(&original(&input.to_vec()), output);
    }
  }
}
