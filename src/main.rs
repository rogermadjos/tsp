mod tsp;

use crate::tsp::cities;
use crate::tsp::total_distance;
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
    let cities = cities(3, 100.);
    println!("{:#?}", total_distance(&cities, &[0, 1, 2].to_vec()));
    println!("{:#?}", total_distance(&cities, &[1, 2, 0].to_vec()));
    // let args = Opt::from_args();
    // println!("{:#?}", args);
}
