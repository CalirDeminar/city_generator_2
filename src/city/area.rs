pub mod area {
    use crate::city::city::City;
    use procgen_templater::dictionary::dictionary::Dictionary;
    use rand::Rng;
    use uuid::Uuid;

    pub type AreaId = Uuid;

    #[derive(PartialEq, Debug, Clone)]
    pub struct Area {
        pub id: AreaId,
        pub name: String,
        pub size: usize,
    }

    impl City {
        pub fn area_is_full(_area_id: AreaId) -> bool {
            return false;
        }
    }

    pub fn random_area(dict: &Dictionary) -> Area {
        let name_template = dict
            .get_random_template(vec![vec!["AreaName".to_string()]])
            .unwrap();
        let mut rand = rand::thread_rng();
        return Area {
            id: Uuid::new_v4() as AreaId,
            name: dict.render_template(&name_template.id).unwrap(),
            size: (rand.gen::<f32>() * 20.0) as usize,
        };
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
