// A search might find nothing — so its return type says so out loud:
// `Option<i32>` is EITHER `Some(the number)` OR `None` (nothing matched).
// (`&[i32]` is a slice — a borrowed view of a list. See Concept 12.)

fn first_even(numbers: &[i32]) -> Option<i32> {
    // Walk the numbers. As soon as you find an even one (number % 2 == 0),
    // return `Some(number)`. If the loop finishes with nothing found,
    // return `None`.
    // your code here
}

fn main() {
    let numbers = [7, 3, 8, 2];
    if let Some(found) = first_even(&numbers) {
        println!("first even: {found}");   // should print: first even: 8
    } else {
        println!("no even number");
    }

    let odds = [1, 5, 9];
    if let Some(found) = first_even(&odds) {
        println!("first even: {found}");
    } else {
        println!("no even number");         // should print: no even number
    }
}
