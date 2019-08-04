#[cfg(test)]
extern crate speculate;

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
      chromosome.push(rng.gen::<usize>() % (cities.len() - i));
    }

    Individual::new(&chromosome, cities)
  }

  fn crossover(&self, other: &Self, cities: &Vec<City>, rng: &mut ThreadRng) -> (Individual, Individual) {
    let len = self.chromosome.len();

    let points = (rng.gen::<usize>() % len, rng.gen::<usize>() % len);

    let pns = if points.0 <= points.1 {
      points.1 - points.0
    } else {
      len - points.0 + points.1
    };

    let mut offsprings = (self.chromosome.clone(), other.chromosome.clone());

    if pns > 0 {
      for i in 0..pns {
        let j = (points.0 + i) % len;
        offsprings.0[j] = other.chromosome[j];
        offsprings.1[j] = self.chromosome[j];
      }
    }

    (Individual::new(&offsprings.0, cities), Individual::new(&offsprings.1, cities))
  }

  fn mutate(&mut self, rng: &mut ThreadRng) {
    let len = self.chromosome.len();
    let pos = rng.gen::<usize>() % len;

    self.chromosome[pos] = rng.gen::<usize>() % (len - pos);
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

fn random_individual(cities: &Vec<City>, rng: &mut ThreadRng) -> Individual {
  let mut perm: Vec<usize> = (0..cities.len()).collect();
  perm.shuffle(rng);

  let fitness = 1. / total_distance(cities, &perm);

  Individual {
    chromosome: invert(&perm),
    fitness,
  }
}

fn crossover(parents: (&Vec<usize>, &Vec<usize>), points: (usize, usize)) -> (Vec<usize>, Vec<usize>) {
  let len = parents.0.len();

  let pns = if points.0 <= points.1 {
    points.1 - points.0
  } else {
    parents.0.len() - points.0 + points.1
  };

  let mut offsprings = (parents.0.clone(), parents.1.clone());

  if pns > 0 {
    for i in 0..pns {
      let j = (points.0 + i) % len;
      offsprings.0[j] = parents.1[j];
      offsprings.1[j] = parents.0[j];
    }
  }


  offsprings
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
  pub mutation_rate: f64,
}

pub fn solve(cities: &Vec<City>, options: GAOptions) -> Vec<usize> {
  let mut rng = thread_rng();
  let mut population = Vec::new();
  for _ in 0..options.pool_size {
    population.push(Individual::random(cities, &mut rng));
  }

  population.sort();

  let elites_count = ((options.pool_size as f64) * options.elitism) as usize;
  let mut population: Vec<Individual> = population.iter().cloned().take(elites_count).collect();
  for _ in 0..(options.pool_size - elites_count) {

  }

  // let survivors: Vec<Individual> = population.iter().cloned().take(survivors_size).collect();
  // let parents: Vec<&Individual> = survivors.choose_multiple(&mut rng, 2).collect();
  // let mark: usize = rng.gen::<usize>() % cities.len();

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
          mutation_rate: 0.007,
          pool_size: 100
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

    describe "crossover" {
      const PARENTS: ([usize; 7], [usize; 7]) = ([4, 1, 1, 1, 2, 0, 0], [3, 1, 2, 1, 0, 1, 0]);

      describe "given point 1 is greater than point 2" {
        const POINTS: (usize, usize) = (1,4);
        #[ignore]
        it "should generate correct output" {
          assert_eq!(crossover((&PARENTS.0.to_vec(), &PARENTS.1.to_vec()), POINTS), ([4, 1, 2, 1, 2, 0, 0].to_vec(), [3, 1, 1, 1, 0, 1, 0].to_vec()));
        }
      }

      describe "given point 2 is greater than point 1" {
        const POINTS: (usize, usize) = (5, 2);
        #[ignore]
        it "should generate correct output" {
          assert_eq!(crossover((&PARENTS.0.to_vec(), &PARENTS.1.to_vec()), POINTS), ([3, 1, 1, 1, 2, 1, 0].to_vec(), [4, 1, 2, 1, 0, 0, 0].to_vec()));
        }
      }

      describe "given point 1 is equal to point 2" {
        const POINTS: (usize, usize) = (4, 4);
        #[ignore]
        it "should generate offsprings equal to the parents" {
          assert_eq!(crossover((&PARENTS.0.to_vec(), &PARENTS.1.to_vec()), POINTS), ([4, 1, 1, 1, 2, 0, 0].to_vec(), [3, 1, 2, 1, 0, 1, 0].to_vec()));
        }
      }
    }
  }
}