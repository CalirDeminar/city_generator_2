pub mod relations {
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
}
