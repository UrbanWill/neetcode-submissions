impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut len = m + n - 1;
        let mut m = m -1 as i32;
        let mut n = n -1 as i32;

        while n >= 0 {
            if m >= 0 && nums1[m as usize] > nums2[n as usize] {
                nums1[len as usize] = nums1[m as usize];
                m -= 1;
            } else {
                nums1[len as usize] = nums2[n as usize];
                n -= 1;
            }
            len -= 1;
        } 

    }
}
