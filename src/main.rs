mod double_list;
mod single_list;

fn main() {
    double_list();
    single_list();
}

fn double_list() {
    use double_list::{Strong, List, insert_after, insert_before};

    // hi, there
    let hi = Strong::new(List::new("hi"));
    let there= Strong::new(List::new("there"));
    insert_after(&hi,&there);
    let im = Strong::new(List::new("im"));
    insert_before(&there, &im);

    for (index, el) in hi.into_iter().enumerate() {
        let el_r = el.borrow();
        println!("double_list: {index}: '{el}'", index=index, el=(*el_r).value);
    }
}

fn single_list() {
    use single_list::{Strong, List, insert_after};

    // hi, there
    let hi = Strong::new(List::new("hi"));
    let there= Strong::new(List::new("there"));
    insert_after(&hi,&there);
    let im = Strong::new(List::new("im"));
    insert_after(&hi, &im);

    for (index, el) in hi.into_iter().enumerate() {
        let el_r = el.borrow();
        println!("single_list: {index}: '{el}'", index=index, el=(*el_r).value);
    }
}


