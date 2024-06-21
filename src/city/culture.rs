pub mod surnames;
pub mod culture {
    use procgen_templater::dictionary::{
        dictionary::Dictionary,
        word::word::{Word, WordType},
    };
    use rand::Rng;
    use uuid::Uuid;

    use super::surnames::surnames::{
        random_child_surname_formats, random_marriage_surname_formats, SurnameFormat,
    };

    #[derive(PartialEq, Debug, Clone)]
    pub struct Culture {
        pub id: Uuid,
        pub adult_age: u32,
        pub landlocked: bool,
        pub staple_meats: Vec<Word>,
        pub staple_plants: Vec<Word>,
        pub avg_lifespan: u32,
        pub avg_lifespan_variance: u32,
        // surname formats are layed out in a (male (or eldest), female (or youngest)) format
        pub child_surname_formats: Vec<(SurnameFormat, SurnameFormat)>,
        pub marriage_surname_formats: Vec<(SurnameFormat, SurnameFormat)>,
        pub historical_names: Vec<(String, String, String)>,
    }

    pub fn random_culture(dictionary: &Dictionary) -> Culture {
        let mut rng = rand::thread_rng();
        let landlocked = rng.gen::<f32>() > 0.5;

        return Culture {
            id: Uuid::new_v4(),
            adult_age: 18 + ((8.0 * rng.gen::<f32>()) * -4.0) as u32,
            landlocked,
            staple_meats: generate_random_meats(dictionary, &landlocked),
            staple_plants: generate_random_plants(dictionary),
            avg_lifespan: 75 + (30.0 * rng.gen::<f32>() - 15.0) as u32,
            avg_lifespan_variance: 10,
            child_surname_formats: random_child_surname_formats(),
            marriage_surname_formats: random_marriage_surname_formats(),
            historical_names: generate_historical_figures(dictionary),
        };
    }

    fn generate_random_plants(dictionary: &Dictionary) -> Vec<Word> {
        let mut output: Vec<Word> = Vec::new();
        let mut rng = rand::thread_rng();
        for _i in 0..(rng.gen::<f32>() * 4.0) as usize + 1 {
            output.push(
                dictionary
                    .get_random_word((WordType::Noun, vec![vec!["Crop".to_string()]]))
                    .unwrap()
                    .clone(),
            )
        }
        for _i in 0..(rng.gen::<f32>() * 2.0) as usize + 1 {
            output.push(
                dictionary
                    .get_random_word((WordType::Noun, vec![vec!["Grain".to_string()]]))
                    .unwrap()
                    .clone(),
            )
        }
        return output;
    }

    fn generate_random_meats(dictionary: &Dictionary, landlocked: &bool) -> Vec<Word> {
        let mut output: Vec<Word> = Vec::new();
        let mut rng = rand::thread_rng();
        for _i in 0..(rng.gen::<f32>() * 10.0) as usize + 3 {
            if *landlocked || rng.gen::<f32>() > 0.3 {
                // land creature (non carnivore)
                // should exclude ocean creatures
                output.push(
                    dictionary
                        .get_random_word_without(
                            (
                                WordType::Noun,
                                vec![
                                    vec![
                                        "Mammal".to_string(),
                                        "Bird".to_string(),
                                        "Reptile".to_string(),
                                    ],
                                    vec!["Medium".to_string(), "Large".to_string()],
                                ],
                            ),
                            vec![
                                "Carnivore".to_string(),
                                "Sentient".to_string(),
                                "Ocean".to_string(),
                                "Magical".to_string(),
                            ],
                        )
                        .unwrap()
                        .clone(),
                );
            } else {
                // fish (can be carivore)
                output.push(
                    dictionary
                        .get_random_word_without(
                            (
                                WordType::Noun,
                                vec![
                                    vec!["Fish".to_string(), "Mammal".to_string()],
                                    vec!["Ocean".to_string()],
                                ],
                            ),
                            vec![
                                "Sentient".to_string(),
                                "Collosal".to_string(),
                                "Magical".to_string(),
                            ],
                        )
                        .unwrap()
                        .clone(),
                );
            }
        }
        return output;
    }

    fn generate_historical_figures(dictionary: &Dictionary) -> Vec<(String, String, String)> {
        let mut rng = rand::thread_rng();
        let mut historical_names: Vec<(String, String, String)> = Vec::new();
        for _i in 0..(rng.gen::<f32>() * 10.0) as usize {
            let gender = if rng.gen::<bool>() { "Male" } else { "Female" }.to_string();
            let title = dictionary
                .get_random_word((
                    WordType::Noun,
                    vec![vec!["Title".to_string()], vec![gender.clone()]],
                ))
                .unwrap()
                .base
                .clone();
            let f_name = dictionary
                .get_random_word((
                    WordType::Noun,
                    vec![vec!["FirstName".to_string()], vec![gender.clone()]],
                ))
                .unwrap()
                .base
                .clone();
            let l_name = dictionary
                .get_random_word((WordType::Noun, vec![vec!["LastName".to_string()]]))
                .unwrap()
                .base
                .clone();
            historical_names.push((title, f_name, l_name));
        }
        return historical_names;
    }

    #[test]
    fn test_random_culture() {
        use procgen_templater::dictionary::dictionary::build_dictionary_from_folder;
        println!(
            "{:#?}",
            random_culture(&build_dictionary_from_folder("./data_files"))
        );
    }
}
