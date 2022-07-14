// Modify `assert!` to make it work


// fn main() {
//     let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
//     assert!(v == 1579);

//     println!("Success!");
// } 

fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);
    println!("Success!");
}


// solution: changed v == 1579 -> 1597 as, if we add the v it will give 1597.