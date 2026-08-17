// The guess API is already defined for you.
// fn guess(num: i64) -> i32:
//     -1 if num is higher than the picked number
//      1 if num is lower than the picked number
//      0 if num is equal to the picked number

impl Solution {
    pub unsafe fn guess_number(n: i64) -> i64 {
        let mut l = 1;
        let mut r = n;

        loop {
            let m = (r + l) / 2;

            match guess(m) {
                1 => l = m + 1,
                -1 => r = m -1,
                _ => break m
            }
        }
    }
}
