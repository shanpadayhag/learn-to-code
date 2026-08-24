fn main() {
    let rows = [[1, 2], [3, 4], [5, 6]];

    'search: for row in rows {
        for number in row {
            if number > 3 {
                println!("found: {number}");
                break 'search;
            }
        }
    }
}
