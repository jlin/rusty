fn main() {
    println!("Hello, world!");

    let a = 55;

    if a > 15 {
        println!("bigger than 15");
    } else if a > 4 {
        println!("bigger than 4");
    } else {
        println!("smaller or equal to 4");
    }

    let b = if a > 15 {1} else {15};
    println!("b is {b}");

    'outer: loop {
        println!{"print forever"};
        loop {
            println!{"break out of forever loop"};
            break 'outer;
        }
    }

    let mut a = 0;
    while a <= 5 {
        println!("a is {a}");
        a += 1; // a = a + 1
    }

    // for loops loop through collections

    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for element in a {
        println!{"element is {}", element};
    }

}
