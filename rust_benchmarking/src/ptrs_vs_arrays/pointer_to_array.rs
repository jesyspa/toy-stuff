use crate::common::to_index;

#[derive(Default)]
pub struct MapSum {
    children: Option<Box<[MapSum; 26]>>,
    contrib: i32,
    sum: i32,
}

impl MapSum {
    pub fn new() -> Self {
        Default::default()
    }

    fn insert_impl(&mut self, key: &str, val: i32) -> i32 {
        let change;
        match key.bytes().next().map(to_index) {
            None => {
                change = val - self.contrib;
                self.contrib = val;
            }
            Some(ix) => {
                if self.children.is_none() {
                    self.children = Some(Box::new(Default::default()));
                }
                change = self.children.as_mut().unwrap()[ix].insert_impl(&key[1..], val);
            }
        }
        self.sum += change;
        change
    }

    pub fn insert(&mut self, key: &str, val: i32) {
        self.insert_impl(key, val);
    }

    pub fn sum(&self, prefix: &str) -> i32 {
        match prefix.bytes().next().map(to_index) {
            None => self.sum,
            Some(ix) => match &self.children {
                None => 0,
                Some(arr) => arr[ix].sum(&prefix[1..]),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_sum() {
        let ms = MapSum::new();
        assert_eq!(ms.sum(""), 0);
    }

    #[test]
    fn one_string() {
        let mut ms = MapSum::new();
        ms.insert("foo", 3);
        assert_eq!(ms.sum(""), 3);
        assert_eq!(ms.sum("f"), 3);
        assert_eq!(ms.sum("foo"), 3);
        assert_eq!(ms.sum("bar"), 0);
    }

    #[test]
    fn two_strings() {
        let mut ms = MapSum::new();
        ms.insert("foo", 3);
        ms.insert("fee", 2);
        assert_eq!(ms.sum(""), 5);
        assert_eq!(ms.sum("f"), 5);
        assert_eq!(ms.sum("foo"), 3);
        assert_eq!(ms.sum("fe"), 2);
        assert_eq!(ms.sum("bar"), 0);
    }

    #[test]
    fn prefix() {
        let mut ms = MapSum::new();
        ms.insert("foo", 3);
        ms.insert("food", 2);
        assert_eq!(ms.sum(""), 5);
        assert_eq!(ms.sum("f"), 5);
        assert_eq!(ms.sum("foo"), 5);
        assert_eq!(ms.sum("food"), 2);
        assert_eq!(ms.sum("bar"), 0);
    }
}
