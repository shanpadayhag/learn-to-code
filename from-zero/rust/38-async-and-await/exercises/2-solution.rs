// A future is a struct: a tag for how far the function got, plus the locals
// that have to survive the pause. So you can MEASURE a paused function.
//
// Run with:  rustc --edition 2024 2-solution.rs && ./2-solution

use std::mem::size_of_val;

async fn nothing() {}

// Each .await nests the inner future INSIDE the outer one, and adds one byte
// of state tag. Three levels of calls become one flat struct.
async fn level_a() -> u64 { nothing().await; 1 }
async fn level_b() -> u64 { level_a().await + 1 }
async fn level_c() -> u64 { level_b().await + 1 }

// `big` is dead before the pause, so it never becomes a field.
async fn scoped_then_await() -> u64 {
    let total = { let big = [0u8; 512]; big.len() as u64 };
    nothing().await;
    total
}

// `big` is used after the pause, so it must survive it — 512 bytes of field.
async fn held_across_await() -> u64 {
    let big = [0u8; 512];
    nothing().await;
    big.len() as u64
}

fn main() {
    println!("nothing()             {:>4} bytes", size_of_val(&nothing()));
    println!("level_a()             {:>4} bytes", size_of_val(&level_a()));
    println!("level_b()             {:>4} bytes", size_of_val(&level_b()));
    println!("level_c()             {:>4} bytes", size_of_val(&level_c()));
    println!("scoped_then_await()   {:>4} bytes", size_of_val(&scoped_then_await()));
    println!("held_across_await()   {:>4} bytes", size_of_val(&held_across_await()));
}

// nothing()                1 bytes
// level_a()                2 bytes
// level_b()                3 bytes
// level_c()                4 bytes
// scoped_then_await()     16 bytes
// held_across_await()    514 bytes
//
// The rule: what you hold across an .await is what your task costs.
// A thread costs a 2 MiB stack whether it is working or waiting. A paused
// task costs exactly the bytes above — which is why a program can hold a
// million of them.
