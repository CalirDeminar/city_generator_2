pub mod area;
pub mod culture;
pub mod dieties;
pub mod institutions;
pub mod population;
pub mod city {
    use std::{collections::HashMap, fmt};

    use procgen_templater::dictionary::dictionary::Dictionary;
    use uuid::Uuid;

    use crate::city::{
        institutions,
        population::mind::{mind::Mind, relations::relations::RelationVerb},
    };

    use super::{
        area::area::{Area, AreaId},
        culture::culture::{random_culture, Culture},
        institutions::institutions::Institution,
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
        pub institutions: HashMap<Uuid, Institution>,
        pub year: usize,
    }

    impl City {
        pub fn simulate_year(self: &mut Self, dict: &Dictionary) {
            self.year += 1;
            self.increment_citizen_ages();
            // employment
            self.fire_percentage(0.05);
            self.fill_and_create_jobs(dict);
            //social
            self.temp_add_friends();
            self.update_mind_partner_relations();
            self.generate_children(dict);

            self.cleanup(5);
        }
        fn increment_citizen_ages(self: &mut Self) {
            for citizen in self.population.values_mut() {
                if citizen.alive {
                    citizen.age();
                }
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
                (single_citizens.len() as f32 / living_citizens.len() as f32) * 100.0
            );
            let friendless_citizens: Vec<&&Mind> = living_citizens
                .iter()
                .filter(|m| {
                    m.age > self.culture.adult_age
                        && ((!m.relations.contains_key(&RelationVerb::Friend)
                            || m.relations.get(&RelationVerb::Friend).unwrap().len() < 1)
                            && (!m.relations.contains_key(&RelationVerb::CloseFriend)
                                || m.relations.get(&RelationVerb::CloseFriend).unwrap().len() < 1))
                })
                .collect();
            println!("Friendless Citizens :{}", friendless_citizens.len());
        }
        pub fn cleanup(self: &mut Self, interval: usize) {
            let rem = self.year.checked_rem(interval);
            if rem.is_some() && rem.unwrap().eq(&0) {
                println!("Running Cleanup");
                for mind in self.population.values_mut() {
                    mind.cleanup();
                    if mind.employer.is_some() && !mind.alive {
                        let employer = self.institutions.get_mut(&mind.employer.unwrap()).unwrap();
                        if employer.staff.contains_key(&mind.id) {
                            employer.staff.remove(&mind.id);
                        }
                        mind.employer = None;
                    }
                }
            }
        }
    }

    pub fn random_city(dict: &Dictionary, era: Era, base_population: usize) -> City {
        let culture = random_culture(dict, era);
        let mut population: Population = HashMap::new();
        for _i in 0..base_population {
            let m = random_mind(&dict, &culture, 0);
            population.insert(m.id.clone(), m);
        }
        return City {
            id: Uuid::new_v4(),
            name: String::new(),
            culture,
            population,
            areas: HashMap::new(),
            institutions: HashMap::new(),
            year: 0,
        };
    }
}
