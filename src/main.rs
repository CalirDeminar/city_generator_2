use city::city::{random_city, Era};
use procgen_templater::dictionary::dictionary::build_dictionary_from_folder;
pub mod city;
pub mod grammar;
fn main() {
    let dict = build_dictionary_from_folder("./data_files");

    let mut city = random_city(&dict, Era::Medieval, 50);
    for i in 0..100 {
        println!("Year {} ----------", i);
        city.inspect_population();
        city.simulate_year();
    }
    for mind in city.population.values() {
        // mind.inspect(&city);
    }
}
