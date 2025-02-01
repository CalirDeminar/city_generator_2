pub mod personality;
pub mod physical_description;
pub mod relations;
pub mod mind {
    use crate::{
        city::{city::City, culture::culture::Culture, dieties::dieties::Diety},
        grammar::grammar::{a_or_an, render_list},
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
        pub origional_last_name: String,
        pub age: u32,
        pub gender: Gender,
        pub sexuality: Sexuality,
        pub relations: HashMap<RelationVerb, HashSet<Uuid>>,
        pub description: PhysicalDescription,
        pub personality: Personality,
        pub dieties: HashSet<Uuid>,
        pub employer: Option<Uuid>,
        pub year_of_birth: i32,
    }

    impl Mind {
        pub fn age(self: &mut Self) {
            if self.alive {
                self.age += 1;
                let death_threashhold =
                    ((self.age as f32 - 30.0) / 30.0).max(0.01).powf(2.25) * 0.1;
                // println!("Death Threshhold {}: {:.2}", self.age, death_threashhold);
                let mut rng = rand::thread_rng();
                if rng.gen::<f32>() < death_threashhold {
                    self.alive = false;
                }
            }
        }
        pub fn inspect(self: &Self, city: &City) {
            println!(
                "\n{} {}, {}, age: {}, born: {},  {}",
                self.first_name,
                self.last_name,
                self.gender,
                self.age,
                self.year_of_birth,
                if self.alive { "Alive" } else { "Dead" }
            );
            println!(
                "  They are {} and {}",
                self.sexuality,
                if self.is_single() { "Single" } else { "Taken" }
            );
            println!("  {}", self.description.render(None));
            let traits: Vec<String> = self
                .personality
                .traits
                .iter()
                .map(|i| i.to_string().to_ascii_lowercase())
                .collect();
            println!(
                "  They are said to be {}",
                render_list(traits.iter().map(|t| t.as_str()).collect())
            );
            for d_id in &self.dieties {
                let diety = city.culture.dieties.get(&d_id).unwrap();
                println!("  They worship {}. {}.", diety.name, diety.render_summary());
            }
            if self.employer.is_some() {
                let employer = city.institutions.get(&self.employer.unwrap()).unwrap();
                let position = employer.staff.get(&self.id).unwrap();
                println!(
                    "  They work at: \"{}\" as {} {}",
                    employer.name,
                    a_or_an(&position.title),
                    position.title
                );
            }
            println!("  Relations: ");
            for (verb, rel_ids) in &self.relations {
                for r_id in rel_ids {
                    let r = city.population.get(&r_id).unwrap();
                    println!(
                        "       {} {}: {} {}",
                        r.first_name,
                        r.last_name,
                        if r.alive { "" } else { "Late" },
                        verb
                    );
                }
            }
        }
        pub fn is_single(self: &Self) -> bool {
            return (self.sexuality.eq(&Sexuality::Asexual)
                || ((!self.relations.contains_key(&RelationVerb::Spouse)
                    || self
                        .relations
                        .get(&RelationVerb::Spouse)
                        .unwrap()
                        .len()
                        .eq(&0))
                    && (!self.relations.contains_key(&RelationVerb::Partner)
                        || self
                            .relations
                            .get(&RelationVerb::Partner)
                            .unwrap()
                            .len()
                            .eq(&0))));
        }
        pub fn is_relation_of(self: &Self, other: &Uuid) -> bool {
            let relation_verbs = vec![
                RelationVerb::Parent,
                RelationVerb::Grandparent,
                RelationVerb::Child,
                RelationVerb::Grandchild,
                RelationVerb::Pibling,
                RelationVerb::Nibling,
                RelationVerb::Cousin,
            ];
            for v in relation_verbs {
                let possible_ids = self.relations.get(&v);
                if possible_ids.is_some() {
                    let ids = possible_ids.unwrap();
                    if ids.contains(other) {
                        return true;
                    }
                }
            }
            return false;
        }
        pub fn get_relations(self: &Self, verb: RelationVerb) -> HashSet<Uuid> {
            let rel = self.relations.get(&verb);
            if rel.is_none() {
                return HashSet::new();
            } else {
                return rel.unwrap().clone();
            }
        }
        pub fn cleanup(self: &mut Self) {
            let relation_ref = self.relations.clone();
            for (id, verbs) in &relation_ref {
                if verbs.len().eq(&0) {
                    self.relations.remove(&id);
                }
            }
        }
    }

    pub fn random_mind(dict: &Dictionary, culture: &Culture, year: i32) -> Mind {
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
        let last_name = dict
            .get_random_word((
                WordType::Noun,
                vec![vec!["LastName".to_string()], vec![culture.era.to_string()]],
            ))
            .unwrap()
            .base
            .clone();
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
            last_name: last_name.clone(),
            origional_last_name: last_name,
            gender,
            sexuality: random_sexuality(),
            relations: HashMap::new(),
            description: random_description(&dict),
            personality,
            dieties,
            employer: None,
            year_of_birth: year - culture.adult_age as i32 + (age_offset as i32),
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
        let m = random_mind(&dict, &city.culture, 0);
        m.inspect(&city);
    }
}
