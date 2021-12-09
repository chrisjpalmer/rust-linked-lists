use std::rc;
use std::cell;


pub struct List<T> {
    pub (super) next:Option<Strong<T>>,
    pub value: T,
}

impl<T> List<T> {
    pub fn new(value: T) -> Self {
        List{next:None, value:value}
    }
}

pub struct Strong<T> {
    list: rc::Rc<cell::RefCell<List<T>>>
}

impl<T> Strong<T> {
    pub fn new(list: List<T>) -> Self {
        Strong{list:rc::Rc::new(cell::RefCell::new(list))}
    }
    pub fn borrow(&self) -> cell::Ref<List<T>> {
        cell::RefCell::borrow(&self.list)
    }
    pub fn borrow_mut(&self) -> cell::RefMut<List<T>> {
        cell::RefCell::borrow_mut(&self.list)
    }
    pub fn clone(&self) -> Strong<T> {
        Strong{list:self.list.clone()}
    }
}

pub fn insert_after<T>(list: &Strong<T>, el: &Strong<T>) {
    // el must be None
    validate_none(el);

    // disconnect list->next -> should return the reference to the next.
    let last_next = disconnect(list);

    // connect list with el.
    connect(list, el);

    // connect el with last next - if there was one
    if let Some(last_next) = last_next {
        connect(el, &last_next);
    }
}

fn validate_none<T>(el: &Strong<T>) {
    let el_r = el.borrow();
    match &((*el_r).next) {
        None => {}
        _ => panic!("el was expected to be a single element")
    }
}

// disconnects list from its next value
fn disconnect<T>(list: &Strong<T>) -> Option<Strong<T>> {
    let mut list_r = list.borrow_mut();

    // capture the next value
    let next = (*list_r).next.as_ref().map(|n| {n.clone()});

    // unset the next
    (*list_r).next = None;

    next
} 

// connects el to list.
fn connect<T>(list: &Strong<T>, el:&Strong<T>) {
    // determine the new list refs
    let mut list_r = list.borrow_mut();
    (*list_r).next = Some(el.clone());
}


