pub mod debub_timer {
    use std::time::Instant;

    #[derive(PartialEq, Debug, Clone)]
    pub struct DebugTimer {
        pub start: Instant,
        pub times_millis: Vec<(String, u128)>,
    }

    pub fn new_debug_timer() -> DebugTimer {
        return DebugTimer {
            start: Instant::now(),
            times_millis: vec![],
        };
    }

    impl DebugTimer {
        pub fn add_timestamp(self: &mut Self, name: &str) {
            let timestamp = self.start.elapsed();
            let matching = self.times_millis.iter_mut().find(|(n, _t)| n.eq(&name));
            if matching.is_some() {
                matching.unwrap().1 += timestamp.as_millis();
            } else {
                self.times_millis
                    .push((name.to_string(), timestamp.as_millis()));
            }
            self.start = Instant::now();
        }
        pub fn clear<'a>(self: &'a mut Self) {
            self.start = Instant::now();
            self.times_millis = vec![];
        }
        pub fn print(self: &Self) {
            println!("Timer Debug:");
            for (label, time) in &self.times_millis {
                println!("  {:<30} {time}ms", format!("{label}:"))
            }
        }
    }
}
