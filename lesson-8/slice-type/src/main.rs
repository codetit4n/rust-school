fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear(); //empties the string. It becomes ""

    // println!("{}", word); // will throw error when we use string slices
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    let s1 = String::from("hello world");

    let hello = &s1[0..5];
    let world = &s1[6..11];

    println!("{hello} {world}");

    // Ranges in Rust
    let s2 = String::from("hello");

    // both of these are same
    let slice = &s2[0..2];
    let slice = &s2[..2];

    let len = s2.len();

    // both of these are same
    let slice = &s2[3..len];
    let slice = &s2[3..];

    // both of these are same
    let slice = &s2[0..len];
    let slice = &s2[..];

    // Examples for string literal in parameters
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    // Other slices
    let a = [1, 2, 3, 4, 5];
    // &[i32] type slice
    let slice = &a[1..3];
}

// without string slices
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// with string slices
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
