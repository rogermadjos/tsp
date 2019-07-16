use rand::Rng;

#[derive(Debug)]
pub struct City {
    pub x: f64,
    pub y: f64,
}

impl City {
    pub fn distance(&self, city: &City) -> f64 { ((self.x - city.x).powf(2.) + (self.y - city.y).powf(2.)).sqrt() }
}

pub fn cities(num_cities: u32, world_size: f64) -> Vec<City> {
    let mut rng = rand::thread_rng();

    let mut cities = vec![];

    for _ in 0..num_cities {
        let city = City {
            x: rng.gen_range(0., world_size),
            y: rng.gen_range(0., world_size),
        };

        cities.push(city);
    }

    cities
}

pub fn total_distance(cities: &Vec<City>, sequence: &Vec<u32>) -> f64 {
    let mut distance = 0.;

    if cities.len() != sequence.len() {
        panic!("cities and sequence should have the same length");
    }

    for i in 0..cities.len() {
        let one = &cities[i];
        let two = &cities[(i + 1) % cities.len()];

        distance += one.distance(two);
    }

    distance
}
