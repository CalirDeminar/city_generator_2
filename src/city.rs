pub mod area;
pub mod culture;
pub mod dieties;
pub mod institutions;
pub mod population;
pub mod city {
    use std::{collections::HashMap, fmt};

    use procgen_templater::dictionary::dictionary::Dictionary;
    use uuid::Uuid;

    use crate::city::population::mind::mind::Mind;

    use super::{
        area::area::{Area, AreaId},
        culture::culture::{random_culture, Culture},
        population::{mind::mind::random_mind, population::Population},
    };

    #[derive(PartialEq, Debug, Clone)]
    pub enum Era {
        Modern,
        Medieval,
        Fantasy,
    }

    impl fmt::Display for Era {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Era::Modern => write!(f, "EraModern"),
                Era::Medieval => write!(f, "EraMedieval"),
                Era::Fantasy => write!(f, "EraFantasy"),
            }
        }
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct City {
        pub id: Uuid,
        pub name: String,
        pub culture: Culture,
        pub population: Population,
        pub areas: HashMap<AreaId, Area>,
        pub year: usize,
    }

    impl City {
        pub fn simulate_year(self: &mut Self) {
            self.year += 1;
            self.increment_citizen_ages();
            self.temp_add_friends();
            self.update_mind_partner_relations();
        }
        fn increment_citizen_ages(self: &mut Self) {
            for citizen in self.population.values_mut() {
                citizen.age();
            }
        }
        pub fn current_citizens(self: &Self) -> Vec<Uuid> {
            return self
                .population
                .values()
                .filter(|c| c.alive)
                .map(|m| m.id)
                .collect();
        }
        pub fn current_single_citizens(self: &Self) -> Vec<Uuid> {
            return self
                .population
                .values()
                .filter(|c| c.alive && c.is_single())
                .map(|m| m.id)
                .collect();
        }
        pub fn inspect_population(self: &Self) {
            let living_citizens: Vec<&Mind> =
                self.population.values().filter(|m| m.alive).collect();
            println!("Citizens: {}", living_citizens.len());
            let single_citizens: Vec<&&Mind> =
                living_citizens.iter().filter(|m| m.is_single()).collect();
            println!(
                "Single Citizens: {}/{} - {:.2}%",
                single_citizens.len(),
                living_citizens.len(),
                1.0 - (single_citizens.len() as f32 / living_citizens.len() as f32)
            );
        }
    }

    pub fn random_city(dict: &Dictionary, era: Era, base_population: usize) -> City {
        let culture = random_culture(dict, era);
        let mut population: Population = HashMap::new();
        for _i in 0..base_population {
            let m = random_mind(&dict, &culture);
            population.insert(m.id.clone(), m);
        }
        return City {
            id: Uuid::new_v4(),
            name: String::new(),
            culture,
            population,
            areas: HashMap::new(),
            year: 0,
        };
    }
}
