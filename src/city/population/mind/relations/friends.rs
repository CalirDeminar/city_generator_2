pub mod friends {
    use std::collections::{HashMap, HashSet};

    use uuid::Uuid;

    use crate::city::city::City;

    fn generate_age_cache(city: &City) -> HashMap<u32, HashSet<Uuid>> {
        let mut cache: HashMap<u32, HashSet<Uuid>> = HashMap::new();
        for mind in city.population.values() {
            if cache.contains_key(&mind.age) {
                cache.get_mut(&mind.age).unwrap().insert(mind.id.clone());
            } else {
                let mut i: HashSet<Uuid> = HashSet::new();
                i.insert(mind.id.clone());
                cache.insert(mind.age, i);
            }
        }
        return cache;
    }

    pub fn temp_add_friends<'a>(city: &'a mut City) -> &'a mut City {
        return city;
    }
}
