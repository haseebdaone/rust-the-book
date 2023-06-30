fn main() {
    // x can't be changed unless you add 'mut'
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // You are able to redeclare a 'let' variable and change the type
    let spaces = "   ";
    println!("spaces = {spaces}");
    let spaces = spaces.len();
    println!("spaces = {spaces}");

    // changing type with mut will give you an error
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
