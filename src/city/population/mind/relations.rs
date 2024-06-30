pub mod friends;
pub mod relations {
    use std::fmt;

    #[derive(PartialEq, Debug, Clone)]
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
                RelationVerb::Child => write!(f, "Child"),
                RelationVerb::Acquaintance => write!(f, "Acquaintance"),
                RelationVerb::Friend => write!(f, "Friend"),
                RelationVerb::CloseFriend => write!(f, "Close Friend"),
                _ => write!(f, ""),
            }
        }
    }
}
