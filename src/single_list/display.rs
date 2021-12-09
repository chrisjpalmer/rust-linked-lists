use super::list::{Strong};
use std::fmt;


impl<T> fmt::Display for Strong<T>
where T:fmt::Display {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("[")?;
        for list in self.clone().into_iter() {
            // write the value
            let list_r = list.borrow();
            ((*list_r).value).fmt(formatter)?;

            // write the comma
            let has_next = (*list_r).next.is_some();
            if has_next {
                formatter.write_str(", ")?;
            }
           
        }
        formatter.write_str("]")
    }
}