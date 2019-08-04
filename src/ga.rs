#[cfg(test)]
extern crate speculate;

#[path = "tsp.rs"]
mod tsp;

use crate::tsp::City;
use crate::tsp::total_distance;
use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;
use std::cmp::Ordering;

#[derive(Clone)]
struct Individual {
  chromosome: Vec<usize>,
  fitness: f64,
}

impl Individual {
  fn new(chromosome: &Vec<usize>, cities: &Vec<City>) -> Individual {
    Individual {
      chromosome: chromosome.clone(),
      fitness: 1. / total_distance(cities, &revert(chromosome))
    }
  }

  fn random(cities: &Vec<City>, rng: &mut ThreadRng) -> Individual {
    let mut chromosome: Vec<usize> = Vec::new();
    for i in 0..cities.len() {
      chromosome.push(rng.gen_range(0, cities.len() - i));
    }

    Individual::new(&chromosome, cities)
  }

  fn crossover(&self, other: &Self, cities: &Vec<City>, mutation: f64, rng: &mut ThreadRng) -> (Individual, Individual) {
    let len = self.chromosome.len();

    let points = (rng.gen_range(0, len), rng.gen_range(0, len));

    let pns = if points.0 <= points.1 {
      points.1 - points.0
    } else {
      len - points.0 + points.1
    };

    let mut offsprings = (self.chromosome.clone(), other.chromosome.clone());

    if pns > 0 {
      for i in 0..pns {
        let j = (points.0 + i) % len;

        if rng.gen_range(0., 1.) < mutation {
          offsprings.0[j] = rng.gen_range(0, cities.len() - j);
        } else {
          offsprings.0[j] = other.chromosome[j];
        }

        if rng.gen_range(0., 1.) < mutation {
          offsprings.1[j] = rng.gen_range(0, cities.len() - j);
        } else {
          offsprings.1[j] = self.chromosome[j];
        }
      }
    }

    (Individual::new(&offsprings.0, cities), Individual::new(&offsprings.1, cities))
  }
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

fn revert(inv: &Vec<usize>) -> Vec<usize> {
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

pub struct GAOptions {
  pub pool_size: usize,
  pub elitism: f64,
  pub mutation: f64,
  pub generations: usize,
}

pub fn solve(cities: &Vec<City>, options: GAOptions) -> Vec<usize> {
  let mut rng = thread_rng();
  let mut population = Vec::new();
  let elites_count = ((options.pool_size as f64) * options.elitism) as usize;

  for _ in 0..options.pool_size {
    population.push(Individual::random(cities, &mut rng));
  }

  for _ in 0..options.generations {
    // Selection
    population.sort();
    let best = &population[0];
    println!("{}", best.fitness);
    population.truncate(elites_count);

    // Crossover and Mutation
    while population.len() < options.pool_size {
      let parent_1_pos = rng.gen_range(0, elites_count);
      let mut parent_2_pos = rng.gen_range(0, elites_count);
      while parent_2_pos == parent_1_pos {
        parent_2_pos = rng.gen_range(0, elites_count);
      }

      let parent_1 = &population[parent_1_pos];
      let parent_2 = &population[parent_2_pos];

      let offsprings = parent_1.crossover(parent_2, cities, options.mutation, &mut rng);
      population.push(offsprings.0);
      population.push(offsprings.1);
    }
  }

  vec![]
}

#[cfg(test)]
mod tests {
  use super::*;
  use speculate::speculate;
  use crate::tsp::cities;

  speculate! {
    describe "solve" {
      it "should run without errors" {
        solve(&cities(10, 100.), GAOptions {
          elitism: 0.15,
          mutation: 0.005,
          pool_size: 100,
          generations: 100,
        });
      }
    }

    describe "invert" {
      const EXAMPLES: [([usize; 7], [usize; 7]); 2] = [
        ( [5, 1, 2, 3, 0, 6, 4], [4, 1, 1, 1, 2, 0, 0] ),
        ( [4, 1, 3, 0, 2, 6, 5], [3, 1, 2, 1, 0, 1, 0] )
      ];
      #[ignore]
      it "should generate correct output" {
        for (input, output) in EXAMPLES.iter() {
          assert_eq!(&invert(&input.to_vec()), output);
        }
      }
    }

    describe "revert" {
      const EXAMPLES: [([usize; 7], [usize; 7]); 2] = [
        ( [4, 1, 1, 1, 2, 0, 0], [5, 1, 2, 3, 0, 6, 4] ),
        ( [3, 1, 2, 1, 0, 1, 0], [4, 1, 3, 0, 2, 6, 5] )
      ];
      #[ignore]
      it "should generate correct output" {
        for (input, output) in EXAMPLES.iter() {
          assert_eq!(&revert(&input.to_vec()), output);
        }
      }
    }
  }
}
