struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let list = Node {
        val: 10,
        next: Some(Box::new(Node {
            val: 20,
            next: Some(Box::new(Node {
                val: 30,
                next: None,
            })),
        })),
    };

    let mut current = Some(&list);
    while let Some(node) = current {
        println!("{}", node.val);
        current = node.next.as_deref();
    }
}
