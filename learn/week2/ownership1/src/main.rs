fn main() {
    println!("Hello, world!");

    let s1 = String:: from("String test");
    let r1 = &s1;
    
    print_this_by_borrowing(r1);
    print_this_and_take_ownership(s1);
    
    // can't run this line because ownership of s1 moved into the function
    //println!("s1 is {s1}");

    /*
    can't borrow here because print_this_and_take_ownership takes over and the variable is dropped at the end of that function
    print_this_by_borrowing(r1); // invalid, previously move is invalid 
    */

}

fn print_this_and_take_ownership(p1:String) {
    println!("{p1}, I now own this string");
    println!("The line for {p1} is drawn here, no further.");
}

fn print_this_by_borrowing(p1:&String){
    println!("{p1}, I borrowed this string by using a reference");
}
