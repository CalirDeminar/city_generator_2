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

    use crate::city::{
        city::City,
        culture::culture::Culture,
        dieties::dieties::Diety,
        population::mind::mind::{Mind, MindId},
    };

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
        pub wealth: usize,
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
        pub started_year: usize,
    }

    impl Institution {
        pub fn next_role(self: &Self) -> Option<String> {
            let manager_titles: Vec<&String> = self.management.iter().map(|mp| &mp.title).collect();
            let current_manager_count = self
                .staff
                .values()
                .filter(|s| manager_titles.contains(&&s.title))
                .count();
            let basic_staff_count = self
                .staff
                .values()
                .filter(|s| !manager_titles.contains(&&s.title))
                .count();
            let can_afford_more_managers = current_manager_count < self.wealth + 1;
            let basic_staff_max = self
                .staff
                .values()
                .filter(|s| manager_titles.contains(&&s.title))
                .fold(2, |count: usize, manager| {
                    let position = self
                        .management
                        .iter()
                        .find(|mp| mp.title.eq(&manager.title))
                        .unwrap();
                    return count + position.max_reportees as usize;
                });
            if self.base_job_titles.len() > 0 {
                // Check needed managers
                if can_afford_more_managers {
                    let target_manager = self.management.iter().find(|ms| {
                        let related_staff_count = self
                            .staff
                            .values()
                            .filter(|st| ms.reportee_types.contains(&st.title))
                            .count();
                        let current_position_count = self
                            .staff
                            .values()
                            .filter(|st| ms.title.eq(&st.title))
                            .count();
                        return related_staff_count as f32 / (current_position_count as f32)
                            > ms.max_reportees as f32;
                    });

                    if target_manager.is_some() {
                        return Some(target_manager.unwrap().title.clone());
                    }
                }
                if basic_staff_count < basic_staff_max || self.staff.len().eq(&0) {
                    // Base Employee
                    let mut titles = self.base_job_titles.clone();
                    titles.shuffle(&mut rand::thread_rng());
                    return Some(titles.first().unwrap().clone());
                }
            }
            return None;
        }

        pub fn inspect(self: &Self, city: &City) {
            println!(
                "{}{}:",
                self.name,
                if self.next_role().is_some() {
                    " (Hiring)"
                } else {
                    ""
                }
            );
            for (s_id, s) in &self.staff {
                let staff_member = city.population.get(&s_id).unwrap();
                println!(
                    "   {}: {} {}",
                    s.title, staff_member.first_name, staff_member.last_name
                );
            }
        }
    }

    pub fn random_institution(dict: &Dictionary, city: &City) -> Institution {
        let mut rng = rand::thread_rng();
        let roll = rng.gen::<f32>();
        let underworld_chance = 0.2;
        let temple_chance = 0.2;
        if roll < underworld_chance {
            return generate_underground(dict, &city.culture);
        } else if roll < (underworld_chance + temple_chance) {
            return generate_temple(dict, &city.culture);
        } else {
            return generate_social(dict, &city.culture);
        }
    }

    pub fn generate_temple(dict: &Dictionary, culture: &Culture) -> Institution {
        let template = dict
            .get_random_template(vec![
                vec!["Altar".to_string()],
                vec![culture.era.to_string()],
            ])
            .unwrap();
        let diety_ref = culture.dieties.clone();
        let mut dieties: Vec<&Diety> = diety_ref.values().collect();
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
            management: vec![ManagementSpecification {
                title: String::from("Minister"),
                reportee_types: vec![String::from("Priest")],
                min_reportees: 1,
                max_reportees: 4,
            }],
            base_job_titles: vec!["Priest".to_string()],
            staff: HashMap::new(),
            related_diety: Some(diety.id.clone()),
            wealth: 0,
        };

        return output;
    }

    pub fn generate_underground(dict: &Dictionary, culture: &Culture) -> Institution {
        let template = dict
            .get_random_template(vec![
                vec!["SeedyBar".to_string(), "Fence".to_string()],
                vec![culture.era.to_string()],
            ])
            .unwrap();
        let is_fence = template.tags.contains("Fence");
        let output = Institution {
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
            wealth: 0,
        };
        return output;
    }

    pub fn generate_social(dict: &Dictionary, culture: &Culture) -> Institution {
        let template = dict
            .get_random_template(vec![
                vec!["Social".to_string()],
                vec![culture.era.to_string()],
            ])
            .unwrap();
        let mut output = Institution {
            id: Uuid::new_v4(),
            name: dict.render_template(&template.id).unwrap(),
            category: InsitutionCategory::Social,
            management: Vec::new(),
            base_job_titles: Vec::new(),
            staff: HashMap::new(),
            related_diety: None,
            wealth: 0,
        };
        if template.tags.contains("Food") {
            output.base_job_titles.push("Cook".to_string());
            output.management.push(ManagementSpecification {
                title: "Chef".to_string(),
                reportee_types: vec!["Cook".to_string()],
                min_reportees: 1,
                max_reportees: 4,
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
                min_reportees: 1,
                max_reportees: 4,
            });
        }
        if template.tags.contains("Music") {
            output.base_job_titles.push("Waiter".to_string());
            output.management.push(ManagementSpecification {
                title: "Musician".to_string(),
                reportee_types: vec!["Actor".to_string()],
                min_reportees: 1,
                max_reportees: 4,
            });
        }
        if template.tags.contains("Cinema") {
            output.base_job_titles.push("Waiter".to_string());
            output.management.push(ManagementSpecification {
                title: "Manager".to_string(),
                reportee_types: vec!["Shopkeeper".to_string()],
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
                    if institution.staff.len() > 1 {
                        institution.staff.remove(&mind.id);
                        // get institution
                        // remove from institution employees
                        mind.employer = None;
                    }
                }
            }
        }
        pub fn fill_and_create_jobs(self: &mut Self, dict: &Dictionary) {
            let population_ref = self.population.clone();
            let institution_ref = self.institutions.clone();
            let mut unemployed: Vec<&Mind> = population_ref
                .values()
                .filter(|m| m.employer.is_none())
                .collect();
            unemployed.shuffle(&mut rand::thread_rng());

            let mut hiring_institutions: Vec<Uuid> = institution_ref
                .values()
                .filter(|i| i.next_role().is_some())
                .map(|i| i.id)
                .collect();
            for mind in unemployed {
                // TEMP - late keep minds in same institution type unless chance to break out happens

                // find usable institution
                let target_key = hiring_institutions.first();

                if target_key.is_some() {
                    // let institution = self.institutions.get(&target_key).unwrap();

                    let mind_mut = self.population.get_mut(&mind.id).unwrap();
                    mind_mut.employer = Some(target_key.unwrap().clone());

                    let institution_mut = self.institutions.get_mut(&target_key.unwrap()).unwrap();
                    institution_mut.staff.insert(
                        mind.id.clone(),
                        StaffDefinition {
                            title: institution_mut.next_role().unwrap(),
                            employee_id: mind.id.clone(),
                            salary: 0,
                            started_year: self.year.clone(),
                        },
                    );
                    if institution_mut.next_role().is_none() {
                        hiring_institutions.retain(|i| !i.eq(&institution_mut.id));
                    }
                } else {
                    // New Institution
                    let mut new_institution = random_institution(dict, &self);
                    new_institution.staff.insert(
                        mind.id.clone(),
                        StaffDefinition {
                            title: new_institution.next_role().unwrap(),
                            employee_id: mind.id.clone(),
                            salary: 0,
                            started_year: self.year.clone(),
                        },
                    );
                    let mind_mut = self.population.get_mut(&mind.id).unwrap();
                    mind_mut.employer = Some(new_institution.id.clone());
                    hiring_institutions.push(new_institution.id.clone());
                    self.institutions
                        .insert(new_institution.id.clone(), new_institution);
                }
            }
        }
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
