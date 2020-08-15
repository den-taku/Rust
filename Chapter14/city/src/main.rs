use std::thread;

struct City {
    name: String,
    population: i64,
    country: String
}

/// Helper function for sorting cities by population.
fn city_population_descending(city: &City) -> i64 {
    -city.population
}

fn sort_cities_not_closure(cities: &mut Vec<City>) {
    cities.sort_by_key(city_population_descending);
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population);
}

/// Sort by any of several different statistics.
fn sort_by_statistics(cities: &mut Vec<City>, stat: Statistic) {
    cities.sort_by_key(|city| -city.get_statistic(stat));
}

fn start_sorting_thread(mut cities: Vec<City>, stat: Statistic)
    -> thread::JoinHandle<Vec<City>>
{
    // let key_fn = |city: &City| -> i64 { -city.get_statistics(stat) };
    let key_fn = move |city: &City| -> i64 { -city.get_statistic(stat) };

    thread::spawn(move || {
        cities.sort_by_key(key_fn);
        cities
    })
}

fn main() {
    println!("Hello, world!");
}
