// Send and Sync are ordinary trait bounds, so you can interrogate the compiler
// about any type without running a thread at all. The bound IS the test.
//
// Send = may MOVE to another thread.  Sync = may be SHARED (&T) with another.

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard};

// The bodies never test anything. If the call compiles, the type qualifies.
fn assert_send<T: Send>(label: &str) {
    println!("{:<16} Send ✅", label);
}

fn assert_sync<T: Sync>(label: &str) {
    println!("{:<16} Sync ✅", label);
}

fn main() {
    // Plain data: nothing to race on, so both.
    assert_send::<i32>("i32");
    assert_sync::<i32>("i32");
    assert_send::<String>("String");
    assert_sync::<String>("String");

    // Rc: non-atomic owner count, so NEITHER.
    // assert_send::<Rc<i32>>("Rc<i32>");
    //   error[E0277]: `Rc<i32>` cannot be sent between threads safely
    // assert_sync::<Rc<i32>>("Rc<i32>");
    //   error[E0277]: `Rc<i32>` cannot be shared between threads safely

    // RefCell: fine to move (one thread consults the borrow flag)...
    assert_send::<RefCell<i32>>("RefCell<i32>");
    // ...but not to share: two threads could both read "nobody's borrowing".
    // assert_sync::<RefCell<i32>>("RefCell<i32>");
    //   error[E0277]: `RefCell<i32>` cannot be shared between threads safely
    //   note: if you want aliasing and mutation between threads, use RwLock

    // Arc + Mutex: an atomic count and a real lock, so both — which is exactly
    // why Arc<Mutex<T>> is the pairing that works across threads.
    assert_send::<Arc<i32>>("Arc<i32>");
    assert_sync::<Arc<i32>>("Arc<i32>");
    assert_send::<Mutex<i32>>("Mutex<i32>");
    assert_sync::<Mutex<i32>>("Mutex<i32>");

    // MutexGuard splits the pair the other way round. Sharing a &guard only
    // lets you read through it — safe. But the guard is a receipt for a held
    // lock, and its drop is what unlocks; on many platforms the unlocking
    // thread must be the locking one. So it may be shared, never moved.
    assert_sync::<MutexGuard<'static, i32>>("MutexGuard<i32>");
    // assert_send::<MutexGuard<'static, i32>>("MutexGuard<i32>");
    //   error[E0277]: `MutexGuard<'static, i32>` cannot be sent between threads safely

    // None of it costs a byte: the traits live in the compiler, not the value.
    println!("Rc<i32>  is {} bytes", mem::size_of::<Rc<i32>>());
    println!("Arc<i32> is {} bytes", mem::size_of::<Arc<i32>>());
}
