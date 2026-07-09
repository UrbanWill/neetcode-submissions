impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];

        for op in operations {
            match op.as_str() {
                "D" => {
                    let last = stack.last().unwrap();
                    stack.push(last * 2);
                }
                "+" => {
                    let len = stack.len();
                    stack.push(stack[len - 1] + stack[len - 2]);
                }
                "C" => {
                    stack.pop();
                }
                other => {
                    let other = other.parse::<i32>().unwrap();
                    stack.push(other);
                }
            }
        }
        stack.iter().sum()
    }
}
