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

    pub struct Institution {
        pub id: Uuid,
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
        pub employee_id: Uuid,
        pub salary: u32,
        pub started_year: u32,
    }
}
