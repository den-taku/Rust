// use std::thread;

struct City {
    name: String,
    population: i64,
    country: String,
    monster_attack_risk: f32
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

// /// Sort by any of several different statistics.
// fn sort_by_statistics(cities: &mut Vec<City>, stat: Statistic) {
//     cities.sort_by_key(|city| -city.get_statistic(stat));
// }

// fn start_sorting_thread(mut cities: Vec<City>, stat: Statistic)
//     -> thread::JoinHandle<Vec<City>>
// {
//     // let key_fn = |city: &City| -> i64 { -city.get_statistics(stat) };
//     let key_fn = move |city: &City| -> i64 { -city.get_statistic(stat) };
// 
//     thread::spawn(move || {
//         cities.sort_by_key(key_fn);
//         cities
//     })
// }

/// Given a list of cities and a test function,
/// return how many cities pass the test.
fn count_selected_cities<F>(cities: &Vec<City>, test_fn: F) -> usize 
where F: Fn(&City) -> bool
{
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }
    count
}

/// An example of a test function. Note that the type of 
/// this function is `fn(&City) -> bool`, the same as
/// the `test_fn` argument to `count_selected_cities`.
fn has_monster_attacks(city: &City) -> bool {
    city.monster_attack_risk > 0.0
}

fn call_twice<F>(clousure: F) where F: Fn() {
    clousure();
    clousure();
}

fn main() {
    // let my_key_fn: fn(&City) -> i64 =
    //     if user.prefs.by_population {
    //         city_population_descending
    //     } else {
    //         city_monster_attack_risk_descending
    //     };
    //
    // cities.sort_by_key(my_key_fn);

    // How many cities are at risk for monster attack?
    let my_cities = vec![];
    let n = count_selected_cities(&my_cities, has_monster_attacks);
    println!("{} cities has at risk for monster attack.", n);

    let limit = 0.0;
    let n = count_selected_cities(
        &my_cities,
        |city| city.monster_attack_risk > limit);
    println!("{} cities has at risk for monster attack.", n);

    let my_str = "hello".to_string();
    let f = || drop(my_str);
    // f();
    // f();
    // call_twice(f);
}






























