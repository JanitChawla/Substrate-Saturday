// Fix errors and panics to make it work

// fn main() {
//    let v1 = 251_u8 + 8;
//    let v2 = i8::checked_add(251, 8).unwrap();
//    println!("{},{}",v1,v2);
// } 


fn main() {
   let v1 = 247_u8 + 8;
   let v2 = i8::checked_add(119, 8).unwrap();
   println!("{},{}",v1,v2);
}



// solution: changed 251_u8 to 247_u8 in v1 and add(251,8) to (119,8) in v2, since it is the max value of u8 and i8.