pub mod physical_description {
    use procgen_templater::dictionary::{dictionary::Dictionary, word::word::WordType};
    use rand::Rng;

    #[derive(PartialEq, Debug, Clone)]
    pub struct PhysicalDescription {
        pub hair_colour: String,
        pub hair_style: String,
        pub hair_adjective: String,
        pub eye_colour: String,
        pub height_adjective: String,
        pub build_adjective: String,
    }

    impl PhysicalDescription {
        pub fn render(&self, pronoun: Option<String>) -> String {
            let active_pronoun = if pronoun.is_some() {
                pronoun.unwrap()
            } else {
                "They".to_string()
            };
            return format!(
                "{} have {} {} {} hair and {} eyes. {} are {} with a {} build.",
                active_pronoun,
                self.hair_adjective,
                self.hair_style,
                self.hair_colour,
                self.eye_colour,
                active_pronoun,
                self.height_adjective,
                self.build_adjective
            );
        }
    }

    fn random_descriptor(dict: &Dictionary, tag: String) -> String {
        return dict
            .get_random_word((
                WordType::Adjective,
                vec![vec![tag], vec!["Personal".to_string()]],
            ))
            .unwrap()
            .base
            .to_ascii_lowercase()
            .to_string();
    }

    pub fn random_description(dict: &Dictionary) -> PhysicalDescription {
        return PhysicalDescription {
            hair_colour: random_descriptor(dict, "HairColour".to_string()),
            hair_style: random_descriptor(dict, "HairStyle".to_string()),
            hair_adjective: random_descriptor(dict, "HairState".to_string()),
            eye_colour: random_descriptor(dict, "EyeColour".to_string()),
            height_adjective: random_descriptor(dict, "Height".to_string()),
            build_adjective: random_descriptor(dict, "Build".to_string()),
        };
    }

    fn choose_or_mutate_attribute(
        dict: &Dictionary,
        tag: String,
        a1: &String,
        a2: &String,
    ) -> String {
        let mut rng = rand::thread_rng();
        let roll = rng.gen::<f32>();
        if roll < 0.45 {
            return a1.clone();
        } else if roll < 0.9 {
            return a2.clone();
        } else {
            return random_descriptor(dict, tag);
        }
    }

    pub fn merge_descriptions(
        dict: &Dictionary,
        p1: &PhysicalDescription,
        p2: &PhysicalDescription,
    ) -> PhysicalDescription {
        return PhysicalDescription {
            hair_colour: choose_or_mutate_attribute(
                &dict,
                "HairColour".to_string(),
                &p1.hair_colour,
                &p2.hair_colour,
            ),
            hair_style: random_descriptor(dict, "HairStyle".to_string()),
            hair_adjective: choose_or_mutate_attribute(
                &dict,
                "HairState".to_string(),
                &p1.hair_adjective,
                &p2.hair_adjective,
            ),
            eye_colour: choose_or_mutate_attribute(
                &dict,
                "EyeColour".to_string(),
                &p1.eye_colour,
                &p2.eye_colour,
            ),
            height_adjective: choose_or_mutate_attribute(
                &dict,
                "Height".to_string(),
                &p1.height_adjective,
                &p2.height_adjective,
            ),
            build_adjective: choose_or_mutate_attribute(
                &dict,
                "Build".to_string(),
                &p1.build_adjective,
                &p2.build_adjective,
            ),
        };
    }

    // #[test]
    // fn test_random_descriptions() {
    //     use procgen_templater::dictionary::dictionary::build_dictionary_from_folder;
    //     let dict = build_dictionary_from_folder("./data_files");
    //     let mut prev = random_description(&dict);
    //     for _i in 0..50 {
    //         let d = random_description(&dict);
    //         println!("{}", merge_descriptions(&dict, &prev, &d).render(None));
    //         prev = d;
    //     }
    // }
}
