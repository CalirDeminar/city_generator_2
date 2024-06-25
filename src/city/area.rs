pub mod area {
    use uuid::Uuid;

    #[derive(PartialEq, Debug, Clone)]
    pub struct Area {
        pub id: Uuid,
        pub name: String,
        pub size: usize,
    }
}
