mod tsp;
mod genetic;

use crate::tsp::cities;
use crate::genetic::solve;
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
    let cities = cities(5, 100.);
    solve(&cities);
    // let args = Opt::from_args();
    // println!("{:#?}", args);
}
