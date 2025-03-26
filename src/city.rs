pub mod area;
pub mod culture;
pub mod dieties;
pub mod institutions;
pub mod population;
pub mod city {
    use std::io::prelude::*;
    use std::{
        collections::{HashMap, HashSet},
        fmt,
        fs::File,
        io::LineWriter,
        path::Path,
    };

    use procgen_templater::dictionary::dictionary::Dictionary;
    use uuid::Uuid;

    use crate::city::population::mind::{mind::Mind, relations::relations::RelationVerb};

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
            let ref_citizens = self.population.clone();
            for citizen in ref_citizens.values() {
                if citizen.alive {
                    let citizen_mut = self.population.get_mut(&citizen.id).unwrap();
                    citizen_mut.age();
                    if !citizen.alive {
                        let spare: HashSet<Uuid> = HashSet::new();

                        let partners = citizen
                            .relations
                            .get(&RelationVerb::Partner)
                            .unwrap_or(&spare);
                        for partner in partners {
                            let m = self.population.get_mut(partner).unwrap();
                            m.relations
                                .get_mut(&RelationVerb::Partner)
                                .unwrap()
                                .remove(partner);
                            if !m.relations.contains_key(&RelationVerb::LatePartner) {
                                m.relations
                                    .insert(RelationVerb::LatePartner, HashSet::new());
                            }
                            m.relations
                                .get_mut(&RelationVerb::LatePartner)
                                .unwrap()
                                .insert(citizen.id.clone());
                        }
                        let spouses = citizen
                            .relations
                            .get(&RelationVerb::Spouse)
                            .unwrap_or(&spare);
                        for spouse in spouses {
                            let m = self.population.get_mut(spouse).unwrap();
                            m.relations
                                .get_mut(&RelationVerb::Spouse)
                                .unwrap()
                                .remove(spouse);
                            if !m.relations.contains_key(&RelationVerb::LateSpouse) {
                                m.relations.insert(RelationVerb::LateSpouse, HashSet::new());
                            }
                            m.relations
                                .get_mut(&RelationVerb::LateSpouse)
                                .unwrap()
                                .insert(citizen.id.clone());
                        }
                    }
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
        }
        pub fn population_graph(self: &Self) {
            let bucket_size: usize = 10;
            let age_buckets: Vec<u32> = (0..=100).step_by(bucket_size).collect();
            let population = self.current_citizens();
            let graph: Vec<(u32, u32)> = age_buckets
                .iter()
                .map(|target_age| {
                    (
                        target_age.clone(),
                        population
                            .iter()
                            .filter(|id| {
                                let m = self.population.get(id).unwrap();
                                return m.age >= *target_age
                                    && m.age < target_age + bucket_size as u32;
                            })
                            .count() as u32,
                    )
                })
                .collect();
            let axis_line = graph.iter().fold(String::new(), |acc, (i, _)| {
                format!("{}{}{}", acc, "    ", i)
            });
            let data_line = graph.iter().fold(String::new(), |acc, (_, i)| {
                format!(
                    "{}{}{}",
                    acc,
                    if i > &99 {
                        "   "
                    } else if i > &9 {
                        "    "
                    } else {
                        "     "
                    },
                    i
                )
            });
            println!("{}", data_line);
            println!(" {}", axis_line);
        }

        pub fn export(self: &Self) {
            let mind_file = File::create(Path::new("./export/minds_export.md")).unwrap();
            let institution_file =
                File::create(Path::new("./export/institutions_export.md")).unwrap();
            let mut mind_file_writer = LineWriter::new(mind_file);
            let mut institution_file_writer = LineWriter::new(institution_file);

            for id in self.current_citizens() {
                mind_file_writer
                    .write_all(self.population.get(&id).unwrap().print(&self).as_bytes())
                    .unwrap();
            }
            for id in self.institutions.keys() {
                institution_file_writer
                    .write_all(self.institutions.get(&id).unwrap().print(&self).as_bytes())
                    .unwrap();
            }
            mind_file_writer.flush().unwrap();
            institution_file_writer.flush().unwrap();
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
                let institutions_clone = self.institutions.clone();
                let institutions = institutions_clone.values();
                for inst in institutions {
                    if inst.staff.len().eq(&0) {
                        self.institutions.remove(&inst.id);
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
