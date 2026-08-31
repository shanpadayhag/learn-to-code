// A future is a struct: a tag for how far the function got, plus the locals
// that have to survive the pause. So you can MEASURE a paused function.
//
// Run with:  rustc --edition 2024 2-starter.rs && ./2-starter

use std::mem::size_of_val;

async fn nothing() {}

// 1. Build a chain, each one awaiting the previous.
// async fn level_a() -> u64 { nothing().await; 1 }
// async fn level_b() -> u64 { level_a().await + 1 }
// async fn level_c() -> u64 { level_b().await + 1 }

// 2. Two functions doing the SAME work with a 512-byte array. In the first,
//    the array dies before the pause; in the second, it is used after it.
//
// async fn scoped_then_await() -> u64 {
//     let total = { let big = [0u8; 512]; big.len() as u64 };
//     nothing().await;
//     total
// }
//
// async fn held_across_await() -> u64 {
//     let big = [0u8; 512];
//     nothing().await;
//     big.len() as u64
// }

fn main() {
    // 3. PREDICT before you compile. Write your guesses down:
    //
    //      nothing()             ? bytes
    //      level_a()             ? bytes
    //      level_b()             ? bytes
    //      level_c()             ? bytes
    //      scoped_then_await()   ? bytes
    //      held_across_await()   ? bytes

    // 4. Now print them all. size_of_val takes a reference, and a call
    //    expression is enough to make one:
    //    println!("{:>5}", size_of_val(&level_a()));

    // 5. The chain should walk up by exactly one byte per layer. Work out what
    //    that one byte is, and why the layers nest rather than stack.

    // 6. The last pair is the real lesson. Same array, same arithmetic, and a
    //    ~500 byte difference. Name the rule in one sentence before moving on.
}
