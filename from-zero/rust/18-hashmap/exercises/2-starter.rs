// Looking up a key that isn't in the map must NOT crash. `.get(&key)` returns an
// Option (Concept 15): Some(&value) if the key exists, None if it doesn't. So the
// "missing key" case is a value you handle with `match`, never a surprise crash.

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 88);
    scores.insert("Bob", 92);
    scores.insert("Dana", 75);

    for name in ["Alice", "Carol"] {
        // Use `scores.get(name)` and `match` the Option:
        //   Some(score) => print "{name}: {score}"
        //   None        => print "no score for {name}"
        // your code here
    }
    // Expected:
    //   Alice: 88
    //   no score for Carol
}
