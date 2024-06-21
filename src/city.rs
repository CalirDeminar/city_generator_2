pub mod culture;
pub mod institutions;
pub mod city {
    use uuid::Uuid;

    use super::culture::culture::Culture;

    pub enum Era {
        Modern,
        Medieval,
        Fantasy,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct City {
        pub id: Uuid,
        pub name: String,
        pub culture: Culture,
    }
}
