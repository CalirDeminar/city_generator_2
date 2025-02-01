use city::city::{random_city, Era};
use procgen_templater::dictionary::dictionary::build_dictionary_from_folder;
pub mod city;
pub mod grammar;
fn main() {
    let dict = build_dictionary_from_folder("./data_files");

    let mut city = random_city(&dict, Era::Medieval, 150);
    for i in 0..200 {
        println!("Year {} ----------", i);
        city.inspect_population();
        // city.population_graph();
        city.simulate_year(&dict);
    }
    city.cleanup(1);
    for id in city.current_citizens() {
        city.population.get(&id).unwrap().inspect(&city);
    }
    for inst in city.institutions.values() {
        inst.inspect(&city);
    }
}
