impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for c in s.chars() {
            match c {
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                _ => {
                    if stack.pop() != Some(c) {
                        return false;
                    }
                }
            }
        }

        stack.is_empty()
    }
}

