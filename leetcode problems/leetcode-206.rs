// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    /*
        Version 1: without the use of take
        Take Aways:
        - destructure
            * used in assigning prev and curr kinda like unpacking a var
            * let Some() some is an enum/option choice like None
                + Used as in rust a var must have a type and not be None but with some the type is optional and None allowed
        - setting up a public class along with an implementation or class
    */
        let (mut prev, mut curr) = (None, head);
        while let Some(mut node) = curr {
            curr = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    
    /*
        Version 2: Using Take
        Take Aways:
        - take is used to move the value from the memory address its located at to the new location
            * Normally used to avoid partial moves and to keep the value in a valid state so that it can be returned

        let (mut prev, mut curr) = (None, head);
        while let Some(mut node) = curr{
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            curr = next;
        }
        prev
    */
    }
}