#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T: PartialEq> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }

    pub fn insert_sorted<F>(&mut self, data: T, compare: F)
    where
        F: Fn(&T, &T) -> std::cmp::Ordering,
    {
        match self.0 {
            None => self.push_front(data),
            Some((ref head, ref mut tail)) => match compare(&data, head) {
                std::cmp::Ordering::Less => self.push_front(data),
                _ => tail.insert_sorted(data, compare),
            },
        }
    }

    pub fn find(&self, value: &T) -> Option<&T> {
        let mut current = &self.0;
        while let Some((ref data, ref next)) = current {
            if data == value {
                return Some(data);
            }
            current = &next.0;
        }
        None
    }
    pub fn find_match(&self, value: &T) -> Option<&T> {
        match &self.0 {
            Some((ref data, ref next)) if data == value => Some(data),
            Some((_, ref next)) => next.find(value),
            None => None,
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_front(2);
    list.push_front(1);
    list.push_back(5);
    list.push_back(6);

    list.insert_sorted(4, |a, b| a.cmp(b));

    println!("{:?}", list);

    println!("{:?}", list.find(&5));
}
