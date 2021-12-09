use std::rc;
use std::cell;


pub struct List<T> {
    pub (super) refs:Refs<T>,
    pub value: T,
}

impl<T> List<T> {
    pub fn new(value: T) -> Self {
        List{refs:Refs::None, value:value}
    }
}

pub (super) enum Refs<T> {
    Head {
        next: Strong<T>,
    },
    Middle {
        next: Strong<T>,
        prev: Weak<T>,
    },
    Tail {
        prev: Weak<T>,
    },
    None,
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
    fn downgrade(&self) -> Weak<T> {
        Weak{list:rc::Rc::downgrade(&self.list)}
    }
}

pub struct Weak<T> {
    list: rc::Weak<cell::RefCell<List<T>>>
}


impl <T> Weak<T> {
    fn upgrade(&self) -> Option<Strong<T>> {
        self.list.upgrade().map(|list| { Strong{list} })
    }
    fn clone(&self) -> Weak<T> {
        Weak{list:self.list.clone()}
    }
}

pub fn insert_after<T>(list: &Strong<T>, el: &Strong<T>) {
    // el must be None
    validate_none(el);

    // disconnect list->next -> should return the reference to the next.
    let last_next = disconnect_next(list);

    // connect list with el.
    connect(list, el);

    // connect el with last next - if there was one
    if let Some(last_next) = last_next {
        connect(el, &last_next);
    }
}

pub fn insert_before<T>(list: &Strong<T>, el: &Strong<T>) {
    // el must be None
    validate_none(el);

    // disconnect list->prev -> should return the reference to the prev.
    let last_prev = disconnect_prev(list);
            
    // connect last_prev with el
    if let Some(last_prev) = last_prev {
        connect(&last_prev, el);
    }

    // connect el with list
    connect(el, list);
}

fn validate_none<T>(el: &Strong<T>) {
    let el_r = el.borrow();
    match &((*el_r).refs) {
        Refs::None => {}
        _ => panic!("el was expected to be None")
    }
}

// disconnects list from its next value
// list must be Head or Middle for the operation to do anything
fn disconnect_next<T>(list: &Strong<T>) -> Option<Strong<T>> {
    let mut list_r = list.borrow_mut();

    // calculate the new refs for list.
    let disc = match &((*list_r).refs) {
        Refs::Head{next} => {
            // if list was the Head of the list, it now becomes a None
            Some((Refs::None, next.clone()))
        },
        Refs::Middle{prev, next} => {
            // if list was the Middle of the list it now becomes a Tail
            Some((Refs::Tail{prev: prev.clone()}, next.clone()))
        },
        _ => None
    };

    // handle the disconnection if it will occur
    match disc {
        Some((new_list_refs, next)) => {
            (*list_r).refs = new_list_refs;

            // calculate the new next refs
            let mut next_r = next.borrow_mut();
            let new_next_refs = match &((*next_r).refs) {
                Refs::Middle{next, ..} => {
                    // if next was the middle of the list it now becomes the new Head
                    Refs::Head{next:next.clone()}
                },
                Refs::Tail{..} => {
                    // if next was the end of the list, it now becomes a None
                    Refs::None
                },
                _ => panic!("when calculating new_next_refs, next was either None or Head which is an invalid state")
            };
            (*next_r).refs = new_next_refs;
            Some(next.clone())
        },
        None => None
    }
} 

// disconnects list from its prev value
// list must be Middle or Tail
fn disconnect_prev<T>(list: &Strong<T>) -> Option<Strong<T>> {
    let mut list_r = list.borrow_mut();

    // calculate the new refs for list
    let disc = match &((*list_r).refs) {
        Refs::Middle{prev, next} => {
            // if list was the Middle of the list, it now becomes a Head
            Some((Refs::Head{next:next.clone()}, prev.clone()))
        },
        Refs::Tail{prev} => {
            // if list was the Tail of the list, it now becomes a None
            Some((Refs::None, prev.clone()))
        },
        _ => None
    };

    // handle the disconnection if it will occur
    match disc {
        Some((new_list_refs, prev)) => {
            (*list_r).refs = new_list_refs;

            // calculate the new prev refs
            let prev = prev.upgrade().unwrap();
            let mut prev_r = prev.borrow_mut();
            let new_prev_refs = match &((*prev_r).refs) {
                Refs::Head{..} => {
                    // if prev was the Head of the list, it now becomes a None
                    Refs::None
                },
                Refs::Middle{prev, ..} => {
                    // if prev was in the Middle of the list, it now becomes a Tail
                    Refs::Tail{prev:prev.clone()}
                }
                _ => panic!("when calculating new_prev_refs, prev was either None or Tail which is an invalid state")
            };
            (*prev_r).refs = new_prev_refs;
            Some(prev.clone())
        },
        None => None
    }
} 

// connects el to list.
// list must be Tail or None and el must be Head or None
fn connect<T>(list: &Strong<T>, el:&Strong<T>) {
    // determine the new list refs
    let mut list_r = list.borrow_mut();
    let new_list_refs = match &((*list_r).refs) {
        Refs::Tail{prev} => {
            // if list was a Tail, it becomes a Middle
            Refs::Middle{
                prev: prev.clone(),
                next: el.clone(),
            }
        },
        Refs::None => {
            // if list was a None, it becomes a Head
            Refs::Head{
                next: el.clone(),
            }
        }
        _ => panic!("list must be None or Tail when connecting"),
    };
    (*list_r).refs = new_list_refs;
    
    let mut el_r = el.borrow_mut();
    let new_el_refs = match &((*el_r).refs) {
        Refs::None => {
            // if el was a None, it becomes a Tail
            Refs::Tail{
                prev: list.downgrade(),
            }
        },
        Refs::Head{next} => {
            // if el was a Head, it becomes a Middle
            Refs::Middle{
                prev: list.downgrade(),
                next: next.clone(),
            }
        },
        _ => panic!("el must be None or Head when connecting")
    };
    (*el_r).refs = new_el_refs;
}


