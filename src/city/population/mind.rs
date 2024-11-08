pub mod personality;
pub mod physical_description;
pub mod relations;
pub mod mind {
    use crate::{
        city::{city::City, culture::culture::Culture, dieties::dieties::Diety},
        grammar::grammar::render_list,
    };

    use super::{
        personality::personality::{random_personality, Personality},
        physical_description::physical_description::{random_description, PhysicalDescription},
        relations::relations::RelationVerb,
    };
    use procgen_templater::dictionary::{dictionary::Dictionary, word::word::WordType};
    use rand::Rng;
    use std::{
        collections::{HashMap, HashSet},
        fmt,
    };
    use uuid::Uuid;

    #[derive(PartialEq, Debug, Clone, Hash, Eq)]
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

    #[derive(PartialEq, Debug, Clone, Hash, Eq)]
    pub enum Sexuality {
        Hetrosexual,
        Homosexual,
        Asexual,
        Bisexual,
    }

    impl fmt::Display for Sexuality {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Sexuality::Hetrosexual => write!(f, "straight"),
                Sexuality::Homosexual => write!(f, "gay"),
                Sexuality::Bisexual => write!(f, "bi"),
                Sexuality::Asexual => write!(f, "asexual"),
                _ => write!(f, ""),
            }
        }
    }

    const HOMOSEXUALITY_CHANCE: f32 = 0.075;
    const ASEXUALITY_CHANCE: f32 = 0.05;
    const BISEXUALITY_CHANCE: f32 = 0.075;

    pub type MindId = Uuid;

    #[derive(PartialEq, Debug, Clone)]
    pub struct Mind {
        pub id: MindId,
        pub alive: bool,
        pub first_name: String,
        pub last_name: String,
        pub age: u32,
        pub gender: Gender,
        pub sexuality: Sexuality,
        pub relations: HashMap<MindId, HashSet<RelationVerb>>,
        pub description: PhysicalDescription,
        pub personality: Personality,
        pub dieties: HashSet<Uuid>,
        pub employer: Option<Uuid>,
    }

    impl Mind {
        pub fn age(self: &mut Self) {
            if self.alive {
                self.age += 1;
                let death_threashhold = (self.age as f32 / 60.0).powf(2.5) * 0.1;
                println!("Death Threshhold {}: {:.2}", self.age, death_threashhold);
                let mut rng = rand::thread_rng();
                if rng.gen::<f32>() < death_threashhold {
                    self.alive = false;
                }
            }
        }
        pub fn inspect(self: &Self, city: &City) {
            println!(
                "\n{} {}, {}, age: {},  {}",
                self.first_name,
                self.last_name,
                self.gender,
                self.age,
                if self.alive { "Alive" } else { "Dead" }
            );
            println!(
                "They are {} and {}",
                self.sexuality,
                if self.is_single() { "Single" } else { "Taken" }
            );
            println!("{}", self.description.render(None));
            let traits: Vec<String> = self
                .personality
                .traits
                .iter()
                .map(|i| i.to_string().to_ascii_lowercase())
                .collect();
            println!(
                "They are said to be {}",
                render_list(traits.iter().map(|t| t.as_str()).collect())
            );
            for d_id in &self.dieties {
                let diety = city.culture.dieties.get(&d_id).unwrap();
                println!("They worship {}. {}.", diety.name, diety.render_summary());
            }
            println!("Relations: ");
            for (r_id, verbs) in &self.relations {
                let r = city.population.get(&r_id).unwrap();
                let verbs: Vec<String> = verbs.iter().map(|v| format!("{}", v)).collect();
                if verbs.len() > 0 {
                    println!(
                        "   {} {}: {}",
                        r.first_name,
                        r.last_name,
                        render_list(verbs.iter().map(|v| v.as_str()).collect())
                    );
                }
            }
        }
        pub fn is_single(self: &Self) -> bool {
            return !(self.sexuality.eq(&Sexuality::Asexual)
                || self.relations.iter().any(|(_, verbs)| {
                    verbs.contains(&RelationVerb::Spouse) || verbs.contains(&RelationVerb::Partner)
                }));
        }
    }

    pub fn random_mind(dict: &Dictionary, culture: &Culture) -> Mind {
        let gender = random_gender();
        let personality = random_personality(&culture);
        let mut dieties: HashSet<Uuid> = HashSet::new();
        if personality.thiest {
            let diety_list: Vec<&Diety> = culture.dieties.values().collect();
            let random_diety = diety_list
                .get((rand::thread_rng().gen::<f32>() * diety_list.len() as f32) as usize)
                .unwrap();
            dieties.insert(random_diety.id.clone());
        }
        let age_offset = rand::thread_rng().gen::<f32>() * 30.0;
        return Mind {
            id: Uuid::new_v4(),
            alive: true,
            age: culture.adult_age + (age_offset as u32),
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
            personality,
            dieties,
            employer: None,
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
        use crate::city::city::random_city;
        use crate::city::city::Era;
        use procgen_templater::dictionary::dictionary::build_dictionary_from_folder;
        let dict = build_dictionary_from_folder("./data_files");
        let city = random_city(&dict, Era::Medieval, 1);
        let m = random_mind(&dict, &city.culture);
        m.inspect(&city);
    }
}
