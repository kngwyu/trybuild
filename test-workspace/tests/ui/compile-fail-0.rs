use std::rc::Rc;

struct NotSend {
    value: Rc<usize>,
}

impl test_crate::Sendable for NotSend {}

fn main() {}
