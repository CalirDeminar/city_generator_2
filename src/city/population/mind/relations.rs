pub mod children;
pub mod friends;
pub mod partners;
pub mod relations {
    use std::{collections::HashSet, fmt};

    use uuid::Uuid;

    use crate::city::city::City;

    #[derive(PartialEq, Debug, Clone, Eq, Hash)]
    pub enum RelationVerb {
        // family
        Parent,
        Child,
        AdoptedParent,
        AdoptedChild,
        Partner,
        ExPartner,
        LatePartner,
        Spouse,
        ExSpouse,
        LateSpouse,
        Sibling,
        Grandparent,
        Grandchild,
        Cousin,
        Pibling, // Aunt/Uncle
        Nibling, // Neice/Nephew
        // business
        Employer,
        Employee,
        Colleague,
        // social
        Acquaintance,
        Friend,
        CloseFriend,
        Grudge,
    }

    impl fmt::Display for RelationVerb {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                RelationVerb::Parent => write!(f, "Parent"),
                RelationVerb::Grandparent => write!(f, "Grandparent"),
                RelationVerb::Child => write!(f, "Child"),
                RelationVerb::Grandchild => write!(f, "Grandchild"),
                RelationVerb::Sibling => write!(f, "Sibling"),
                RelationVerb::Cousin => write!(f, "Cousin"),
                RelationVerb::Nibling => write!(f, "Nibling"),
                RelationVerb::Pibling => write!(f, "Pibling"),
                RelationVerb::Acquaintance => write!(f, "Acquaintance"),
                RelationVerb::Friend => write!(f, "Friend"),
                RelationVerb::CloseFriend => write!(f, "Close Friend"),
                RelationVerb::Partner => write!(f, "Partner"),
                RelationVerb::LatePartner => write!(f, "Late-Partner"),
                RelationVerb::Spouse => write!(f, "Spouse"),
                RelationVerb::LateSpouse => write!(f, "Late-Spouse"),
                RelationVerb::ExPartner => write!(f, "Ex-Partner"),
                RelationVerb::ExSpouse => write!(f, "Ex-Spouse"),
                RelationVerb::AdoptedParent => write!(f, "Adopted-Parent"),
                RelationVerb::AdoptedChild => write!(f, "Adopted-Child"),
                RelationVerb::Employer => write!(f, "Employer"),
                RelationVerb::Employee => write!(f, "Employee"),
                RelationVerb::Colleague => write!(f, "Colleague"),
                RelationVerb::Grudge => write!(f, "Grudge"),
            }
        }
    }

    impl City {
        pub fn generate_family_relations(self: &mut Self, target_id: &Uuid) {
            let reference_population = self.population.clone();
            let target = reference_population.get(target_id).unwrap();
            let mut relations_to_add: HashSet<(Uuid, RelationVerb)> = HashSet::new();
            let parents = target.get_relations(RelationVerb::Parent);
            for p_id in parents {
                let parent = reference_population.get(&p_id).unwrap();
                // Siblings
                let siblings = parent.get_relations(RelationVerb::Child);
                for sibling_id in siblings {
                    if !sibling_id.eq(&target_id) {
                        let sibling = self.population.get_mut(&sibling_id).unwrap();
                        if !sibling.relations.contains_key(target_id) {
                            sibling.relations.insert(target_id.clone(), HashSet::new());
                        }
                        sibling
                            .relations
                            .get_mut(target_id)
                            .unwrap()
                            .insert(RelationVerb::Sibling);
                        relations_to_add.insert((sibling_id.clone(), RelationVerb::Sibling));
                    }
                }
                // Grandchildren
                let grandparents = parent.get_relations(RelationVerb::Parent);
                for grandparent_id in grandparents {
                    let grandparent = self.population.get_mut(&grandparent_id).unwrap();
                    if !grandparent.relations.contains_key(target_id) {
                        grandparent
                            .relations
                            .insert(target_id.clone(), HashSet::new());
                    }
                    grandparent
                        .relations
                        .get_mut(target_id)
                        .unwrap()
                        .insert(RelationVerb::Grandchild);
                    relations_to_add.insert((grandparent_id.clone(), RelationVerb::Grandparent));
                }
                // Piblings
                let piblings = parent.get_relations(RelationVerb::Sibling);
                for pibling_id in piblings {
                    let pibling = self.population.get_mut(&pibling_id).unwrap();
                    if !pibling.relations.contains_key(target_id) {
                        pibling.relations.insert(target_id.clone(), HashSet::new());
                    }
                    pibling
                        .relations
                        .get_mut(target_id)
                        .unwrap()
                        .insert(RelationVerb::Nibling);
                    relations_to_add.insert((pibling_id.clone(), RelationVerb::Pibling));
                }
                // Cousins
                let cousins = parent.get_relations(RelationVerb::Nibling);
                for cousin_id in cousins {
                    let cousin = self.population.get_mut(&cousin_id).unwrap();
                    if !cousin.relations.contains_key(target_id) {
                        cousin.relations.insert(target_id.clone(), HashSet::new());
                    }
                    cousin
                        .relations
                        .get_mut(target_id)
                        .unwrap()
                        .insert(RelationVerb::Cousin);
                    relations_to_add.insert((cousin_id.clone(), RelationVerb::Cousin));
                }
            }
            let target_mut = self.population.get_mut(target_id).unwrap();
            for (r_id, verb) in relations_to_add {
                if !target_mut.relations.contains_key(&r_id) {
                    target_mut.relations.insert(r_id.clone(), HashSet::new());
                }
                target_mut.relations.get_mut(&r_id).unwrap().insert(verb);
            }
        }
    }
}
