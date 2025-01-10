pub mod children {
    use std::collections::HashSet;

    use procgen_templater::dictionary::dictionary::Dictionary;
    use rand::{seq::SliceRandom, Rng};
    use uuid::Uuid;

    use crate::city::{
        city::City,
        population::mind::{
            mind::{random_mind, Gender},
            physical_description::physical_description::merge_descriptions,
            relations::relations::RelationVerb,
        },
    };

    const CHILD_MAX: usize = 5;
    const PARTNER_CHILD_ANNUAL_CHANCE: f32 = 0.05;
    const SPOUSE_CHILD_ANNUAL_CHANCE: f32 = 0.1;

    impl City {
        pub fn generate_children(self: &mut Self, dict: &Dictionary) {
            let population_ref = self.population.clone();
            let mut included: HashSet<&Uuid> = HashSet::new();
            let mut partners: HashSet<(&Uuid, &Uuid, RelationVerb)> = HashSet::new();
            for mind in population_ref.values() {
                if !included.contains(&mind.id) {
                    for (r_id, verbs) in &mind.relations {
                        if !included.contains(&r_id) {
                            if verbs.contains(&RelationVerb::Partner) {
                                included.insert(r_id);
                                included.insert(&mind.id);
                                partners.insert((&mind.id, r_id, RelationVerb::Partner));
                            }
                            if verbs.contains(&RelationVerb::Spouse) {
                                included.insert(r_id);
                                included.insert(&mind.id);
                                partners.insert((&mind.id, r_id, RelationVerb::Spouse));
                            }
                        }
                    }
                }
            }
            let mut rng = rand::thread_rng();
            for (m1_id, m2_id, relation) in partners {
                let mind_1 = population_ref.get(m1_id).unwrap();
                let mind_2 = population_ref.get(m2_id).unwrap();
                let child_total = mind_1
                    .relations
                    .iter()
                    .filter(|(_id, verbs)| verbs.contains(&RelationVerb::Child))
                    .count()
                    + mind_2
                        .relations
                        .iter()
                        .filter(|(_id, verbs)| verbs.contains(&RelationVerb::Child))
                        .count();
                let roll = rng.gen::<f32>();
                let roll_target = if relation.eq(&RelationVerb::Spouse) {
                    SPOUSE_CHILD_ANNUAL_CHANCE
                } else {
                    PARTNER_CHILD_ANNUAL_CHANCE
                };
                if child_total < CHILD_MAX
                    && mind_1.age < 50
                    && mind_2.age < 50
                    && roll < roll_target
                {
                    let mut child = random_mind(dict, &self.culture, self.year as i32);
                    child.year_of_birth = self.year as i32;
                    child.description =
                        merge_descriptions(&dict, &mind_1.description, &mind_2.description);
                    child.age = 0;

                    let mut surname_formats = self.culture.child_surname_formats.clone();
                    surname_formats.shuffle(&mut rng);
                    let formats = surname_formats.first().unwrap();
                    let format = if child.gender.eq(&Gender::Male) {
                        formats.0.clone()
                    } else if child.gender.eq(&Gender::Female) {
                        formats.1.clone()
                    } else if rng.gen::<f32>() < 0.5 {
                        formats.0.clone()
                    } else {
                        formats.1.clone()
                    };
                    child.last_name = format.render(
                        mind_1.first_name.clone(),
                        mind_1.last_name.clone(),
                        mind_2.first_name.clone(),
                        mind_2.last_name.clone(),
                    );

                    child.relations.insert(mind_1.id.clone(), HashSet::new());
                    child
                        .relations
                        .get_mut(&mind_1.id)
                        .unwrap()
                        .insert(RelationVerb::Parent);
                    child.relations.insert(mind_2.id.clone(), HashSet::new());
                    child
                        .relations
                        .get_mut(&mind_2.id)
                        .unwrap()
                        .insert(RelationVerb::Parent);

                    let mind_1_mut = self.population.get_mut(&mind_1.id).unwrap();
                    mind_1_mut
                        .relations
                        .insert(child.id.clone(), HashSet::new());
                    mind_1_mut
                        .relations
                        .get_mut(&child.id)
                        .unwrap()
                        .insert(RelationVerb::Child);

                    let mind_2_mut = self.population.get_mut(&mind_2.id).unwrap();
                    mind_2_mut
                        .relations
                        .insert(child.id.clone(), HashSet::new());
                    mind_2_mut
                        .relations
                        .get_mut(&child.id)
                        .unwrap()
                        .insert(RelationVerb::Child);

                    let c_id = child.id.clone();
                    self.population.insert(child.id.clone(), child);
                    self.generate_family_relations(&c_id);
                }
            }
        }
    }
}
