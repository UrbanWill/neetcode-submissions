impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut last = (m + n - 1) as usize;
        let mut m = m -1 ;
        let mut n = n -1;

        while n >= 0 {
            if m >= 0 && nums1[m as usize] > nums2[n as usize] {
                nums1[last] = nums1[m as usize];
                m -= 1;
            } else {
                nums1[last] = nums2[n as usize];
                n -= 1;
            }

            last -= 1;
        } 
    }
}
