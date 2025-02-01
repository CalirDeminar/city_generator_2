pub mod partners {
    use std::{collections::HashSet, time::Instant};

    use rand::{seq::SliceRandom, Rng};
    use uuid::Uuid;

    use crate::city::{
        city::City,
        population::mind::{
            mind::{Gender, Mind, Sexuality},
            relations::relations::RelationVerb,
        },
    };

    // const PARTNER_CHANCE_GENERAL: f32 = 0.33; // multiple annual chances
    const PARTNER_MARRIAGE_RATE: f32 = 0.075; // single anunal chance
    const PARTNER_SPLIT_RATE: f32 = 0.06; // single annual chance
    const MARRIAGE_SPLIT_RATE: f32 = 0.03; // single annual chance

    impl City {
        pub fn update_mind_partner_relations(self: &mut Self) {
            let mut start = Instant::now();
            temp_find_partners(self);
            let find_partner_duration = start.elapsed().as_millis();
            start = Instant::now();
            temp_partner_evolution(self);
            let partner_evolution_duration = start.elapsed().as_millis();
            println!(
                "Partner Relations Durations: Find: {}ms - Evolution: {}ms",
                find_partner_duration, partner_evolution_duration
            );
        }
    }

    fn target_sexuality_genders(m: &Mind) -> HashSet<Gender> {
        let mut output: HashSet<Gender> = HashSet::new();
        if m.sexuality.eq(&Sexuality::Asexual) {
            return output;
        }
        output.insert(Gender::Ambiguous);
        if m.sexuality.eq(&Sexuality::Bisexual) || m.gender.eq(&Gender::Ambiguous) {
            output.insert(Gender::Male);
            output.insert(Gender::Female);
        };
        if m.sexuality.eq(&Sexuality::Hetrosexual) {
            if m.gender.eq(&Gender::Male) {
                output.insert(Gender::Female);
            } else {
                output.insert(Gender::Male);
            }
        }
        if m.sexuality.eq(&Sexuality::Homosexual) {
            if m.gender.eq(&Gender::Female) {
                output.insert(Gender::Female);
            } else {
                output.insert(Gender::Male);
            }
        }

        return output;
    }

    fn is_sexuality_compatible(a: &Mind, b: &Mind) -> bool {
        let a_target = target_sexuality_genders(a);
        let b_target = target_sexuality_genders(b);
        return a_target.contains(&b.gender) && b_target.contains(&a.gender);
    }

    pub fn temp_find_partners<'a>(city: &'a mut City) -> &'a mut City {
        let culture = city.culture.clone();
        // let mut rng = rand::thread_rng();
        let citizen_ids = city.current_citizens();
        let mut reference_citizens = city.population.clone();

        for id in &citizen_ids {
            let population = &mut city.population;
            let mind = population.get(id).unwrap().clone();

            if mind.is_single() && mind.age > city.culture.adult_age
            // && rng.gen::<f32>() < PARTNER_CHANCE_GENERAL
            {
                let mut single_friend_ids: Vec<&Uuid> = mind
                    .relations
                    .iter()
                    .map(|(verb, ids)| {
                        return if verb.eq(&&RelationVerb::Friend)
                            || verb.eq(&&RelationVerb::CloseFriend)
                            || verb.eq(&&RelationVerb::Acquaintance)
                        {
                            ids.iter()
                                .filter(|id| population.get(id).unwrap().is_single())
                                .collect()
                        } else {
                            Vec::new()
                        };
                    })
                    .flatten()
                    .collect();
                single_friend_ids.shuffle(&mut rand::thread_rng());
                let possible_target = single_friend_ids.iter().find(|f_id| {
                    let f = reference_citizens.get(f_id).unwrap();
                    return is_sexuality_compatible(&mind, f) && f.age > culture.adult_age;
                });
                if possible_target.is_some() {
                    let target_id = possible_target.unwrap();

                    let mind_mut = population.get_mut(&id).unwrap();
                    if mind_mut.relations.contains_key(&RelationVerb::Friend) {
                        mind_mut
                            .relations
                            .get_mut(&RelationVerb::Friend)
                            .unwrap()
                            .retain(|r_id| !r_id.eq(&target_id));
                    }
                    if mind_mut.relations.contains_key(&RelationVerb::CloseFriend) {
                        mind_mut
                            .relations
                            .get_mut(&RelationVerb::CloseFriend)
                            .unwrap()
                            .retain(|r_id| !r_id.eq(&target_id));
                    }

                    if !mind_mut.relations.contains_key(&RelationVerb::Partner) {
                        mind_mut
                            .relations
                            .insert(RelationVerb::Partner, HashSet::new());
                    }
                    mind_mut
                        .relations
                        .get_mut(&RelationVerb::Partner)
                        .unwrap()
                        .insert((*target_id).clone());

                    let target_mut = population.get_mut(&target_id).unwrap();
                    if target_mut.relations.contains_key(&RelationVerb::Friend) {
                        target_mut
                            .relations
                            .get_mut(&RelationVerb::Friend)
                            .unwrap()
                            .retain(|r_id| !r_id.eq(&id));
                    }
                    if target_mut
                        .relations
                        .contains_key(&RelationVerb::CloseFriend)
                    {
                        target_mut
                            .relations
                            .get_mut(&RelationVerb::CloseFriend)
                            .unwrap()
                            .retain(|r_id| !r_id.eq(&id));
                    }

                    if !target_mut.relations.contains_key(&RelationVerb::Partner) {
                        target_mut
                            .relations
                            .insert(RelationVerb::Partner, HashSet::new());
                    }
                    target_mut
                        .relations
                        .get_mut(&RelationVerb::Partner)
                        .unwrap()
                        .insert(id.clone());

                    reference_citizens = city.population.clone();
                }
            }
        }
        return city;
    }

    fn temp_partner_evolution<'a>(city: &'a mut City) -> &'a mut City {
        // TODO - improve perf. Major perf bottleneck
        let mut rng = rand::thread_rng();
        let citizen_ids = city.current_citizens();
        let mut reference_citizens = city.population.clone();

        let mut processed: HashSet<Uuid> = HashSet::new();
        for id in citizen_ids {
            let mind = reference_citizens.get(&id).unwrap();
            if !processed.contains(&id) && !mind.is_single() {
                processed.insert(id.clone());

                let possible_p = mind.relations.get(&RelationVerb::Partner);
                let possible_s = mind.relations.get(&RelationVerb::Spouse);
                let possible_ps = possible_p.or(possible_s);

                let ids: Vec<&Uuid> = possible_ps.unwrap().iter().collect::<Vec<&Uuid>>().clone();
                let possible_partner: Option<(&&Uuid, &RelationVerb)> = if possible_ps.is_some() {
                    if ids.len() > 0 {
                        let first_id = ids.first();
                        Some((
                            first_id.unwrap(),
                            if possible_p.is_some() {
                                &RelationVerb::Partner
                            } else {
                                &RelationVerb::Spouse
                            },
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                };

                if possible_partner.is_some() {
                    let (partner_id, verb) = possible_partner.unwrap();
                    let partner = reference_citizens.get(partner_id).unwrap();
                    if partner.alive {
                        processed.insert((*partner_id).clone());
                        let split_chance = if verb.eq(&RelationVerb::Spouse) {
                            MARRIAGE_SPLIT_RATE
                        } else {
                            PARTNER_SPLIT_RATE
                        };
                        let mut new_verb: Option<RelationVerb> = if verb.eq(&RelationVerb::Partner)
                        {
                            Some(RelationVerb::ExPartner)
                        } else {
                            Some(RelationVerb::ExSpouse)
                        };
                        if rng.gen::<f32>() > split_chance {
                            new_verb = Some(verb.clone());
                        }
                        if verb.eq(&RelationVerb::Partner)
                            && rng.gen::<f32>() < PARTNER_MARRIAGE_RATE
                        {
                            new_verb = Some(RelationVerb::Spouse);
                        }
                        let mind_mut = city.population.get_mut(&id).unwrap();
                        if new_verb.is_some() {
                            let partner = reference_citizens.get(partner_id).unwrap();

                            let (new_mind_last_name, new_partner_last_name) =
                                if new_verb.eq(&Some(RelationVerb::Spouse)) {
                                    let mut surname_formats =
                                        city.culture.marriage_surname_formats.clone();
                                    surname_formats.shuffle(&mut rng);
                                    let surname_format = surname_formats.first().unwrap();
                                    (
                                        Some(surname_format.0.render(
                                            mind.first_name.clone(),
                                            mind.last_name.clone(),
                                            partner.first_name.clone(),
                                            partner.last_name.clone(),
                                        )),
                                        Some(surname_format.1.render(
                                            mind.first_name.clone(),
                                            mind.last_name.clone(),
                                            partner.first_name.clone(),
                                            partner.last_name.clone(),
                                        )),
                                    )
                                } else {
                                    (None, None)
                                };

                            if !mind_mut.relations.contains_key(&verb) {
                                mind_mut.relations.insert(verb.clone(), HashSet::new());
                            }

                            mind_mut
                                .relations
                                .get_mut(&verb)
                                .unwrap()
                                .retain(|rid| !rid.eq(&partner_id));
                            mind_mut
                                .relations
                                .get_mut(&verb)
                                .unwrap()
                                .insert((*partner_id).clone());

                            if new_verb.eq(&Some(RelationVerb::Spouse)) {
                                mind_mut.last_name = new_mind_last_name.unwrap();
                            } else if new_verb.eq(&Some(RelationVerb::ExSpouse)) {
                                mind_mut.last_name = mind_mut.origional_last_name.clone();
                            }
                            let partner_mut = city.population.get_mut(&partner_id).unwrap();
                            if !partner_mut.relations.contains_key(&verb) {
                                partner_mut.relations.insert(verb.clone(), HashSet::new());
                            }
                            partner_mut
                                .relations
                                .get_mut(&verb)
                                .unwrap()
                                .retain(|rid| !rid.eq(&id));
                            if new_verb.is_some() {
                                let new_verb_unwrapped = new_verb.clone().unwrap();
                                if !partner_mut.relations.contains_key(&new_verb_unwrapped) {
                                    partner_mut
                                        .relations
                                        .insert(new_verb_unwrapped.clone(), HashSet::new());
                                }
                                partner_mut
                                    .relations
                                    .get_mut(&new_verb_unwrapped)
                                    .unwrap()
                                    .insert(id.clone());
                            }
                            if new_verb.eq(&Some(RelationVerb::Spouse)) {
                                partner_mut.last_name = new_partner_last_name.unwrap();
                            } else if new_verb.eq(&Some(RelationVerb::ExSpouse)) {
                                partner_mut.last_name = partner_mut.origional_last_name.clone();
                            }
                        }
                    }
                }
            }
            reference_citizens = city.population.clone();
        }

        return city;
    }

    #[test]
    fn test_matching() {
        use crate::city::{
            city::Era,
            culture::culture::random_culture,
            population::mind::mind::{random_mind, Gender, Sexuality},
        };
        use procgen_templater::dictionary::dictionary::build_dictionary_from_folder;

        let dict = build_dictionary_from_folder("./data_files/");
        let culture = random_culture(&dict, Era::Medieval);
        let mut mind = random_mind(&dict, &culture, 0);
        mind.gender = Gender::Male;
        mind.sexuality = Sexuality::Hetrosexual;
        println!(
            "Straight Male Targets: {:#?}",
            target_sexuality_genders(&mind)
        );
        mind.gender = Gender::Female;
        mind.sexuality = Sexuality::Hetrosexual;
        println!(
            "Straight Female Targets: {:#?}",
            target_sexuality_genders(&mind)
        );
    }
}
