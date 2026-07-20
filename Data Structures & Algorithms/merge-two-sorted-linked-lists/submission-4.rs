// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;

        let mut prehead = ListNode::new(-1);
        let mut cur = &mut prehead;

        while let (Some(node1), Some(node2)) = (&list1, &list2) {
            if node1.val < node2.val {
                cur.next = list1.take();
                cur = cur.next.as_mut().unwrap();
                list1 = cur.next.take();
            } else {
                cur.next = list2.take();
                cur = cur.next.as_mut().unwrap();
                list2 = cur.next.take();
            }
        }
        cur.next = list1.or(list2);

        prehead.next
        
    }
}
