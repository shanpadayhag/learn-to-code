// `Send` = this value may MOVE to another thread. It is an auto trait: the
// compiler grants it to your struct only if every field has it. One `Rc` field
// takes it away from the whole struct.

use std::mem;
use std::rc::Rc;
use std::thread;

struct Report {
    title: String,
    lines: Rc<Vec<String>>,
}

fn main() {
    let shared_lines = Rc::new(vec![String::from("disk ok"), String::from("network ok")]);

    let report = Report {
        title: String::from("worker 1"),
        lines: Rc::clone(&shared_lines),
    };

    // 1. Uncomment this spawn and compile. Read the error all the way down —
    //    it names the trait (`Send`), the field's type (`Rc<Vec<String>>`),
    //    and the line "required because it appears within the type `Report`".
    //    That last line is the auto-trait rule talking.
    // let handle = thread::spawn(move || {
    //     println!("{}: {} lines", report.title, report.lines.len());
    // });
    // handle.join().unwrap();

    // 2. Fix it: change the field's type to Arc<Vec<String>>, build the value
    //    with Arc::new, and clone with Arc::clone. Nothing else changes.

    // 3. Then spawn TWO threads, each with its own Report holding its own
    //    Arc handle to the same lines, and join them both.

    // 4. Print the size of both pointer types. The fix changed which traits
    //    the type has — not what it looks like in memory.
    println!("Rc<Vec<String>>  is {} bytes", mem::size_of::<Rc<Vec<String>>>());
}
