pub mod surnames;
pub mod culture {
    use std::collections::{HashMap, HashSet};

    use procgen_templater::dictionary::{
        dictionary::Dictionary,
        word::word::{Word, WordType},
    };
    use rand::Rng;
    use uuid::Uuid;

    use crate::city::{
        city::Era,
        dieties::dieties::{random_dieties, Diety, DietyId},
    };

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
        pub historical_names: Vec<(Word, Word, Word)>,
        pub spirituality: f32,
        pub dieties: HashMap<DietyId, Diety>,
        pub era: Era,
    }

    pub fn random_culture(dictionary: &Dictionary, era: Era) -> Culture {
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
            historical_names: generate_historical_figures(dictionary, &era),
            era,
            dieties: random_dieties(&dictionary),
            spirituality: rng.gen::<f32>(),
        };
    }

    fn balance_words_to_dict<'a>(
        dict: &'a mut Dictionary,
        words: &Vec<Word>,
        tags: Vec<String>,
        fraction: f32,
    ) -> &'a mut Dictionary {
        let mut id_set: HashSet<Uuid> = HashSet::new();
        for (i, tag) in tags.iter().enumerate() {
            if i.eq(&0) {
                id_set = dict
                    .index
                    .tag_words
                    .get(&(WordType::Noun, tag.to_string()))
                    .unwrap()
                    .clone();
            } else {
                let buffer = dict
                    .index
                    .tag_words
                    .get(&(WordType::Noun, tag.to_string()))
                    .unwrap()
                    .clone();
                id_set.retain(|t| buffer.contains(t));
            }
        }
        let tag_count_in_dict = id_set.len();
        let repeats = (((1.0 - (words.len() as f32 / tag_count_in_dict as f32)) * fraction)
            * tag_count_in_dict as f32) as usize;
        for word in words {
            for _i in 0..repeats {
                let mut wc = word.clone();
                let id = Uuid::new_v4();
                wc.id = id.clone();
                dict.words.insert(id, wc);
                for t in &word.tags {
                    dict.index
                        .tag_words
                        .get_mut(&(WordType::Noun, t.to_string()))
                        .unwrap()
                        .insert(id.clone());
                }
            }
        }
        return dict;
    }

    pub fn rebalance_dict_for_culture(culture: &Culture, dictionary: &Dictionary) -> Dictionary {
        let mut dict = dictionary.clone();

        balance_words_to_dict(
            &mut dict,
            &culture.staple_meats,
            vec!["Creature".to_string()],
            1.0,
        );

        balance_words_to_dict(
            &mut dict,
            &culture.staple_plants,
            vec!["Plant".to_string()],
            1.0,
        );

        let titles: Vec<Word> = culture
            .historical_names
            .iter()
            .map(|i| i.0.clone())
            .collect();
        balance_words_to_dict(&mut dict, &titles, vec!["Title".to_string()], 0.5);

        let fnames: Vec<Word> = culture
            .historical_names
            .iter()
            .map(|i| i.1.clone())
            .collect();
        balance_words_to_dict(&mut dict, &fnames, vec!["FirstName".to_string()], 0.2);

        let lnames: Vec<Word> = culture
            .historical_names
            .iter()
            .map(|i| i.2.clone())
            .collect();
        balance_words_to_dict(&mut dict, &lnames, vec!["LastName".to_string()], 0.2);

        return dict;
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
                                "Mythical".to_string(),
                            ],
                        )
                        .unwrap()
                        .clone(),
                );
            }
        }
        return output;
    }

    fn generate_historical_figures(dictionary: &Dictionary, era: &Era) -> Vec<(Word, Word, Word)> {
        let mut rng = rand::thread_rng();
        let mut historical_names: Vec<(Word, Word, Word)> = Vec::new();
        for _i in 0..((rng.gen::<f32>() * 10.0) + 3.0) as usize {
            let gender = if rng.gen::<bool>() { "Male" } else { "Female" }.to_string();
            let title = dictionary
                .get_random_word((
                    WordType::Noun,
                    vec![vec!["Title".to_string()], vec![gender.clone()]],
                ))
                .unwrap()
                .clone();
            let f_name = dictionary
                .get_random_word((
                    WordType::Noun,
                    vec![
                        vec!["FirstName".to_string()],
                        vec![gender.clone()],
                        vec![format!("{}", era)],
                    ],
                ))
                .unwrap()
                .clone();
            let l_name = dictionary
                .get_random_word((
                    WordType::Noun,
                    vec![vec!["LastName".to_string()], vec![format!("{}", era)]],
                ))
                .unwrap()
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
            random_culture(&build_dictionary_from_folder("./data_files"), Era::Medieval)
        );
    }

    #[test]
    fn rebalance_dict() {
        use procgen_templater::dictionary::dictionary::build_dictionary_from_folder;
        let dict = build_dictionary_from_folder("./data_files");
        let culture = random_culture(&dict, Era::Medieval);

        let dict2 = rebalance_dict_for_culture(&culture, &dict);
        assert!(
            dict.index
                .tag_words
                .get(&(WordType::Noun, "Creature".to_string()))
                .unwrap()
                .len()
                < dict2
                    .index
                    .tag_words
                    .get(&(WordType::Noun, "Creature".to_string()))
                    .unwrap()
                    .len()
        );
    }
}
