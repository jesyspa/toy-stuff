use std::ptr;
use std::iter;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: ptr::null_mut() }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = box Node { elem, next: None };
        let raw_tail: *mut _ = &mut *new_tail;

        if self.tail.is_null() {
            self.head = Some(new_tail);
        } else {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        }
        self.tail = raw_tail;
    }

    fn pop_node(&mut self) -> Option<Node<T>> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            *node
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
        Iter {
            next: self.head.as_deref(),
        }
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
        IterMut {
            next: self.head.as_deref_mut(),
        }
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
    fn pop_empty() {
        let mut xs: List<i32> = List::new();
        assert_eq!(xs.pop(), None);
    }

    #[test]
    fn push_pop() {
        let mut xs: List<i32> = List::new();

        xs.push(1);
        xs.push(2);
        xs.push(3);

        assert_eq!(xs.pop(), Some(1));
        assert_eq!(xs.pop(), Some(2));
        assert_eq!(xs.pop(), Some(3));
        assert_eq!(xs.pop(), None);
    }

    #[test]
    fn push_pop_push_pop() {
        let mut xs: List<i32> = List::new();

        xs.push(1);
        xs.push(2);

        assert_eq!(xs.pop(), Some(1));
        assert_eq!(xs.pop(), Some(2));
        assert_eq!(xs.pop(), None);

        xs.push(3);
        xs.push(4);

        assert_eq!(xs.pop(), Some(3));
        assert_eq!(xs.pop(), Some(4));
        assert_eq!(xs.pop(), None);
    }

    #[test]
    fn into_iter_collect() {
        let mut list: List<i32> = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    #[test]
    fn iter_collect() {
        let mut list: List<i32> = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut it = list.iter();
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), None);

        // Ensure the data is still there.
        assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    #[test]
    fn iter_mut_collect() {
        let mut list: List<i32> = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut it = list.iter_mut();
        assert_eq!(it.next(), Some(&mut 1));
        assert_eq!(it.next(), Some(&mut 2));
        assert_eq!(it.next(), Some(&mut 3));
        assert_eq!(it.next(), None);

        // Ensure the data is still there.
        assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1, 2, 3]);
    }
}
