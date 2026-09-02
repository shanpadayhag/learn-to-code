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
        first_number_head: Option<Box<ListNode>>,
        second_number_head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut total_list_head = Box::new(ListNode::new(0));
        let mut total_list_tail = &mut total_list_head;
        let mut carried_digit_value = 0;

        let mut remaining_first_digits = first_number_head;
        let mut remaining_second_digits = second_number_head;

        while remaining_first_digits.is_some()
            || remaining_second_digits.is_some()
            || carried_digit_value != 0
        {
            let mut current_digit_sum = carried_digit_value;

            if let Some(first_digit_node) = remaining_first_digits.take() {
                current_digit_sum += first_digit_node.val;
                remaining_first_digits = first_digit_node.next;
            }
            if let Some(second_digit_node) = remaining_second_digits.take() {
                current_digit_sum += second_digit_node.val;
                remaining_second_digits = second_digit_node.next;
            }

            carried_digit_value = current_digit_sum / 10;
            total_list_tail.next = Some(Box::new(ListNode::new(current_digit_sum % 10)));
            total_list_tail = total_list_tail.next.as_mut().unwrap();
        }

        total_list_head.next
    }
}

fn main() {
    check(vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]);
    check(vec![0], vec![0], vec![0]);
    check(vec![9, 9, 9, 9, 9, 9, 9], vec![9, 9, 9, 9], vec![8, 9, 9, 9, 0, 0, 0, 1]);
    check(vec![5], vec![5], vec![0, 1]);
    check(vec![1], vec![9, 9, 9], vec![0, 0, 0, 1]);
}

fn check(
    first_number_digits: Vec<i32>,
    second_number_digits: Vec<i32>,
    expected_total_digits: Vec<i32>,
) {
    let total_list_head = Solution::add_two_numbers(
        build_digit_list(&first_number_digits),
        build_digit_list(&second_number_digits),
    );
    let actual_total_digits = collect_digit_values(total_list_head);
    assert_eq!(actual_total_digits, expected_total_digits);
    println!(
        "{:?} + {:?} = {:?}",
        first_number_digits, second_number_digits, actual_total_digits
    );
}

fn build_digit_list(digit_values: &[i32]) -> Option<Box<ListNode>> {
    digit_values
        .iter()
        .rev()
        .fold(None, |next_node, &digit_value| {
            Some(Box::new(ListNode { val: digit_value, next: next_node }))
        })
}

fn collect_digit_values(mut remaining_node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut digit_values = Vec::new();
    while let Some(current_node) = remaining_node {
        digit_values.push(current_node.val);
        remaining_node = current_node.next;
    }
    digit_values
}
