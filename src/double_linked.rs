// use std::{
//     cell::RefCell,
//     rc::{Rc, Weak},
// };

#[derive(Debug)]
// struct Node<T> {
//     // Has to be RefCell to avoid copying the type
//     // Cell needs the inner type to be Copy and Node is not copy
//     next: Option<Rc<RefCell<Node<T>>>>,
//     // Has to be Weak to avoid circular Rcs which would lead to memory leaks
//     prev: Option<Weak<RefCell<Node<T>>>>,
//     val: T,
// }
struct Node<T> {
    next: Option<Box<Node<T>>>,
    prev: Option<*mut Node<T>>,
    val: T,
}
impl<T> Node<T> {
    fn push_front(&mut self, val: T) {
        let self_ptr: *mut Node<T> = self;
        // Now point the second box (new_box) to the first one
        self.prev = Some(self_ptr);
        let mut new_box = Node {
            val,
            next: None,
            prev: None,
        };
        std::mem::swap(self, &mut new_box);
        self.next = Some(Box::new(new_box));
    }
}
pub fn gen() {
    // ble->bla
    let mut bla = Node {
        next: None,
        prev: None,
        val: 2,
    };

    bla.push_front(1);
    dbg!(&bla);
    // Access ble through bla
    let v = unsafe { (*bla.next.unwrap().prev.unwrap()).val };
    assert_eq!(v, 1);
}
