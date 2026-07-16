use std::collections::HashMap;

impl Solution {
pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut res = students.len() as i32;

    let mut ctn = students.into_iter().fold(HashMap::new(), |mut map, stdn| {
        *map.entry(stdn).or_insert(0) += 1;
        map
    });

    for s in sandwiches.iter() {
        if let Some(count) = ctn.get_mut(s) {
            if *count > 0 {
                *count -= 1;
                res -= 1;
            } else {
                return res;
            }
        } else {
            return res;
        }
    }

    res
}

}