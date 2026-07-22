impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for arr in matrix.iter() {
            let mut l = 0;
            let mut r = arr.len() as i32 - 1;

            while l <= r {
                let m = l + (r - l) / 2;
                
                if arr[m as usize] > target {
                    r = m - 1;
                } else if arr[m as usize] < target {
                    l = m + 1;
                } else {
                    return true;
                }
            }
        }

        return false;
    }
}
