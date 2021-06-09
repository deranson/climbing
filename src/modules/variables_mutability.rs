/// Variables and Mutability
///
/// Note:
///
///   A variable is immutable by default in rust
///
///   Keyword mut make it mutable eg: let mut x = 1;
///
///   Shadow the variable can reuse the name but can change the value and type

pub fn create_variable() {
    let x = 5; // immutable by default, and i32 type by default
               // x= 6; // this will cause a compiling error as x is immutable by default
    println!("Value of x is {}", x);
}

pub fn mut_variable() {
    let mut m = 1; // use the 'mut' keyword to make it mutable
    println!("Now the value of m is {}", m);

    m = 2; // the value can be changed as it is mutable
    println!("And now the value of m is {}", m);
}

pub fn const_variable() {
    const C: u32 = 3; // const keyword, uppercase name, type of the value must be annotated, immutable always
    println!("The value of c is {}", C);

    // const mut E: u32 = 9; // Constants can't be mutable
}

pub fn shadowed_variable() {
    let s: u8 = 3;
    println!("Now the value of s is {}", s);

    let s: &str = "s"; // Shadowing the s
    println!("Now the value of s is {}", s);

    let s: String = String::from("Hello Rustaceans!"); // Shadowing the variable again
    println!("Now the value of s is {}", s);
}
