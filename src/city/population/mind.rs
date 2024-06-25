pub mod physical_description;
pub mod relations;
pub mod mind {
    use crate::city::{
        city::Era,
        culture::culture::{random_culture, Culture},
    };

    use super::{
        physical_description::physical_description::{random_description, PhysicalDescription},
        relations::relations::RelationVerb,
    };
    use procgen_templater::dictionary::{dictionary::Dictionary, word::word::WordType};
    use rand::Rng;
    use std::{collections::HashMap, fmt};
    use uuid::Uuid;

    #[derive(PartialEq, Debug, Clone)]
    pub enum Gender {
        Male,
        Female,
        Ambiguous,
    }

    impl fmt::Display for Gender {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Gender::Male => write!(f, "Male"),
                Gender::Female => write!(f, "Female"),
                Gender::Ambiguous => write!(f, "Ambiguous"),
            }
        }
    }

    const AMBIGUOUS_GENDER_CHANCE: f32 = 0.1;

    #[derive(PartialEq, Debug, Clone)]
    pub enum Sexuality {
        Hetrosexual,
        Homosexual,
        Asexual,
        Bisexual,
    }

    const HOMOSEXUALITY_CHANCE: f32 = 0.075;
    const ASEXUALITY_CHANCE: f32 = 0.05;
    const BISEXUALITY_CHANCE: f32 = 0.75;

    pub type MindId = Uuid;

    #[derive(PartialEq, Debug, Clone)]
    pub struct Mind {
        pub id: MindId,
        pub alive: bool,
        pub first_name: String,
        pub last_name: String,
        pub gender: Gender,
        pub sexuality: Sexuality,
        pub relations: HashMap<MindId, Vec<RelationVerb>>,
        pub description: PhysicalDescription,
    }

    pub fn random_mind(dict: &Dictionary, culture: &Culture) -> Mind {
        let gender = random_gender();
        return Mind {
            id: Uuid::new_v4(),
            alive: true,
            first_name: dict
                .get_random_word((
                    WordType::Noun,
                    vec![
                        vec!["FirstName".to_string()],
                        vec![gender.to_string()],
                        vec![culture.era.to_string()],
                    ],
                ))
                .unwrap()
                .base
                .clone(),
            last_name: dict
                .get_random_word((
                    WordType::Noun,
                    vec![vec!["LastName".to_string()], vec![culture.era.to_string()]],
                ))
                .unwrap()
                .base
                .clone(),
            gender,
            sexuality: random_sexuality(),
            relations: HashMap::new(),
            description: random_description(&dict),
        };
    }

    fn random_gender() -> Gender {
        let roll = rand::thread_rng().gen::<f32>();
        if roll < AMBIGUOUS_GENDER_CHANCE {
            return Gender::Ambiguous;
        } else if roll < (AMBIGUOUS_GENDER_CHANCE + ((1.0 - AMBIGUOUS_GENDER_CHANCE) / 2.0)) {
            return Gender::Female;
        } else {
            return Gender::Male;
        }
    }

    fn random_sexuality() -> Sexuality {
        let roll = rand::thread_rng().gen::<f32>();
        if roll < HOMOSEXUALITY_CHANCE {
            return Sexuality::Homosexual;
        } else if roll < (HOMOSEXUALITY_CHANCE + ASEXUALITY_CHANCE) {
            return Sexuality::Asexual;
        } else if roll < (HOMOSEXUALITY_CHANCE + ASEXUALITY_CHANCE + BISEXUALITY_CHANCE) {
            return Sexuality::Bisexual;
        } else {
            return Sexuality::Hetrosexual;
        }
    }

    #[test]
    fn test_random_mind() {
        use procgen_templater::dictionary::dictionary::build_dictionary_from_folder;
        let dict = build_dictionary_from_folder("./data_files");
        let culture = random_culture(&dict, Era::Medieval);
        println!("{:#?}", random_mind(&dict, &culture))
    }
}
