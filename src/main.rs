mod tsp;
mod ga;

use crate::tsp::cities;
use crate::ga::solve;
use crate::ga::GAOptions;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    num_cities: u32,

    #[structopt(short, long, default_value = "100")]
    world_size: f64,

    #[structopt(short, long, default_value = "0")]
    minimum_distance: f64,
}

fn main() {
    let cities = cities(10, 100.);
    solve(&cities, GAOptions {
        elitism: 0.15,
        mutation_rate: 0.007,
        pool_size: 100
    });
    // let args = Opt::from_args();
    // println!("{:#?}", args);
}
