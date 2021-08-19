use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    fn prepend_node(&self, node: Node<T>) -> List<T> {
        List { head: Some(Rc::new(node)) }
    }

    pub fn prepend(&self, t: T) -> List<T> {
        self.prepend_node(Node {
            elem: t,
            next: self.head.clone(),
        })
    }

    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List::new()
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

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(boxed_node) = self.head.take() {
            if let Ok(mut node) = Rc::try_unwrap(boxed_node) {
                self.head = node.next.take()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn head_after_prepend() {
        let xs: List<i32> = List::new();
        assert_eq!(xs.head(), None);
        let xs = xs.prepend(1);
        assert_eq!(xs.head(), Some(&1));
        let xs = xs.prepend(3);
        assert_eq!(xs.head(), Some(&3));
    }

    #[test]
    fn head_after_tail() {
        let xs: List<i32> = List::new();
        let xs = xs.prepend(1);
        let xs = xs.prepend(3);
        let xs = xs.tail();
        assert_eq!(xs.head(), Some(&1));
        let xs = xs.tail();
        assert_eq!(xs.head(), None);
        let xs = xs.tail();
        assert_eq!(xs.head(), None);
    }

    #[test]
    fn iter() {
        let xs: List<i32> = List::new();
        let xs = xs.prepend(1);
        let xs = xs.prepend(3);
        assert_eq!(xs.iter().collect::<Vec<_>>(), vec![&3, &1]);
    }
}
