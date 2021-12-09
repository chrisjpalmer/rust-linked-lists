
use super::list::{Strong, List, insert_after, insert_before};

#[test]
fn test_insert_after1() {
    // hi, there
    let hi = Strong::new(List::new("hi"));

    // insert after None
    let there= Strong::new(List::new("there"));
    insert_after(&hi,&there);
    assert_eq!("[hi, there]", format!("{}",hi));
}

#[test]
fn test_insert_after2() {
    // hi, there
    let hi = Strong::new(List::new("hi"));
    let there= Strong::new(List::new("there"));
    insert_after(&hi,&there);

    // insert in between Head and Tail
    let im = Strong::new(List::new("im"));
    insert_after(&hi, &im);
    assert_eq!("[hi, im, there]", format!("{}",hi));
}

#[test]
fn test_insert_after3() {
    // hi, there
    let hi = Strong::new(List::new("hi"));
    let there= Strong::new(List::new("there"));
    insert_after(&hi,&there);

    // insert after Tail
    let chris = Strong::new(List::new("chris"));
    insert_after(&there, &chris);
    assert_eq!("[hi, there, chris]", format!("{}",hi));
}

#[test]
fn test_insert_after4() {
    // hi, there
    let hi = Strong::new(List::new("hi"));
    let there= Strong::new(List::new("there"));
    insert_after(&hi,&there);
    let chris = Strong::new(List::new("chris"));
    insert_after(&there, &chris);

    // insert after Middle and before Tail
    let how_are_you = Strong::new(List::new("how are you"));
    insert_after(&there, &how_are_you);
    assert_eq!("[hi, there, how are you, chris]", format!("{}",hi));
}

#[test]
fn test_insert_after5() {
    // hi, there
    let hi = Strong::new(List::new("hi"));
    let there= Strong::new(List::new("there"));
    insert_after(&hi,&there);
    let chris = Strong::new(List::new("chris"));
    insert_after(&there, &chris);
    let how_are_you = Strong::new(List::new("how are you"));
    insert_after(&chris, &how_are_you);

    // insert between 2 Middles
    let im_bob = Strong::new(List::new("im bob"));
    insert_after(&there, &im_bob);
    assert_eq!("[hi, there, im bob, chris, how are you]", format!("{}",hi));
}


// insert before tests..
#[test]
fn test_insert_before1() {
    // hi, there
    let hi = Strong::new(List::new("hi"));

    // insert before None
    let there= Strong::new(List::new("there"));
    insert_before(&there,&hi);
    assert_eq!("[hi, there]", format!("{}",hi));
}

#[test]
fn test_insert_before2() {
    // hi, there
    let hi = Strong::new(List::new("hi"));
    let there= Strong::new(List::new("there"));
    insert_after(&hi,&there);

    // insert in between Head and Tail
    let im = Strong::new(List::new("im"));
    insert_before(&there, &im);
    assert_eq!("[hi, im, there]", format!("{}",hi));
}

#[test]
fn test_insert_before3() {
    // hi, there
    let hi = Strong::new(List::new("hi"));
    let there= Strong::new(List::new("there"));
    insert_after(&hi,&there);

    // insert before Head
    let chris = Strong::new(List::new("chris"));
    insert_before(&hi, &chris);
    assert_eq!("[chris, hi, there]", format!("{}",chris));
}

#[test]
fn test_insert_before4() {
    // hi, there
    let hi = Strong::new(List::new("hi"));
    let there= Strong::new(List::new("there"));
    insert_after(&hi,&there);
    let chris = Strong::new(List::new("chris"));
    insert_after(&there, &chris);

    // insert before Tail and after Middle
    let how_are_you = Strong::new(List::new("how are you"));
    insert_before(&chris, &how_are_you);
    assert_eq!("[hi, there, how are you, chris]", format!("{}",hi));
}

#[test]
fn test_insert_before5() {
    // hi, there
    let hi = Strong::new(List::new("hi"));
    let there= Strong::new(List::new("there"));
    insert_after(&hi,&there);
    let chris = Strong::new(List::new("chris"));
    insert_after(&there, &chris);
    let how_are_you = Strong::new(List::new("how are you"));
    insert_after(&chris, &how_are_you);

    // insert between 2 Middles
    let im_bob = Strong::new(List::new("im bob"));
    insert_before(&chris, &im_bob);
    assert_eq!("[hi, there, im bob, chris, how are you]", format!("{}",hi));
}