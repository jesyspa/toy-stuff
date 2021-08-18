use std::iter;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push(&mut self, x: T) {
        let new_node = box Node {
            elem: x,
            next: self.head.take(),
        };

        self.head = Some(new_node);
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    fn pop_node(&mut self) -> Link<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node
        })
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_node().map(|node| node.elem)
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List::new()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_node().is_some() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> iter::IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn new_is_empty() {
        let list: List<i32> = List::new();
        assert!(list.is_empty());
    }

    #[test]
    fn empty_pop_gives_none() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn push_is_not_empty() {
        let mut list: List<i32> = List::new();
        list.push(5);
        assert!(!list.is_empty());
    }

    #[test]
    fn pop_gives_last_value() {
        let mut list: List<i32> = List::new();
        list.push(5);
        assert_eq!(list.pop(), Some(5));

        let mut list: List<i32> = List::new();
        list.push(3);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
    }

    #[test]
    fn pop_gives_prev_values() {
        let mut list: List<i32> = List::new();
        list.push(1);
        list.push(3);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn empty_peek_gives_none() {
        let list: List<i32> = List::new();
        assert_eq!(list.peek(), None);
    }

    #[test]
    fn peek_gives_last_value() {
        let mut list: List<i32> = List::new();
        list.push(3);
        assert_eq!(list.peek().map(|c| *c), Some(3));
        list.push(5);
        assert_eq!(list.peek().map(|c| *c), Some(5));
    }

    #[test]
    fn empty_peek_mut_gives_none() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.peek_mut(), None);
    }

    #[test]
    fn peek_mut_gives_last_value() {
        let mut list: List<i32> = List::new();
        list.push(3);
        if let Some(a) = list.peek_mut() {
            assert_eq!(*a, 3);
            *a = 5;
        } else {
            assert!(false);
        }
        assert_eq!(list.peek_mut().map(|c| *c), Some(5));
    }

    #[test]
    fn into_iter_collect_backwards() {
        let mut list: List<i32> = List::new();
        list.push(3);
        list.push(2);
        list.push(1);

        assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    #[test]
    fn iter_collect_backwards() {
        let mut list: List<i32> = List::new();
        list.push(3);
        list.push(2);
        list.push(1);

        let mut it = list.iter();
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), None);

        // Ensure the data is still there.
        assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    #[test]
    fn iter_mut_collect_backwards() {
        let mut list: List<i32> = List::new();
        list.push(3);
        list.push(2);
        list.push(1);

        let mut it = list.iter_mut();
        assert_eq!(it.next(), Some(&mut 1));
        assert_eq!(it.next(), Some(&mut 2));
        assert_eq!(it.next(), Some(&mut 3));
        assert_eq!(it.next(), None);

        // Ensure the data is still there.
        assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1, 2, 3]);
    }
}
