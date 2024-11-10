pub mod friends {
    use std::collections::{HashMap, HashSet};

    use crate::city::{
        city::City,
        population::mind::{mind::MindId, relations::relations::RelationVerb},
    };
    use rand::{seq::SliceRandom, Rng};
    use uuid::Uuid;

    pub const SOCIAL_RELATIONS: [RelationVerb; 3] = [
        RelationVerb::Acquaintance,
        RelationVerb::Friend,
        RelationVerb::CloseFriend,
    ];

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

    fn filter_to_social_relations(
        r: &HashMap<Uuid, HashSet<RelationVerb>>,
    ) -> HashMap<Uuid, HashSet<RelationVerb>> {
        return r
            .iter()
            .filter(|(_id, verb_set)| verb_set.iter().any(|v| SOCIAL_RELATIONS.contains(&v)))
            .map(|(i, j)| (i.clone(), j.clone()))
            .collect();
    }

    fn temp_generate_eligible_friend_list(
        cache: &HashMap<u32, HashSet<Uuid>>,
        min_age: u32,
        max_age: u32,
        exclude: &HashSet<&Uuid>,
    ) -> Vec<Uuid> {
        let mut output: Vec<Uuid> = Vec::new();
        for i in min_age..max_age {
            if cache.contains_key(&i) {
                let set = cache.get(&i).unwrap();
                for j in set {
                    if !exclude.contains(&j) {
                        output.push(j.clone());
                    }
                }
            }
        }
        output.shuffle(&mut rand::thread_rng());
        return output;
    }

    fn temp_friend_evolution<'a>(city: &'a mut City, mind_id: &MindId) -> &'a mut City {
        let mut rng = rand::thread_rng();
        let mind_ref = city.population.get(mind_id).unwrap().clone();
        for (target_id, verbs) in &mind_ref.relations {
            let mut to_remove: Option<RelationVerb> = None;
            let mut to_add: Option<RelationVerb> = None;
            if verbs.contains(&RelationVerb::Acquaintance) {
                if rng.gen::<f32>() < 0.6 {
                    to_remove = Some(RelationVerb::Acquaintance);
                } else if rng.gen::<f32>() < 0.25 {
                    to_remove = Some(RelationVerb::Acquaintance);
                    to_add = Some(RelationVerb::Friend);
                }
            } else if verbs.contains(&RelationVerb::Friend) {
                if rng.gen::<f32>() < 0.25 {
                    to_remove = Some(RelationVerb::Friend);
                    to_add = Some(RelationVerb::Acquaintance);
                } else if rng.gen::<f32>() < 0.125 {
                    to_remove = Some(RelationVerb::Friend);
                    to_add = Some(RelationVerb::CloseFriend);
                }
            } else if verbs.contains(&RelationVerb::CloseFriend) {
                if rng.gen::<f32>() < 0.125 {
                    to_remove = Some(RelationVerb::CloseFriend);
                    to_add = Some(RelationVerb::Friend);
                }
            }

            if to_remove.is_some() {
                let remove = to_remove.unwrap();
                let m = city.population.get_mut(mind_id).unwrap();
                m.relations
                    .get_mut(target_id)
                    .unwrap()
                    .retain(|v| !v.eq(&remove));
                let t = city.population.get_mut(target_id).unwrap();
                t.relations
                    .get_mut(mind_id)
                    .unwrap()
                    .retain(|v| !v.eq(&remove));
            }
            if to_add.is_some() {
                let add = to_add.unwrap();
                let m = city.population.get_mut(mind_id).unwrap();
                if !m.relations.contains_key(target_id) {
                    m.relations.insert(target_id.clone(), HashSet::new());
                }
                m.relations.get_mut(target_id).unwrap().insert(add.clone());
                let t = city.population.get_mut(target_id).unwrap();
                if !t.relations.contains_key(mind_id) {
                    t.relations.insert(mind_id.clone(), HashSet::new());
                }
                t.relations.get_mut(mind_id).unwrap().insert(add.clone());
            }
        }
        return city;
    }

    impl City {
        pub fn temp_add_friends(self: &mut Self) {
            let city = self;
            let mut rng = rand::thread_rng();
            let city_clone = city.clone();
            let mind_ids = city_clone.current_citizens();
            let age_cache = generate_age_cache(&city);
            for m_id in mind_ids.clone() {
                let mind_clone = city.population.get(&m_id).unwrap().clone();
                let social_relations = filter_to_social_relations(&mind_clone.relations);
                let current_friends: HashSet<&Uuid> =
                    HashSet::from_iter(social_relations.iter().map(|(id, _rest)| id));
                let friend_count = current_friends.len();
                let to_add =
                    ((rng.gen::<f32>() * 20.0) as i32 - friend_count as i32).max(0) as usize;

                let source_list = temp_generate_eligible_friend_list(
                    &age_cache,
                    (mind_clone.age - 5).max(5),
                    mind_clone.age + 5,
                    &current_friends,
                );
                for _i in 0..to_add {
                    let index = (rng.gen::<f32>() * source_list.len() as f32) as usize;
                    let possible_target_mind_id = source_list.get(index);
                    if possible_target_mind_id.is_some() {
                        let target_mind_id = possible_target_mind_id.unwrap();
                        if !mind_clone.is_relation_of(target_mind_id) {
                            let source_mind_mut = city.population.get_mut(&m_id).unwrap();
                            if !source_mind_mut.relations.contains_key(&target_mind_id) {
                                source_mind_mut
                                    .relations
                                    .insert(target_mind_id.clone(), HashSet::new());
                            }
                            source_mind_mut
                                .relations
                                .get_mut(&target_mind_id)
                                .unwrap()
                                .insert(RelationVerb::Acquaintance);

                            let target_mind_mut = city.population.get_mut(&target_mind_id).unwrap();
                            if !target_mind_mut.relations.contains_key(&m_id) {
                                target_mind_mut
                                    .relations
                                    .insert(m_id.clone(), HashSet::new());
                            }
                            target_mind_mut
                                .relations
                                .get_mut(&m_id)
                                .unwrap()
                                .insert(RelationVerb::Acquaintance);
                        }
                    }
                }
            }
            for m in mind_ids {
                temp_friend_evolution(city, &m);
            }
        }
    }
}
