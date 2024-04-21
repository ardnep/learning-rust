fn main() {
    // string slices

    let s = String::from("hello world ");
    // let len = s.len();

    // let hello = &s[0..5];
    let hello = &s[..5];
    // let world = &s[6..len];
    let world = &s[6..];

    let hello_world = &s[..];

    println!("{} {} {}", hello, world, hello_world);

    let first_word = first_word(&s);

    println!("first word: {}", first_word);

    let second_word = second_word(&s);

    println!("second word: {}", second_word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, [2, 3]);

    println!("{:?}", slice);
}
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let mut j = 0;
    let mut word_count = 0;
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' || i == s.len() - 1 {
            if word_count == 1 {
                return &s[j..i + 1];
            }
            j = i + 1;
            word_count += 1;
        }
    }

    &s[..]
}
