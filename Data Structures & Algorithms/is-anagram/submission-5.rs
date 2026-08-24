use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map_s = HashMap::new();
        let mut map_t = HashMap::new();

        for (a, b) in s.chars().zip(t.chars()) {
            *map_s.entry(a).or_insert(0) += 1;
            *map_t.entry(b).or_insert(0) += 1;
        }

        map_s == map_t
    }
}
