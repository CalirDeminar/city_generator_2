pub mod area;
pub mod culture;
pub mod dieties;
pub mod institutions;
pub mod population;
pub mod city {
    use std::{collections::HashMap, fmt};

    use uuid::Uuid;

    use super::{
        area::area::{Area, AreaId},
        culture::culture::Culture,
        population::population::Population,
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
    }
}
