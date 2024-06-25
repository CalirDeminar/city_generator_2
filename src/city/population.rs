pub mod mind;
pub mod population {
    use std::collections::HashMap;

    use uuid::Uuid;

    use super::mind::mind::Mind;

    pub type Population = HashMap<Uuid, Mind>;
}
