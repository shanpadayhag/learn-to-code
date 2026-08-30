// Send and Sync are ordinary trait bounds, so you can interrogate the compiler
// about any type without running a thread at all. The bound IS the test.
//
// Send = may MOVE to another thread.  Sync = may be SHARED (&T) with another.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard};

// 1. Write the two probes. Empty bodies — the bound does all the work.
//    fn assert_send<T: Send>(label: &str) { println!("{:<22} Send ✅", label); }
//    fn assert_sync<T: Sync>(label: &str) { ... }

fn main() {
    // 2. Before compiling, predict ✅ or ❌ for each of these twelve:
    //
    //      i32              Send ?   Sync ?
    //      String           Send ?   Sync ?
    //      Rc<i32>          Send ?   Sync ?
    //      RefCell<i32>     Send ?   Sync ?
    //      Arc<i32>         Send ?   Sync ?
    //      Mutex<i32>       Send ?   Sync ?
    //      MutexGuard<i32>  Send ?   Sync ?
    //
    // 3. Now call the probes for every one that you think passes, e.g.
    //    assert_send::<i32>("i32");
    //    Turbofish (::<T>) is how you name the type when no argument carries it.

    // 4. For the ones you think fail, write the call and leave it COMMENTED,
    //    with the compiler's own wording next to it. Uncomment one at a time to
    //    check yourself. The two failure messages differ by a single word —
    //    "cannot be sent" vs "cannot be shared" — and that word is the trait.

    // 5. Two rows are the interesting ones. RefCell splits the pair (it can
    //    travel, it cannot be shared). MutexGuard splits it the OTHER way.
    //    Work out why a lock receipt must not change threads.
}
