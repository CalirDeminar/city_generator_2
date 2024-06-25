pub mod area;
pub mod culture;
pub mod institutions;
pub mod population;
pub mod city {
    use std::fmt;

    use uuid::Uuid;

    use super::{culture::culture::Culture, population::population::Population};

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
    }
}
