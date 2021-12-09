# Linked Lists

This project was a fun little test for myself, to see how hard it was to build a doubly linked list and a singly linked list. The constraints for this linked list implementation were:

1. list is stored on the heap
2. list is not thread safe
3. list is not meant to be performant
4. list must support insert operations. In the case of the doubly linked list, it must also support insertion before a list element.
5. list must support conversion to an iterator. Iterators are fun.
6. list must be printable. Printing stuff is fun.

I think I achieved this :)

## Usage

### Single Linked List

```rust
use single_list::{Strong, List, insert_after};

let hi = Strong::new(List::new("hi"));
let there = Strong::new(List::new("there"));
insert_after(&hi,&there);

for (index, el) in hi.into_iter().enumerate() {
    let el_r = el.borrow();
    println!("single_list: {index}: '{el}'", index=index, el=(*el_r).value);
}
```

### Doubly Linked List

```rust
use double_list::{Strong, List, insert_after, insert_before};

let hi = Strong::new(List::new("hi"));
let there = Strong::new(List::new("there"));
insert_after(&hi,&there);

for (index, el) in hi.into_iter().enumerate() {
    let el_r = el.borrow();
    println!("double_list: {index}: '{el}'", index=index, el=(*el_r).value);
}
```

## Notes

The single linked list maintains references through the member: `next: Option<Strong<T>>`. This is a very simple design and suits the single case well, as you only need to track one reference to the next item in the list.

As for the double case, I chose a more complicated approach which I think has some benefits. I introduced an enum for illustrating what position in the list an element has. The enum was called `Refs`. You can either be in the `Head, Middle, Tail or None` positions. `None` simply means you are not attached to any other element. This design was nice as what came with it were restrictions about the accessibility of the next and prev pointers, depending on where you were in the list. This made it easy to reason about what pointers to handle and it was built right into the typing system. Albeit it does make for some more complex code and special cases... which might be considered bad practice. However I wasn't trying to be perfect here... just having some fun... so I guess it doesn't matter too much.