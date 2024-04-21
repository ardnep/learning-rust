fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    let _r1 = &mut s;
    // let r2 = &mut s; // cannot borrow mutably twice

    change(&mut s);

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    // cannot borrow mutably and immutably at the same time
    // println!("{}", r1);

    // Rules of references:
    // You cannot have more than 1 mutable
    // reference to a particular piece of data in a particular scope
    // This is to prevent data races at compile time (not sure i fully understand why compile
    // time)
    // Maybe something to do with the internals of the compiler ?
    //
    // You also cannot have a mutable reference while you have an immutable one
    // This is to prevent data unexpectedly changing while being read

    // Dangling references
    // let reference_to_nothing = dangle();

    let _reference_to_nothing = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    // cannot modify borrowed content
    // s.push_str(", world!");

    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
