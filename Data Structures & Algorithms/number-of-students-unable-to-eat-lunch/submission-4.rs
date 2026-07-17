impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut res = students.len() as i32;

        let mut ctn = students.iter().fold(HashMap::new(), |mut map, stdn| {
            *map.entry(stdn).or_insert(0) += 1;
            map
        });

        for s in sandwiches.iter() {
            match ctn.get_mut(s) {
                Some(c) if *c > 0 => {
                    *c -= 1;
                    res -= 1;
                }
                _ => return res,
            }
        }

        res
    }
}