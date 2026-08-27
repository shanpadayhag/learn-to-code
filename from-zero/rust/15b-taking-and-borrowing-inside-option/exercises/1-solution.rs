// `.take()` moves the value OUT of an Option and leaves `None` behind, so the
// variable stays alive and usable.

fn main() {
    let mut slot: Option<String> = Some(String::from("hi"));

    // 1. Move the value out; `slot` becomes None but is still a valid Option.
    let taken = slot.take();

    // 2. Both are usable: `taken` holds the value, `slot` is now None.
    println!("{taken:?}"); // Some("hi")
    println!("{slot:?}"); // None

    // 3. Taking again from a None just returns None — no crash, nothing to special-case.
    let again = slot.take();
    println!("{again:?}"); // None
}
