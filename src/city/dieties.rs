pub mod dieties {

    use std::collections::HashMap;

    use crate::{
        city::city::Era,
        grammar::grammar::{a_or_an, render_list},
    };
    use procgen_templater::dictionary::{
        dictionary::Dictionary,
        word::word::{Word, WordType},
    };
    use rand::Rng;
    use uuid::Uuid;

    pub type DietyId = Uuid;

    #[derive(PartialEq, Debug, Clone)]
    pub struct Diety {
        pub id: DietyId,
        pub realms: Vec<Word>,
        pub name: String,
        pub form: Word,
        pub form2: String,
    }

    impl Diety {
        pub fn render_summary(self: &Self) -> String {
            let realms: Vec<&str> = self.realms.iter().map(|r| r.base.as_str()).collect();
            return format!(
                "{}; god of {} takes the form of {} {}",
                self.name,
                render_list(realms),
                a_or_an(&self.form2),
                self.form2
            );
        }
    }

    pub fn random_dieties(dict: &Dictionary) -> HashMap<DietyId, Diety> {
        let mut output: HashMap<DietyId, Diety> = HashMap::new();
        for _i in 0..20 {
            let d = random_diety(&dict);
            output.insert(d.id.clone(), d);
        }
        return output;
    }

    pub fn random_diety(dict: &Dictionary) -> Diety {
        let form2_template = dict
            .get_random_template(vec![vec![String::from("AncientCreature")]])
            .unwrap();
        let name = dict
            .get_random_word((WordType::Noun, vec![vec!["Name".to_string()]]))
            .unwrap()
            .base
            .to_string();
        let quality = dict
            .get_random_word((
                WordType::Adjective,
                vec![vec![
                    String::from("Age"),
                    String::from("Height"),
                    String::from("ObjectState"),
                    String::from("BuildingState"),
                    String::from("Quality"),
                    String::from("Colour"),
                    String::from("Build"),
                ]],
            ))
            .unwrap()
            .base
            .to_string();
        return Diety {
            id: Uuid::new_v4(),
            realms: random_diety_realms(&dict, None),
            name: format!("{} the {}", name, quality),
            form: dict
                .get_random_word_without(
                    (WordType::Noun, vec![vec!["Creature".to_string()]]),
                    vec!["Mythical".to_string()],
                )
                .unwrap()
                .clone(),
            form2: dict.render_template(&form2_template.id).unwrap(),
        };
    }

    fn random_diety_realms(dict: &Dictionary, era: Option<&Era>) -> Vec<Word> {
        let full_era = if era.is_some() {
            era.unwrap().to_string()
        } else {
            Era::Medieval.to_string()
        };
        let mut output: Vec<Word> = Vec::new();
        let mut rng = rand::thread_rng();
        for mut _j in 0..2 {
            let base = dict
                .get_random_word((
                    WordType::Noun,
                    vec![vec!["DivineConcept".to_string()], vec![full_era.clone()]],
                ))
                .unwrap();
            // output.push(base.clone());

            for _i in 0..(if rng.gen::<bool>() { 1 } else { 2 }) {
                let next_base = dict.get_random_word_without(
                    (
                        WordType::Noun,
                        vec![
                            vec!["DivineConcept".to_string()],
                            vec![full_era.clone()],
                            vec![base.base.clone()],
                        ],
                    ),
                    output
                        .iter()
                        .map(|w| format!("Without{}", w.base))
                        .collect(),
                );
                if next_base.is_some() {
                    let nb = next_base.unwrap();
                    if !output.iter().any(|i| i.id.eq(&nb.id)) {
                        output.push(nb.clone());
                    }
                } else if !output.iter().any(|i| i.id.eq(&base.id)) {
                    output.push(base.clone());
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
            println!("{}", random_diety(&dict).render_summary());
        }
    }
}
