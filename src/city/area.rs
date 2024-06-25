pub mod area {
    use uuid::Uuid;

    #[derive(PartialEq, Debug, Clone)]
    pub struct Area {
        pub id: Uuid,
        pub name: String,
        pub size: usize,
    }

    #[test]
    fn area_names() {
        use crate::city::{
            city::Era,
            culture::culture::{random_culture, rebalance_dict_for_culture},
        };
        use procgen_templater::dictionary::dictionary::build_dictionary_from_folder;
        let dict = build_dictionary_from_folder("./data_files");
        let culture = random_culture(&dict, Era::Medieval);
        let dict2 = rebalance_dict_for_culture(&culture, &dict);
        for _i in 0..100 {
            let t = dict2
                .get_random_template(vec![vec!["AreaName".to_string()]])
                .unwrap();
            println!("{}", dict2.render_template_as_title(&t.id).unwrap());
        }
    }
}
