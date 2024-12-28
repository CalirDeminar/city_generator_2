pub mod institutions {
    // Split via GUARDS
    // G - Goverment Buildings
    // U - Underworld Activities
    // A - Altars (Churches, Cults, etc)
    // R - Resources (Sale, Extraction or Production)
    // D - Defenses (Police, Guards, Sherrif, etc)
    // S - Social Hubs

    use std::collections::HashMap;

    use procgen_templater::dictionary::dictionary::Dictionary;
    use rand::{seq::SliceRandom, Rng};
    use uuid::Uuid;

    use crate::city::{city::City, dieties::dieties::Diety, population::mind::mind::MindId};

    #[derive(PartialEq, Debug, Clone)]
    pub enum InsitutionCategory {
        Goverment,
        Underworld,
        Altar,
        Resource,
        Defence,
        Social,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum InsitutionType {
        // Goverment
        Hall,
        Hospital,
        Dock,
        School,
        Library,
        University,
        Prison,
        // Underworld
        ShadyBar,
        Fence,
        // Altar
        Altar,
        Chapel,
        // Resource
        GeneralRetail,
        SpecialistRetail,
        Office,
        Bank,
        // Defence
        PoliceStation,
        // Social
        Pub,
        Restaurant,
        Hotel,
        Theater,
        Casino,
        Club,
        Gym,
        TattooParlor,
    }

    pub type InstitutionId = Uuid;

    #[derive(PartialEq, Debug, Clone)]
    pub struct Institution {
        pub id: InstitutionId,
        pub name: String,
        pub category: InsitutionCategory,
        pub management: Vec<ManagementSpecification>,
        pub base_job_titles: Vec<String>,
        pub staff: HashMap<Uuid, StaffDefinition>,
        pub related_diety: Option<Uuid>,
    }
    #[derive(PartialEq, Debug, Clone)]
    pub struct ManagementSpecification {
        pub title: String,
        pub reportee_types: Vec<String>,
        pub min_reportees: u32,
        pub max_reportees: u32,
    }
    #[derive(PartialEq, Debug, Clone)]
    pub struct StaffDefinition {
        pub title: String,
        pub employee_id: MindId,
        pub salary: u32,
        pub started_year: u32,
    }

    pub fn random_institution(dict: &Dictionary, city: &City) -> Institution {
        let mut rng = rand::thread_rng();
        let roll = rng.gen::<f32>();
        let underworld_chance = 0.2;
        let temple_chance = 0.2;
        let existing_dieties: Vec<&Diety> = city.culture.dieties.values().into_iter().collect();
        if roll < underworld_chance {
            return generate_underground(dict);
        } else if roll < (underworld_chance + temple_chance) {
            return generate_temple(dict, &existing_dieties);
        } else {
            return generate_social(dict);
        }
    }

    pub fn generate_temple(dict: &Dictionary, existing_dieties: &Vec<&Diety>) -> Institution {
        let template = dict
            .get_random_template(vec![vec!["Altar".to_string()]])
            .unwrap();
        let mut dieties = existing_dieties.clone();
        dieties.shuffle(&mut rand::thread_rng());
        let diety = dieties.first().unwrap();
        let output = Institution {
            id: Uuid::new_v4(),
            name: dict
                .render_template(&template.id)
                .unwrap()
                .replace("Concept", &diety.realms.first().unwrap().base)
                .replace("Diety", &diety.name),
            category: InsitutionCategory::Social,
            management: Vec::new(),
            base_job_titles: vec!["Acolyte".to_string()],
            staff: HashMap::new(),
            related_diety: Some(diety.id.clone()),
        };

        return output;
    }

    pub fn generate_underground(dict: &Dictionary) -> Institution {
        let template = dict
            .get_random_template(vec![vec!["SeedyBar".to_string(), "Fence".to_string()]])
            .unwrap();
        let is_fence = template.tags.contains("Fence");
        let institution = Institution {
            id: Uuid::new_v4(),
            name: dict.render_template(&template.id).unwrap(),
            category: InsitutionCategory::Underworld,
            management: Vec::new(),
            base_job_titles: if is_fence {
                vec!["Shopkeeper".to_string()]
            } else {
                vec!["Waiter".to_string()]
            },
            staff: HashMap::new(),
            related_diety: None,
        };
        return institution;
    }

    pub fn generate_social(dict: &Dictionary) -> Institution {
        let template = dict
            .get_random_template(vec![vec!["Social".to_string()]])
            .unwrap();
        let mut output = Institution {
            id: Uuid::new_v4(),
            name: dict.render_template(&template.id).unwrap(),
            category: InsitutionCategory::Social,
            management: Vec::new(),
            base_job_titles: Vec::new(),
            staff: HashMap::new(),
            related_diety: None,
        };
        if template.tags.contains("Food") {
            output.base_job_titles.push("Cook".to_string());
            output.management.push(ManagementSpecification {
                title: "Chef".to_string(),
                reportee_types: vec!["Cook".to_string()],
                min_reportees: 2,
                max_reportees: 6,
            });
            output.base_job_titles.push("Waiter".to_string());
            output.management.push(ManagementSpecification {
                title: "Manager".to_string(),
                reportee_types: vec!["Waiter".to_string()],
                min_reportees: 1,
                max_reportees: 4,
            })
        }
        if template.tags.contains("Theater") {
            output.base_job_titles.push("Actor".to_string());
            output.management.push(ManagementSpecification {
                title: "Stage Hand".to_string(),
                reportee_types: vec!["Actor".to_string()],
                min_reportees: 2,
                max_reportees: 6,
            });
        }
        return output;
    }

    impl City {
        pub fn fire_percentage(self: &mut Self, percentage: f32) {
            let population_clone = self.population.clone();
            let to_fire_count = (population_clone.len() as f32 * percentage) as usize;
            let mut ids: Vec<&Uuid> = population_clone.keys().into_iter().collect();
            ids.shuffle(&mut rand::thread_rng());
            for id in ids.iter().take(to_fire_count) {
                let mind = self.population.get_mut(id).unwrap();
                if mind.employer.is_some() {
                    let employer_id = mind.employer.unwrap();
                    let institution = self.institutions.get_mut(&employer_id).unwrap();
                    institution.staff.remove(&mind.id);
                    // get institution
                    // remove from institution employees
                    mind.employer = None;
                }
            }
        }
        pub fn fill_and_create_jobs(self: &mut Self) {}
    }

    #[test]
    fn test_name_gen() {
        use crate::city::city::{random_city, Era};
        use procgen_templater::dictionary::dictionary::build_dictionary_from_folder;
        let dict = build_dictionary_from_folder("./data_files");
        let city = &random_city(&dict, Era::Medieval, 5);
        for _i in 0..50 {
            let inst = random_institution(&dict, &city);
            println!("{}", inst.name);
        }
    }
}
