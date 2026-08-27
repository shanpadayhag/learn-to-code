// `.take()` moves the value OUT of an Option and leaves `None` behind, so the
// variable stays alive and usable. Prove it: take the value into a new variable,
// then show `slot` is now None but still a valid Option you can call .take() on again.

fn main() {
    let mut slot: Option<String> = Some(String::from("hi"));

    // 1. Use .take() to move the value into `taken`.
    // let taken = ...;

    // 2. Print both `taken` (Some("hi")) and `slot` (None), using {:?}.

    // 3. Call slot.take() a second time and print the result (None).
    // your code here
}
