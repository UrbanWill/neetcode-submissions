impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count_s = HashMap::new();
        let mut count_t = HashMap::new();
        for (a, b) in s.chars().zip(t.chars()) {
            *count_s.entry(a).or_insert(0) += 1;
            *count_t.entry(b).or_insert(0) += 1;
        }

        count_s == count_t
    }
}
