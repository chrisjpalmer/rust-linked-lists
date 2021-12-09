use super::list::{Strong, Refs};

pub struct ListIterator<T>{
    item: Option<Strong<T>>
}

impl<T> std::iter::IntoIterator for Strong<T> {
    type Item = Strong<T>;
    type IntoIter = ListIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        ListIterator{item:Some(self)}
    }
}

impl<T> std::iter::Iterator for ListIterator<T> {
    type Item = Strong<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = match &self.item {
            Some(list) => Some(list.clone()),
            None => None,
        };
        let next = match &self.item {
            Some(list) => {
                let list_r = list.borrow();
                match &((*list_r).refs) {
                    Refs::Head{next} => {
                        Some(next.clone())
                    },
                    Refs::Middle{next, ..} => {
                        Some(next.clone())
                    },
                    _ => None
                }
            }
            None => None,
        };
        self.item = next;
        cur
    }
}