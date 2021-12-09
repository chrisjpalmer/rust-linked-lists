use super::list::{Strong};

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
                ((*list_r).next.as_ref()).map(|n| {n.clone()})
            }
            None => None,
        };
        self.item = next;
        cur
    }
}