//VARIABLES
//1..

// Fix the error below with least amount of modification to the code
fn main() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

//2..

// Fill the blanks in the code to make it compile
fn main() {
    let mut x:i32 =  1;
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}

//3..

// Fix the error below with least amount of modification
fn main() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}

//5.

fn main() {
    let x: i32 = 12;
    {
        let x = 5;
        assert_eq!(x, 5);
    }

    assert_eq!(x, 12);

    let x =  42;
    println!("{}", x); // Prints "42".
}

//6.

fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}

//7.

fn main() {
    let x = 1; 
    println!("{}",x);
}

//8..


// Fix the error below with least amount of modification
fn main() {
    let  (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

//9.

fn main() {
    let (x, y);
    (x,y) = (3, 4);
    let [x, y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [1,2]);

    println!("Success!");
} 