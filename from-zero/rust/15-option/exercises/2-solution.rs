fn first_even(numbers: &[i32]) -> Option<i32> {
    for &number in numbers {
        if number % 2 == 0 {
            return Some(number);
        }
    }
    None
}

fn main() {
    let numbers = [7, 3, 8, 2];
    if let Some(found) = first_even(&numbers) {
        println!("first even: {found}");
    } else {
        println!("no even number");
    }

    let odds = [1, 5, 9];
    if let Some(found) = first_even(&odds) {
        println!("first even: {found}");
    } else {
        println!("no even number");
    }
}
