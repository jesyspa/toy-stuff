use std::mem;

pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn is_empty(&self) -> bool {
        match self.head {
            Link::Empty => true,
            Link::More(_) => false,
        }
    }

    pub fn push(&mut self, x: i32) {
        let new_node = box Node {
            elem: x,
            next: mem::replace(&mut self.head, Link::Empty),
        };

        self.head = Link::More(new_node);
    }

    fn pop_node(&mut self) -> Link {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => Link::Empty,
            Link::More(mut node) => {
                self.head = mem::replace(&mut node.next, Link::Empty);
                Link::More(node)
            }
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.pop_node() {
            Link::Empty => None,
            Link::More(node) => Some(node.elem),
        }
    }
}

impl Default for List {
    fn default() -> Self {
        List::new()
    }
}

impl Drop for List {
    fn drop(&mut self) {
        while let Link::More(_) = self.pop_node() {}
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn new_is_empty() {
        let list = List::new();
        assert!(list.is_empty());
    }

    #[test]
    fn empty_pop_fails() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn push_is_not_empty() {
        let mut list = List::new();
        list.push(5);
        assert!(!list.is_empty());
    }

    #[test]
    fn pop_gives_last_value() {
        let mut list = List::new();
        list.push(5);
        assert_eq!(list.pop(), Some(5));

        let mut list = List::new();
        list.push(3);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
    }

    #[test]
    fn pop_gives_prev_values() {
        let mut list = List::new();
        list.push(1);
        list.push(3);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
