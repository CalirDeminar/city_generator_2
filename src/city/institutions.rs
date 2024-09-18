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
    use rand::seq::SliceRandom;
    use uuid::Uuid;

    use crate::city::{
        city::City,
        dieties::dieties::{random_diety, Diety},
        population::mind::mind::MindId,
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
        // Underworld
        ShadyBar,
        // Altar
        Altar,
        Chapel,
        // Resource
        GeneralRetail,
        SpecialistRetail,
        // Defence
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

    pub fn generate_temple(dict: &Dictionary) -> (Institution, Diety) {
        let template = dict
            .get_random_template(vec![vec!["Altar".to_string()]])
            .unwrap();
        let diety = random_diety(dict);
        let output = Institution {
            id: Uuid::new_v4(),
            name: dict
                .render_template(&template.id)
                .unwrap()
                .replace("CONCEPT", &diety.realms.first().unwrap().base),
            category: InsitutionCategory::Social,
            management: Vec::new(),
            base_job_titles: vec!["Acolyte".to_string()],
            staff: HashMap::new(),
            related_diety: Some(diety.id.clone()),
        };

        return (output, diety);
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
                    // get institution
                    // remove from institution employees
                    mind.employer = None;
                }
            }
        }
    }

    #[test]
    fn test_name_gen() {
        use procgen_templater::dictionary::dictionary::build_dictionary_from_folder;
        let dict = build_dictionary_from_folder("./data_files");
        for _i in 0..50 {
            let inst = generate_social(&dict);
            println!("{:#?}", inst);
        }
    }
}
