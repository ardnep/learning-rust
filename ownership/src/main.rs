fn main() {
    // this is a string literal
    // stored on the stack
    let _s = "hello";

    // this is stored on the heap
    // and is mutable
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    // move
    let s1 = String::from("hello");

    // s1 is moved to s2
    // s1 is no longer valid
    let s2 = s1;

    // clone
    let s3 = s2.clone();

    println!("s2 = {} s3 = {}", s2, s3);
}
