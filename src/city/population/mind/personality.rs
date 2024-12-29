pub mod personality {
    use rand::Rng;
    use std::{collections::HashSet, fmt};

    use crate::city::culture::culture::Culture;

    #[derive(PartialEq, Debug, Clone)]
    pub struct Personality {
        pub matrix: PersonalityMatrix,
        pub traits: HashSet<PersonalityTrait>,
        pub thiest: bool,
    }
    #[derive(PartialEq, Debug, Clone)]
    pub struct PersonalityMatrix {
        // athiest <-> thiest
        pub spirituality: f32,
        // sly/greedy/deceitful <-> sincere/honest/faithful
        pub humility: f32,
        // brave/tough/self_assured <-> emotional/fearful/sentimental
        pub emotionality: f32,
        // shy/passive/introverted <-> outgoing/lively/extraverted
        pub extraversion: f32,
        // stubborn/bad-tempered/choleric <-> patient/tolerant/peaceful
        pub agreeableness: f32,
        // sloppy/reckless/negligent <-> dilligent/careful/disciplined
        pub conscientiousness: f32,
        // shallow/uninmaginative/conventional <-> creative/innovative/intellectual
        pub experience_openness: f32,
    }

    impl fmt::Display for PersonalityTrait {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                PersonalityTrait::Devout => write!(f, "Devout"),
                PersonalityTrait::Greedy => write!(f, "Greedy"),
                PersonalityTrait::Deceitful => write!(f, "Deceitful"),
                PersonalityTrait::Honest => write!(f, "Honest"),
                PersonalityTrait::Faithful => write!(f, "Faithful"),
                PersonalityTrait::SelfAssured => write!(f, "Self Assured"),
                PersonalityTrait::Cold => write!(f, "Cold"),
                PersonalityTrait::Sentimental => write!(f, "Sentimental"),
                PersonalityTrait::Emotional => write!(f, "Emotional"),
                PersonalityTrait::Introverted => write!(f, "Introverted"),
                PersonalityTrait::Shy => write!(f, "Shy"),
                PersonalityTrait::Extraverted => write!(f, "Extraverted"),
                PersonalityTrait::Lively => write!(f, "Lively"),
                PersonalityTrait::Stubborn => write!(f, "Stubborn"),
                PersonalityTrait::BadTempered => write!(f, "Bad Tempered"),
                PersonalityTrait::Patient => write!(f, "Patient"),
                PersonalityTrait::Tolerant => write!(f, "Tolerant"),
                PersonalityTrait::Reckless => write!(f, "Reckless"),
                PersonalityTrait::Sloppy => write!(f, "Sloppy"),
                PersonalityTrait::Disciplined => write!(f, "Disciplined"),
                PersonalityTrait::Careful => write!(f, "Careful"),
                PersonalityTrait::Unimaginative => write!(f, "Unimaginative"),
                PersonalityTrait::Shallow => write!(f, "Shallow"),
                PersonalityTrait::Creative => write!(f, "Creative"),
                PersonalityTrait::Intellectual => write!(f, "Intellectual"),
            }
        }
    }

    #[derive(PartialEq, Debug, Clone, Hash, Eq)]
    pub enum PersonalityTrait {
        // Pure Spirituality
        Devout,
        // Pure Humily Traits
        Greedy,
        Deceitful,
        Honest,
        Faithful,
        // Pure Emotionality Traits
        SelfAssured,
        Cold, // TODO - find better description of this
        Sentimental,
        Emotional,
        // Pure Extraversion Traits
        Introverted,
        Shy,
        Extraverted,
        Lively,
        // Pure Agreeableness Traits
        Stubborn,
        BadTempered,
        Patient,
        Tolerant,
        // Pure Conscientiousness Traits
        Reckless,
        Sloppy,
        Disciplined,
        Careful,
        // Pure Openness Traits
        Unimaginative,
        Shallow,
        Creative,
        Intellectual,
    }

    pub fn random_personality(culture: &Culture) -> Personality {
        let mut rng = rand::thread_rng();
        let mut output = Personality {
            matrix: PersonalityMatrix {
                spirituality: rng.gen::<f32>(),
                humility: rng.gen::<f32>(),
                emotionality: rng.gen::<f32>(),
                extraversion: rng.gen::<f32>(),
                agreeableness: rng.gen::<f32>(),
                conscientiousness: rng.gen::<f32>(),
                experience_openness: rng.gen::<f32>(),
            },
            traits: HashSet::new(),
            thiest: false,
        };
        let mut roll = |t: f32| rng.gen::<f32>() < t;
        if output.matrix.spirituality + (culture.spirituality / 2.0) > 0.25 {
            output.thiest = true;
            if output.matrix.spirituality > 0.85 {
                output.traits.insert(PersonalityTrait::Devout);
            }
        }
        // Humility Low
        if output.matrix.humility < 0.4 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Greedy);
        } else if output.matrix.humility < 0.2 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Deceitful);
        }
        // Humility High
        if output.matrix.humility > 0.6 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Honest);
        } else if output.matrix.humility > 0.8 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Faithful);
        }

        // Emotionality Low
        if output.matrix.emotionality < 0.4 && roll(0.5) {
            output.traits.insert(PersonalityTrait::SelfAssured);
        } else if output.matrix.emotionality < 0.2 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Cold);
        }
        // Emotionality High
        if output.matrix.emotionality > 0.6 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Sentimental);
        } else if output.matrix.emotionality > 0.8 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Emotional);
        }

        // Extraversion Low
        if output.matrix.extraversion < 0.4 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Shy);
        } else if output.matrix.extraversion < 0.2 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Introverted);
        }
        // Extraversion High
        if output.matrix.extraversion > 0.6 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Lively);
        } else if output.matrix.extraversion > 0.8 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Extraverted);
        }

        // Agreeableness Low
        if output.matrix.agreeableness < 0.4 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Stubborn);
        } else if output.matrix.agreeableness < 0.2 && roll(0.5) {
            output.traits.insert(PersonalityTrait::BadTempered);
        }
        // Agreeableness High
        if output.matrix.agreeableness > 0.6 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Tolerant);
        } else if output.matrix.agreeableness > 0.8 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Patient);
        }

        // Conscientiousness Low
        if output.matrix.conscientiousness < 0.4 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Sloppy);
        } else if output.matrix.conscientiousness < 0.2 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Reckless);
        }
        // Conscientiousness High
        if output.matrix.conscientiousness > 0.6 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Careful);
        } else if output.matrix.conscientiousness > 0.8 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Disciplined);
        }

        // Openness Low
        if output.matrix.experience_openness < 0.4 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Shallow);
        } else if output.matrix.experience_openness < 0.2 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Unimaginative);
        }
        // Openness High
        if output.matrix.experience_openness > 0.6 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Intellectual);
        } else if output.matrix.experience_openness > 0.8 && roll(0.5) {
            output.traits.insert(PersonalityTrait::Creative);
        }

        return output;
    }

    // #[test]
    // fn test_random_personality() {
    //     use crate::city::city::Era;
    //     use crate::city::culture::culture::random_culture;
    //     use procgen_templater::dictionary::dictionary::build_dictionary_from_folder;
    //     let dict = build_dictionary_from_folder("./data_files");
    //     let culture = random_culture(&dict, Era::Medieval);
    //     for _i in 0..50 {
    //         println!("{:#?}", random_personality(&culture))
    //     }
    // }
}
