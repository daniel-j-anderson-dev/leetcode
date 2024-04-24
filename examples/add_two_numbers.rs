/// You are given two **non-empty** linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list. <br>
/// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
/// <br> <br>
/// Example 1: <br>
/// <img src="https://assets.leetcode.com/uploads/2020/10/02/addtwonumber1.jpg">
/// <br>
/// ```
/// Input: l1 = [2,4,3], l2 = [5,6,4]
/// Output: [7,0,8]
/// Explanation: 342 + 465 = 807.
/// ```
/// <br>
/// Example 2:
///
/// ```
/// Input: l1 = [0], l2 = [0]
/// Output: [0]
/// ```
/// <br>
/// Example 3:
///
/// ```
/// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
/// Output: [8,9,9,9,0,0,0,1]
/// ```
/// <br>
///
/// Constraints:
/// - The number of nodes in each linked list is in the range [1, 100].
/// - 0 <= list_node.val <= 9
/// - It is guaranteed that the list represents a number that does not have leading zeros.

fn main() {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: usize,
    pub next: Option<Box<ListNode>>,
}

pub struct NodeIter(Option<Box<ListNode>>);
impl Iterator for NodeIter {
    type Item = Box<ListNode>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Some(ref mut next) => {
                let val = next.clone();
                self.0 = next.next.take();
                Some(val)
            }
            None => None,
        }
    }
}
fn list_to_vec(val: Box<ListNode>) -> Vec<usize> {
    list_into_iter(val).map(|node| node.val).collect::<Vec<_>>()
}
fn list_into_iter(val: Box<ListNode>) -> impl Iterator<Item = Box<ListNode>> {
    NodeIter(Some(val))
}
fn list_from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Box<ListNode> {
    let mut head = ListNode { val: 0, next: None };

    let mut current = &mut head;

    for val in iter {
        let new = Some(Box::new(ListNode { val, next: None }));
        current.next = new;
        current = current.next.as_mut().expect("current.next is some");
    }

    return head.next.take().unwrap_or_else(|| Box::new(head));
}
fn list_from_usize(value: usize) -> Box<ListNode> {
    list_from_iter(value.to_string().chars().rev().filter_map(|c| match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None,
    }))
}

fn add(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let l1 = list_to_vec(l1?);
    let l2 = list_to_vec(l2?);
    let (lhs, rhs) = {
        if l1.len() > l2.len() {
            (l1, l2)
        } else {
            (l2, l1)
        }
    };
    let mut sum = vec![0; lhs.len()];

    let mut carry = 0;

    let mut i = 0;

    loop {
        let digit_sum = lhs.get(i).unwrap_or(&0) + rhs.get(i).unwrap_or(&0) + carry;
        carry = 0;

        if digit_sum > 9 {
            match sum.get_mut(i) {
                Some(sum_i) => *sum_i = digit_sum - 10,
                None => sum.push(0),
            };
            carry = 1;

            i += 1;
        } else {
            match sum.get_mut(i) {
                Some(sum_i) => *sum_i = digit_sum,
                None if digit_sum != 0 => sum.push(digit_sum),
                _ => break,
            };

            i += 1;
        }
    }

    Some(list_from_iter(sum.into_iter()))
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn case1() {
        let lhs = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        let rhs = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let expected_sum = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        }));
        let sum = add(lhs, rhs);
        assert_eq!(sum, expected_sum);
    }
    #[test]
    pub fn case2() {
        let lhs = Some(Box::new(ListNode { val: 0, next: None }));
        let rhs = Some(Box::new(ListNode { val: 0, next: None }));
        let expected_sum = Some(Box::new(ListNode { val: 0, next: None }));
        let sum = add(lhs, rhs);
        assert_eq!(sum, expected_sum);
    }
    #[test]
    pub fn case3() {
        let lhs = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode { val: 9, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let rhs = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode { val: 9, next: None })),
                })),
            })),
        }));
        let expected_sum = Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 0,
                                    next: Some(Box::new(ListNode { val: 1, next: None })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let sum = add(lhs, rhs);
        assert_eq!(sum, expected_sum);
    }
}
