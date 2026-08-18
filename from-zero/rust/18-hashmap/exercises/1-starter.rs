// A HashMap<K, V> stores key -> value pairs and finds a key almost instantly.
// The most common HashMap move is COUNTING: for each word, add one to its tally,
// starting at 0 the first time you ever see it. `entry(word).or_insert(0)` does
// exactly that — it hands you the counter slot for `word`, creating it at 0 if
// this is the word's first appearance. Then `*... += 1` bumps it.

use std::collections::HashMap;

fn main() {
    let sentence = "the cat sat on the mat";

    let mut counts = HashMap::new();
    for word in sentence.split(' ') {
        // Bump this word's counter. Create it at 0 if it's new, then add 1.
        // your code here
    }

    for (word, count) in &counts {
        println!("{word}: {count}");
    }
    // Expected (order will vary — a HashMap has no fixed order):
    //   the: 2
    //   cat: 1
    //   sat: 1
    //   on: 1
    //   mat: 1
}
