fn main() {
    println!("Hello, world!");
    let z = my_function(15);
    println!("my_function returned: {}", z)
}

fn my_function(x: i32) -> i32 {
    // return 5; // early return possible with the return keyword
    println!("my_function called with: {}", x);
    let y = 10;
    y // special no semicolon syntax at the last expression returns this last variable
    // this can be an expression
}