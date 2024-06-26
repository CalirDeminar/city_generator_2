pub mod dieties {

    use crate::city::city::Era;
    use procgen_templater::dictionary::{
        dictionary::Dictionary,
        word::word::{Word, WordType},
    };
    use rand::Rng;

    fn random_diety_concepts(dict: &Dictionary, era: Option<&Era>) -> Vec<Word> {
        let full_era = if era.is_some() {
            era.unwrap().to_string()
        } else {
            Era::Medieval.to_string()
        };
        let mut output: Vec<Word> = Vec::new();
        let mut rng = rand::thread_rng();
        let base = dict
            .get_random_word((
                WordType::Noun,
                vec![vec!["DivineConcept".to_string()], vec![full_era.clone()]],
            ))
            .unwrap();
        output.push(base.clone());

        for _i in 0..(rng.gen::<f32>() * 3.0) as usize {
            let next_base = dict.get_random_word((
                WordType::Noun,
                vec![
                    vec!["DivineConcept".to_string()],
                    vec![full_era.clone()],
                    vec![base.base.clone()],
                ],
            ));
            if next_base.is_some() {
                let nb = next_base.unwrap();
                if !output.iter().any(|i| i.id.eq(&nb.id)) {
                    output.push(nb.clone());
                }
            }
        }

        return output;
    }

    #[test]
    fn test_gen_diety_concepts() {
        use procgen_templater::dictionary::dictionary::build_dictionary_from_folder;
        let dict = build_dictionary_from_folder("./data_files");
        for _i in 0..25 {
            let concepts: Vec<String> = random_diety_concepts(&dict, None)
                .iter()
                .map(|w| w.base.clone())
                .collect();
            println!("{:#?}", concepts);
        }
    }
}
