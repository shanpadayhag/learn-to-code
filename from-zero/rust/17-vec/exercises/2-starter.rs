// Indexing a Vec with `v[i]` CRASHES if `i` is out of range. The safe way is
// `.get(i)`, which returns an `Option` (Concept 15): `Some(&value)` if the index
// exists, `None` if it doesn't. So the missing case can't crash you by surprise.

fn main() {
    let scores = vec![88, 92, 79];

    for index in [1, 5] {
        // Use `scores.get(index)` and `match` the Option:
        //   Some(score) => print "score at {index}: {score}"
        //   None        => print "no score at index {index}"
        // your code here
    }
    // Expected:
    //   score at 1: 92
    //   no score at index 5
}
