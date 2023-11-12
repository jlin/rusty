fn main() {
    println!("Hello, world!");

    // data types lesson


    // single value types, scalar data types

    let b1: bool = true;
    println!("{b1}");

    // unsigned integer (greater than 0)

    let i1: u8 = 1;
    let i2: u16 = 1;
    let i3: u32 = 1;
    let i4: u64 = 1;
    let i5: u128 = 1;

    // signed integer

    let i6: i8 = -1;
    let i7: i16 = -1;
    let i7: i32 = -1;
    let i8: i64 = -1;
    let i9: i128 = -1;

    // floating point

    let f1: f32 = 1.0;
    let f2: f64 = 1.0;

    // platform specific integers

    let p1: usize = 1; // pointer sized unsigned integer
    let p2: isize = 1; // pointer sized signed integer


    // characters, &str, String

    // ' ' for char literals
    // " " for string literals
    let c1: char = 'c';
    let s1: &str = "hello"; // string slice &str string literals
    let s2: String = String::from("hello"); 

    // multivalue types, compound data types

    // arrays
    // multiple values of same type

    // explicit type annotation of data type, and number of elements
    // also able to infer type if annotation omitted

    let a1: [i32 ; 5] = [1,2,3,4,5];
    let i1: i32 = a1[0];
    let i2: i32 = a1[1];
    println!("first element 0 indexed from array a1 is {i1}");
    println!("second element 0 indexed from array a1 is {i2}");

    // tuples
    // multiple values of different types

    let t1: (i32, i32, i32) = (1,2,3);
    let t2: (f32, i32, f32) = (1.5, 2, 3.5);
    let t3: (i32, f64, &str) = (5, 5.0, "5");

    // extracting element 3 of t3 tuple
    let s1: &str = t3.2;

    // destructure t3 tuple, type annotations can be automatically infered from the elements of t3
    let(i1, f1, s1)= t3;

    // unit type - empty tuple, returned implicitly
    // functions that don't return value returns a unit type
    let unit = ();

    // type alias

    type Age = u8;

    let a1: Age = 43;


}
