pub mod surnames {
    use rand::seq::SliceRandom;
    use rand::Rng;

    #[derive(PartialEq, Debug, Clone)]
    pub struct SurnameFormat {
        pre: String,
        mind_1_first_present: bool,
        mind_1_last_present: bool,
        between: String,
        mind_2_first_present: bool,
        mind_2_last_present: bool,
        post: String,
    }

    impl SurnameFormat {
        pub fn render(
            self: &Self,
            mind_1_first: String,
            mind_1_last: String,
            mind_2_first: String,
            mind_2_last: String,
        ) -> String {
            let mut output = String::new();
            output.push_str(&self.pre);
            if self.mind_1_first_present {
                output.push_str(&mind_1_first);
            }
            if self.mind_1_last_present {
                output.push_str(&mind_1_last);
            }
            output.push_str(&self.between);
            if self.mind_2_first_present {
                output.push_str(&mind_2_first);
            }
            if self.mind_2_last_present {
                output.push_str(&mind_2_last);
            }
            output.push_str(&self.post);
            return output;
        }
    }

    pub fn random_marriage_surname_formats() -> Vec<(SurnameFormat, SurnameFormat)> {
        let mut output: Vec<(SurnameFormat, SurnameFormat)> = Vec::new();
        let mut list = vec![
            // name1-name2/name2-name1
            (
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: false,
                    mind_1_last_present: true,
                    between: String::from("-"),
                    mind_2_first_present: false,
                    mind_2_last_present: true,
                    post: String::new(),
                },
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: false,
                    mind_1_last_present: true,
                    between: String::from("-"),
                    mind_2_first_present: false,
                    mind_2_last_present: true,
                    post: String::new(),
                },
            ),
            // keep mans/womans
            (
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: false,
                    mind_1_last_present: true,
                    between: String::new(),
                    mind_2_first_present: false,
                    mind_2_last_present: false,
                    post: String::new(),
                },
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: false,
                    mind_1_last_present: true,
                    between: String::new(),
                    mind_2_first_present: false,
                    mind_2_last_present: false,
                    post: String::new(),
                },
            ),
            (
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: false,
                    mind_1_last_present: false,
                    between: String::new(),
                    mind_2_first_present: false,
                    mind_2_last_present: true,
                    post: String::new(),
                },
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: false,
                    mind_1_last_present: false,
                    between: String::new(),
                    mind_2_first_present: false,
                    mind_2_last_present: true,
                    post: String::new(),
                },
            ),
            // keey old name
            (
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: false,
                    mind_1_last_present: true,
                    between: String::new(),
                    mind_2_first_present: false,
                    mind_2_last_present: false,
                    post: String::new(),
                },
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: false,
                    mind_1_last_present: false,
                    between: String::new(),
                    mind_2_first_present: false,
                    mind_2_last_present: true,
                    post: String::new(),
                },
            ),
        ];
        let mut rng = rand::thread_rng();
        list.shuffle(&mut rng);
        for i in 0..if rng.gen::<f32>() < 0.5 { 2 } else { 3 } {
            output.push(list.get(i).unwrap().clone());
        }
        return output;
    }

    pub fn random_child_surname_formats() -> Vec<(SurnameFormat, SurnameFormat)> {
        let mut output: Vec<(SurnameFormat, SurnameFormat)> = Vec::new();
        let mut list = vec![
            // father
            (
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: false,
                    mind_1_last_present: true,
                    between: String::new(),
                    mind_2_first_present: false,
                    mind_2_last_present: false,
                    post: String::new(),
                },
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: false,
                    mind_1_last_present: true,
                    between: String::new(),
                    mind_2_first_present: false,
                    mind_2_last_present: false,
                    post: String::new(),
                },
            ),
            // mother
            (
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: false,
                    mind_1_last_present: false,
                    between: String::new(),
                    mind_2_first_present: false,
                    mind_2_last_present: true,
                    post: String::new(),
                },
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: false,
                    mind_1_last_present: false,
                    between: String::new(),
                    mind_2_first_present: false,
                    mind_2_last_present: true,
                    post: String::new(),
                },
            ),
            // sen/son
            (
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: true,
                    mind_1_last_present: false,
                    between: String::new(),
                    mind_2_first_present: false,
                    mind_2_last_present: false,
                    post: String::from("son"),
                },
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: true,
                    mind_1_last_present: false,
                    between: String::new(),
                    mind_2_first_present: false,
                    mind_2_last_present: false,
                    post: String::from("sen"),
                },
            ),
            (
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: true,
                    mind_1_last_present: false,
                    between: String::new(),
                    mind_2_first_present: false,
                    mind_2_last_present: false,
                    post: String::from("son"),
                },
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: false,
                    mind_1_last_present: false,
                    between: String::new(),
                    mind_2_first_present: true,
                    mind_2_last_present: false,
                    post: String::from("sen"),
                },
            ),
            (
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: false,
                    mind_1_last_present: false,
                    between: String::new(),
                    mind_2_first_present: true,
                    mind_2_last_present: false,
                    post: String::from("son"),
                },
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: false,
                    mind_1_last_present: false,
                    between: String::new(),
                    mind_2_first_present: true,
                    mind_2_last_present: false,
                    post: String::from("sen"),
                },
            ),
            //son/dotter
            (
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: true,
                    mind_1_last_present: false,
                    between: String::new(),
                    mind_2_first_present: false,
                    mind_2_last_present: false,
                    post: String::from("son"),
                },
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: true,
                    mind_1_last_present: false,
                    between: String::new(),
                    mind_2_first_present: false,
                    mind_2_last_present: false,
                    post: String::from("dotter"),
                },
            ),
            (
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: true,
                    mind_1_last_present: false,
                    between: String::new(),
                    mind_2_first_present: false,
                    mind_2_last_present: false,
                    post: String::from("son"),
                },
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: false,
                    mind_1_last_present: false,
                    between: String::new(),
                    mind_2_first_present: true,
                    mind_2_last_present: false,
                    post: String::from("dotter"),
                },
            ),
            (
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: false,
                    mind_1_last_present: false,
                    between: String::new(),
                    mind_2_first_present: true,
                    mind_2_last_present: false,
                    post: String::from("son"),
                },
                SurnameFormat {
                    pre: String::new(),
                    mind_1_first_present: false,
                    mind_1_last_present: false,
                    between: String::new(),
                    mind_2_first_present: true,
                    mind_2_last_present: false,
                    post: String::from("dotter"),
                },
            ),
        ];
        let mut rng = rand::thread_rng();
        list.shuffle(&mut rng);
        for i in 0..if rng.gen::<f32>() < 0.5 { 2 } else { 3 } {
            output.push(list.get(i).unwrap().clone());
        }
        return output;
    }

    #[test]
    fn test_surname_formatter() {
        let t_format = SurnameFormat {
            pre: String::new(),
            mind_1_first_present: true,
            mind_1_last_present: false,
            between: String::new(),
            mind_2_first_present: false,
            mind_2_last_present: false,
            post: String::from("sen"),
        };
        println!(
            "Formatter Test: {}",
            t_format.render(
                "Random".to_string(),
                "Surname".to_string(),
                "Random2".to_string(),
                "Surname2".to_string()
            )
        );
    }
    #[test]
    fn test_random_child_surname_formatter() {
        let t = random_child_surname_formats();
        let t_format = t.first().unwrap();
        println!(
            "Child Surname Test: {}",
            t_format.0.render(
                "Random".to_string(),
                "Surname".to_string(),
                "Random2".to_string(),
                "Surname2".to_string()
            )
        );
        println!(
            "Child Surname Test: {}",
            t_format.1.render(
                "Random".to_string(),
                "Surname".to_string(),
                "Random2".to_string(),
                "Surname2".to_string()
            )
        );
    }

    #[test]
    fn test_random_marriage_surname_formatter() {
        let t = random_marriage_surname_formats();
        let t_format = t.first().unwrap();
        println!(
            "Marriage Surname Test: {}",
            t_format.0.render(
                "Random".to_string(),
                "Surname".to_string(),
                "Random2".to_string(),
                "Surname2".to_string()
            )
        );
        println!(
            "Marriage Surname Test: {}",
            t_format.1.render(
                "Random".to_string(),
                "Surname".to_string(),
                "Random2".to_string(),
                "Surname2".to_string()
            )
        );
    }
}
