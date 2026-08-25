// `.into_iter()` yields each item BY VALUE (owned T), consuming the collection.
// That owned String can be transformed and moved onward with no `.clone()`.

fn main() {
    let words = vec![String::from("a"), String::from("b"), String::from("c")];

    // Use .into_iter().map(...).collect() to uppercase each String into a
    // Vec<String>. `w.to_uppercase()` builds the new String.
    // let shouts: Vec<String> = ... ;
    // your code here

    // println!("{shouts:?}");   // should print: ["A", "B", "C"]
}
