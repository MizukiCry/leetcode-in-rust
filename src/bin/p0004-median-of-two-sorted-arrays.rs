fn find_kth(nums1: &[i32], nums2: &[i32], k: usize) -> i32 {
    if nums1.is_empty() {
        nums2[k - 1]
    } else if nums2.is_empty() {
        nums1[k - 1]
    } else if k == 1 {
        nums1[0].min(nums2[0])
    } else {
        let p1 = (k / 2 - 1).min(nums1.len() - 1);
        let p2 = (k / 2 - 1).min(nums2.len() - 1);
        if nums1[p1] <= nums2[p2] {
            find_kth(&nums1[p1 + 1..], nums2, k - p1 - 1)
        } else {
            find_kth(nums1, &nums2[p2 + 1..], k - p2 - 1)
        }
    }
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len = nums1.len() + nums2.len();
        if len & 1 == 0 {
            (find_kth(&nums1[..], &nums2[..], len / 2) + find_kth(&nums1[..], &nums2[..], len / 2 + 1)) as f64 / 2.0
        } else {
            find_kth(&nums1[..], &nums2[..], len / 2 + 1) as f64
        }
    }
}