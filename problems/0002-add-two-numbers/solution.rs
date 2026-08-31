struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        first_number: Option<Box<ListNode>>,
        second_number: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result_head = Box::new(ListNode::new(0));
        let mut result_tail = &mut result_head;
        let mut carry = 0;

        let mut first_digit = first_number;
        let mut second_digit = second_number;

        while first_digit.is_some() || second_digit.is_some() || carry != 0 {
            let mut digit_sum = carry;

            if let Some(node) = first_digit.take() {
                digit_sum += node.val;
                first_digit = node.next;
            }
            if let Some(node) = second_digit.take() {
                digit_sum += node.val;
                second_digit = node.next;
            }

            carry = digit_sum / 10;
            result_tail.next = Some(Box::new(ListNode::new(digit_sum % 10)));
            result_tail = result_tail.next.as_mut().unwrap();
        }

        result_head.next
    }
}

fn main() {
    check(vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]);
    check(vec![0], vec![0], vec![0]);
    check(vec![9, 9, 9, 9, 9, 9, 9], vec![9, 9, 9, 9], vec![8, 9, 9, 9, 0, 0, 0, 1]);
    check(vec![5], vec![5], vec![0, 1]);
    check(vec![1], vec![9, 9, 9], vec![0, 0, 0, 1]);
}

fn check(first_digits: Vec<i32>, second_digits: Vec<i32>, expected: Vec<i32>) {
    let total = Solution::add_two_numbers(build_list(&first_digits), build_list(&second_digits));
    let total_digits = collect_digits(total);
    assert_eq!(total_digits, expected);
    println!("{:?} + {:?} = {:?}", first_digits, second_digits, total_digits);
}

fn build_list(digits: &[i32]) -> Option<Box<ListNode>> {
    digits
        .iter()
        .rev()
        .fold(None, |next, &digit| Some(Box::new(ListNode { val: digit, next })))
}

fn collect_digits(mut node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut digits = Vec::new();
    while let Some(current) = node {
        digits.push(current.val);
        node = current.next;
    }
    digits
}
