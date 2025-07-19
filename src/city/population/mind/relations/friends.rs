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

    pub const FRIEND_EXCLUSIONS: [RelationVerb; 11] = [
        RelationVerb::ExPartner,
        RelationVerb::ExSpouse,
        RelationVerb::Partner,
        RelationVerb::Spouse,
        RelationVerb::Parent,
        RelationVerb::Child,
        RelationVerb::Cousin,
        RelationVerb::Grandchild,
        RelationVerb::Grandparent,
        RelationVerb::Pibling,
        RelationVerb::Nibling,
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
        r: &HashMap<RelationVerb, HashSet<Uuid>>,
    ) -> HashMap<Uuid, HashSet<RelationVerb>> {
        // return r
        //     .iter()
        //     .filter(|(_id, verb_set)| verb_set.iter().any(|v| SOCIAL_RELATIONS.contains(&v)))
        //     .map(|(i, j)| (i.clone(), j.clone()))
        //     .collect();
        let mut output: HashMap<Uuid, HashSet<RelationVerb>> = HashMap::new();
        for (verb, ids) in r {
            if SOCIAL_RELATIONS.contains(verb) {
                for id in ids {
                    if !output.contains_key(id) {
                        output.insert(id.clone(), HashSet::new());
                    }
                    output.get_mut(id).unwrap().insert(verb.clone());
                }
            }
        }
        return output;
    }

    fn filter_to_friend_exclusion_list(
        r: &HashMap<RelationVerb, HashSet<Uuid>>,
    ) -> HashMap<Uuid, HashSet<RelationVerb>> {
        // return r
        //     .iter()
        //     .filter(|(_id, verb_set)| verb_set.iter().any(|v| FRIEND_EXCLUSIONS.contains(&v)))
        //     .map(|(i, j)| (i.clone(), j.clone()))
        //     .collect();
        let mut output: HashMap<Uuid, HashSet<RelationVerb>> = HashMap::new();
        for (verb, ids) in r {
            if FRIEND_EXCLUSIONS.contains(verb) {
                for id in ids {
                    if !output.contains_key(id) {
                        output.insert(id.clone(), HashSet::new());
                    }
                    output.get_mut(id).unwrap().insert(verb.clone());
                }
            }
        }
        return output;
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
        for (verb, target_ids) in &mind_ref.relations {
            for target_id in target_ids {
                let mut to_remove: Option<RelationVerb> = None;
                let mut to_add: Option<RelationVerb> = None;

                if verb.eq(&RelationVerb::Acquaintance) {
                    if rng.gen::<f32>() < 0.6 {
                        to_remove = Some(RelationVerb::Acquaintance);
                    } else if rng.gen::<f32>() < 0.25 {
                        to_remove = Some(RelationVerb::Acquaintance);
                        to_add = Some(RelationVerb::Friend);
                    }
                } else if verb.eq(&RelationVerb::Friend) {
                    if rng.gen::<f32>() < 0.25 {
                        to_remove = Some(RelationVerb::Friend);
                        to_add = Some(RelationVerb::Acquaintance);
                    } else if rng.gen::<f32>() < 0.125 {
                        to_remove = Some(RelationVerb::Friend);
                        to_add = Some(RelationVerb::CloseFriend);
                    }
                } else if verb.eq(&RelationVerb::CloseFriend) {
                    if rng.gen::<f32>() < 0.125 {
                        to_remove = Some(RelationVerb::CloseFriend);
                        to_add = Some(RelationVerb::Friend);
                    }
                }

                if to_remove.is_some() {
                    let remove = to_remove.unwrap();
                    let mind = city.population.get_mut(&mind_ref.id).unwrap();
                    mind.relations.get_mut(&remove).unwrap().remove(target_id);
                    let target = city.population.get_mut(target_id).unwrap();
                    target
                        .relations
                        .get_mut(&remove)
                        .unwrap()
                        .remove(&mind_ref.id);
                }
                if to_add.is_some() {
                    let add = to_add.unwrap();
                    let mind = city.population.get_mut(&mind_ref.id).unwrap();
                    if !mind.relations.contains_key(&add) {
                        mind.relations.insert(add.clone(), HashSet::new());
                    }
                    mind.relations
                        .get_mut(&add)
                        .unwrap()
                        .insert(target_id.clone());
                    let target = city.population.get_mut(target_id).unwrap();
                    if !target.relations.contains_key(&add) {
                        target.relations.insert(add.clone(), HashSet::new());
                    }
                    target.relations.get_mut(&add).unwrap().insert(mind_ref.id);
                }
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
                let temp_exclude = filter_to_friend_exclusion_list(&mind_clone.relations);
                let mut excluded_relations: HashSet<&Uuid> =
                    HashSet::from_iter(temp_exclude.iter().map(|(id, _rest)| id));
                excluded_relations.extend(&current_friends);
                let friend_count = current_friends.len();
                let to_add =
                    ((rng.gen::<f32>() * 20.0) as i32 - friend_count as i32).max(0) as usize;

                let max_gap = if friend_count < 2 && (mind_clone.age > city.culture.adult_age + 6) {
                    15
                } else if friend_count < 5 && (mind_clone.age > city.culture.adult_age + 3) {
                    10
                } else {
                    5
                };

                let source_list = temp_generate_eligible_friend_list(
                    &age_cache,
                    (mind_clone.age as i32 - max_gap as i32).max(city.culture.adult_age as i32)
                        as u32,
                    mind_clone.age + max_gap,
                    &excluded_relations,
                );
                for _i in 0..to_add {
                    let index = (rng.gen::<f32>() * source_list.len() as f32) as usize;
                    let possible_target_mind_id = source_list.get(index);
                    if possible_target_mind_id.is_some() {
                        let target_mind_id = possible_target_mind_id.unwrap();
                        if !mind_clone.is_relation_of(target_mind_id) {
                            let source_mind_mut = city.population.get_mut(&m_id).unwrap();
                            if !source_mind_mut
                                .relations
                                .contains_key(&RelationVerb::Acquaintance)
                            {
                                source_mind_mut
                                    .relations
                                    .insert(RelationVerb::Acquaintance, HashSet::new());
                            }
                            source_mind_mut
                                .relations
                                .get_mut(&RelationVerb::Acquaintance)
                                .unwrap()
                                .insert(target_mind_id.clone());

                            let target_mind_mut = city.population.get_mut(&target_mind_id).unwrap();
                            if !target_mind_mut
                                .relations
                                .contains_key(&RelationVerb::Acquaintance)
                            {
                                target_mind_mut
                                    .relations
                                    .insert(RelationVerb::Acquaintance, HashSet::new());
                            }
                            target_mind_mut
                                .relations
                                .get_mut(&RelationVerb::Acquaintance)
                                .unwrap()
                                .insert(m_id.clone());
                        }
                    }
                }
            }
            for m in mind_ids {
                temp_friend_evolution(city, &m);
            }
            city.add_timestamp("temp find friends");
        }
    }
}
