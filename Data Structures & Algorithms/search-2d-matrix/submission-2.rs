impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for row in matrix.iter() {
            let mut l = 0;
            let mut r = row.len() as i32 - 1;

            while l <= r {
                let m = (l + r) / 2;
                let cur = row[m as usize];

                if target > cur {
                    l = m + 1;
                } else if target < cur {
                    r = m - 1;
                } else {
                    return true;
                }
            }
        }
        
        false
    }
}
