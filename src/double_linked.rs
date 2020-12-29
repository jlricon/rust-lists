use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node<T> {
    // Has to be RefCell to avoid copying the type
    // Cell needs the inner type to be Copy and Node is not copy
    next: Option<Rc<RefCell<Node<T>>>>,
    // Has to be Weak to avoid circular Rcs which would lead to memory leaks
    prev: Option<Weak<RefCell<Node<T>>>>,
    val: T,
}
pub fn gen() {
    // ble->bla
    let bla = Node {
        next: None,
        prev: None,
        val: 2,
    };
    let ref_a = Rc::new(RefCell::new(bla));
    let ble = Node {
        next: Some(Rc::clone(&ref_a)),
        prev: None,
        val: 1,
    };
    let ref_b = Rc::new(RefCell::new(ble));
    ref_a.borrow_mut().prev = Some(Rc::downgrade(&ref_b));
}
