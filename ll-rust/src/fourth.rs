use std::cell::{Ref, RefMut, RefCell};
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

fn deref_link<T>(link: Link<T>) -> Option<T> {
    link.map(|node| Rc::try_unwrap(node).ok().expect("node multiply referrenced").into_inner().elem)
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let new_node = Node::alloc(elem);
        match self.tail.take() {
            None => {
                self.head = Some(new_node.clone());
            }
            Some(ptr) => {
                // -1 to last node
                ptr.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(ptr);
            }
        }
        self.tail = Some(new_node);
    }

    pub fn push_front(&mut self, elem: T) {
        let new_node = Node::alloc(elem);
        match self.head.take() {
            None => {
                self.tail = Some(new_node.clone());
            }
            Some(ptr) => {
                ptr.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(ptr);
            }
        }
        self.head = Some(new_node);
    }

    fn pop_front_node(&mut self) -> Link<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                None => self.tail = None,
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
            }
            old_head
        })
    }

    pub fn pop_front(&mut self) -> Option<T> {
        deref_link(self.pop_front_node())
    }

    fn pop_back_node(&mut self) -> Link<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                None => self.head = None,
                Some(new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
            }
            old_tail
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        deref_link(self.pop_back_node())
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| Ref::map(node.borrow(), |r| &r.elem) )
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_mut().map(|node| RefMut::map(node.borrow_mut(), |r| &mut r.elem) )
    }
}

impl<T> Node<T> {
    fn alloc(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front_node().is_some() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> std::iter::IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn drop_nonempty() {
        let mut xs: List<i32> = List::new();
        for i in 0..5 {
            xs.push_back(i);
        }
        // check with valgrind that no memory was leaked
    }

    #[test]
    fn push_pop_front() {
        let mut xs: List<i32> = List::new();
        assert_eq!(xs.pop_front(), None);
        xs.push_front(3);
        xs.push_front(5);
        assert_eq!(xs.pop_front(), Some(5));
        assert_eq!(xs.pop_front(), Some(3));
        assert_eq!(xs.pop_front(), None);
    }

    #[test]
    fn push_back_pop_front() {
        let mut xs: List<i32> = List::new();
        assert_eq!(xs.pop_front(), None);
        xs.push_back(3);
        xs.push_back(5);
        assert_eq!(xs.pop_front(), Some(3));
        assert_eq!(xs.pop_front(), Some(5));
        assert_eq!(xs.pop_front(), None);
    }

    #[test]
    fn push_peek_front() {
        let mut xs: List<i32> = List::new();
        assert_eq!(xs.peek_front().as_deref(), None);
        xs.push_front(3);
        assert_eq!(xs.peek_front().as_deref(), Some(&3));
        xs.push_front(5);
        assert_eq!(xs.peek_front().as_deref(), Some(&5));
    }

    #[test]
    fn push_peek_mut_front() {
        let mut xs: List<i32> = List::new();
        assert_eq!(xs.peek_front_mut().as_deref(), None);
        xs.push_front(3);
        assert_eq!(xs.peek_front_mut().as_deref(), Some(&3));
        xs.push_front(5);
        assert_eq!(xs.peek_front_mut().as_deref(), Some(&5));
    }

    #[test]
    fn collect_into_iter() {
        let mut xs: List<i32> = List::new();
        xs.push_back(1);
        xs.push_back(3);
        xs.push_back(5);
        assert_eq!(xs.into_iter().collect::<Vec<_>>(), vec![1, 3, 5]);
    }

    #[test]
    fn double_ended_iterator() {
        let mut xs: List<i32> = List::new();
        for i in 0..5 {
            xs.push_back(i);
        }
        let mut iter = xs.into_iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(4));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}
