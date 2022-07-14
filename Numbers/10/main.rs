 // Fill the blanks
// use std::ops::{Range, RangeInclusive};
// fn main() {
//     assert_eq!((1..__), Range{ start: 1, end: 5 });
//     assert_eq!((1..__), RangeInclusive::new(1, 5));

//     println!("Success!");
// }


use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
    println!("Success!");
}


// solution: Filled with 5 and =5 to complete the code.