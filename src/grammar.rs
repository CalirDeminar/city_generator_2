pub mod grammar {
    pub fn is_vowel(c: &char) -> bool {
        let vowels = vec!['a', 'e', 'i', 'o', 'u'];
        return vowels.iter().any(|v| c.to_ascii_lowercase().eq(v));
    }

    pub fn render_list(l: Vec<&str>) -> String {
        if l.len().eq(&0) {
            return "".to_string();
        }
        if l.len().eq(&1) {
            return l.first().unwrap().to_string();
        }
        let mut main_body = l.clone();
        let tail = main_body.split_off(l.len() - 1);
        return format!("{} and {}", main_body.join(", "), tail.first().unwrap()).to_string();
    }

    pub fn a_or_an(next: &str) -> String {
        if is_vowel(&next.chars().next().unwrap()) {
            return "an".to_string();
        } else {
            return "a".to_string();
        }
    }

    #[test]
    fn test_list_rendering() {
        assert!(
            render_list(vec!["Element1", "Element2", "Element3", "Element4"])
                .eq("Element1, Element2, Element3 and Element4")
        );
        assert!(render_list(vec!["Element1", "Element2"]).eq("Element1 and Element2"));
        assert!(render_list(vec!["Element1"]).eq("Element1"));
    }
}
