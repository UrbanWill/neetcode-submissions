impl Solution {
        pub fn is_valid(s: String) -> bool {
            let close_to_open = HashMap::from([('}', '{'), (')', '('), (']', '[')]);
            let mut stack = vec![];

            for c in s.chars() {
                if close_to_open.contains_key(&c) {
                    if stack.len() > 0 && close_to_open.get(&c) == stack.last() {
                        stack.pop();
                    } else {
                        return false;
                    }
                } else {
                    stack.push(c);
                }
            }

            stack.is_empty()
        }
}
