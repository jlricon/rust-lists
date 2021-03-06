#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<'a, T> Node<T> {
    fn push_back(&mut self, val: T) {
        if let Some(mut next) = self.next.take() {
            next.push_back(val);
            self.next = Some(next);
        } else {
            self.next = Some(Box::new(Node { val, next: None }))
        }
    }
    fn push_front(&mut self, val: T) {
        let mut new_box = Node { val, next: None };
        std::mem::swap(self, &mut new_box);
        self.next = Some(Box::new(new_box));
    }
    fn len(&self) -> usize {
        if let Some(s) = &self.next {
            1 + s.len()
        } else {
            1
        }
    }
    fn tail(&'a self) -> &'a T {
        if let Some(next) = &self.next {
            next.tail()
        } else {
            &self.val
        }
    }
}
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}
impl<T: std::fmt::Debug> LinkedList<T> {
    pub fn print(&self) {
        print!("[");
        let mut node = if let Some(s) = &self.head {
            print!("{:?}", s.val);
            s
        } else {
            print!("]\n");
            return;
        };

        loop {
            if let Some(next_node) = &node.next {
                print!(" {:?}", next_node.val);
                node = &next_node;
            } else {
                break;
            }
        }
        print!("]\n");
    }
}
impl<'a, T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None }
    }

    pub fn push_back(&mut self, val: T) {
        if let Some(h) = &mut self.head {
            h.push_back(val);
        } else {
            self.head = Some(Box::new(Node { val, next: None }))
        }
    }
    pub fn push_front(&mut self, val: T) {
        if let Some(h) = &mut self.head {
            h.push_front(val);
        } else {
            self.head = Some(Box::new(Node { val, next: None }))
        }
    }
    pub fn len(&self) -> usize {
        if let Some(h) = &self.head {
            h.len()
        } else {
            0
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let mut current = &mut self.head;
        loop {
            match current {
                None => return None,
                Some(node) if node.next.is_none() => {
                    let taken = std::mem::take(current);
                    return Some(taken.unwrap().val);
                }
                Some(node) => {
                    current = &mut node.next;
                }
            }
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(mut head) = self.head.take() {
            std::mem::swap(&mut self.head, &mut head.next);
            Some(head.val)
        } else {
            None
        }
    }
    pub fn clear(&mut self) {
        self.head.take();
    }
    pub fn insert_after(&mut self, val: T, index: usize) {
        let mut current = &mut self.head;
        let mut position = 0;
        loop {
            match current {
                None => {
                    self.head = Some(Box::new(Node { val, next: None }));
                    break;
                }
                Some(node) if node.next.is_none() => {
                    node.next = Some(Box::new(Node { val, next: None }));
                    break;
                }
                Some(node) => {
                    if position >= index {
                        let rest = node.next.take();
                        node.next = Some(Box::new(Node { val, next: rest }));
                        break;
                    }
                    current = &mut node.next;
                    position += 1;
                }
            }
        }
    }
    pub fn head(&'a self) -> Option<&'a T> {
        if let Some(head) = &self.head {
            Some(&head.val)
        } else {
            None
        }
    }
    pub fn tail(&'a self) -> Option<&'a T> {
        if let Some(head) = &self.head {
            Some(&head.tail())
        } else {
            None
        }
    }
    pub fn iter(&'a self) -> IterableLinkedList<'a, T> {
        IterableLinkedList { inner: &self.head }
    }
}
impl<T> Into<Vec<T>> for LinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut res: Vec<T> = vec![];
        let mut curr = self.head;
        while let Some(node) = curr {
            res.push(node.val);
            curr = node.next;
        }
        res
    }
}
impl<T> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}
pub struct IterableLinkedList<'a, T> {
    inner: &'a Option<Box<Node<T>>>,
}
impl<'a, T> Iterator for IterableLinkedList<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner {
            None => None,
            Some(v) => {
                self.inner = &v.next;
                Some(&v.val)
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_all() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_front(0);
        assert_eq!(*list.head().unwrap(), 0);
        assert_eq!(*list.tail().unwrap(), 4);
        assert_eq!(list.len(), 5);
        let val = list.pop_front();
        assert_eq!(val.unwrap(), 0);
        list.clear();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let val = list.pop_front();
        assert_eq!(val.unwrap(), 1);
        list.push_back(1);
        let val = list.pop_back();
        assert_eq!(val.unwrap(), 1);
        list.clear();
        list.push_front(1);
        assert_eq!(list.pop_back().unwrap(), 1);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_front(0);

        assert_eq!(list.pop_back().unwrap(), 4);
        assert_eq!(list.pop_back().unwrap(), 3);
        assert_eq!(list.pop_front().unwrap(), 0);

        println!("Sum:{}", list.iter().sum::<i32>());
        list.clear();
        list.push_front(1);
        list.push_front(2);
        list.insert_after(3, 3);
        assert_eq!(list.last().unwrap(), 3);
    }
}
