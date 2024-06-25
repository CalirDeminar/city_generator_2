pub mod institutions {
    // Split via GUARDS
    // G - Goverment Buildings
    // U - Underworld Activities
    // A - Altars (Churches, Cults, etc)
    // R - Resources (Sale, Extraction or Production)
    // D - Defenses (Police, Guards, Sherrif, etc)
    // S - Social Hubs

    use std::collections::HashMap;

    use uuid::Uuid;

    use crate::city::population::mind::mind::MindId;

    pub enum InsitutionCategory {
        Goverment,
        Underworld,
        Altar,
        Resource,
        Defence,
        Social,
    }

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
    }

    pub type InstitutionId = Uuid;

    pub struct Institution {
        pub id: InstitutionId,
        pub name: String,
        pub category: InsitutionCategory,
        pub management: Vec<ManagementSpecification>,
        pub base_job_titles: Vec<String>,
        pub staff: HashMap<Uuid, StaffDefinition>,
    }
    pub struct ManagementSpecification {
        pub title: String,
        pub reportee_types: Vec<String>,
        pub min_reportees: u32,
        pub max_reportees: u32,
    }
    pub struct StaffDefinition {
        pub title: String,
        pub employee_id: MindId,
        pub salary: u32,
        pub started_year: u32,
    }

    #[test]
    fn test_name_gen() {
        use procgen_templater::dictionary::dictionary::build_dictionary_from_folder;
        let dict = build_dictionary_from_folder("./data_files");
        for _i in 0..50 {
            let template = dict
                .get_random_template(vec![vec!["Pub".to_string()]])
                .unwrap();
            println!("{}", dict.render_template(&template.id).unwrap());
        }
    }
}
