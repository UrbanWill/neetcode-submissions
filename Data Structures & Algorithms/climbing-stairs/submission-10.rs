impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut one = 1;
        let mut two = 1;

        for _ in 0..(n -1) {
            let tmp = one;
            one = two;
            two = two + tmp;
        }

        two
    }
}
