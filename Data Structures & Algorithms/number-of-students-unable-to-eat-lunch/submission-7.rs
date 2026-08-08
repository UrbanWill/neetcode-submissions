use std::collections::HashMap;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut res = sandwiches.len() as i32;

        let mut std = students.iter().fold(HashMap::new(), |mut map, s| {
            *map.entry(s).or_insert(0) += 1;
            map
        });

        for s in sandwiches.iter() {
            match std.get_mut(s) {
                Some(c) if *c > 0 => {
                    *c -= 1;
                    res -=1;
                },
                _ => {
                    return res
                }
            }
        }

        res
    }
}