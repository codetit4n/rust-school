fn main() {
    // Without reference
    let s1 = String::from("hello");

    // let (s2, len) = calculate_length(s1);

    // println!("The length of '{}' is {}.", s2, len);

    // With reference
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // using mutable reference
    let mut s = String::from("hello");

    change(&mut s);

    // data race prevention in Rust - Rust won't allow this

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // Using scopes to fix this issue

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    // More restrictions while using references
    // let ref1 = &s; // no problem
    // let ref2 = &s; // no problem
    // let ref3 = &mut s; // PROBLEM - will throw error

    // println!("{}, {}, and {}", ref1, ref2, ref3);

    // A complicated scenario
    let reference1 = &s; // no problem
    let reference2 = &s; // no problem
    println!("{} and {}", reference1, reference2);

    let reference3 = &mut s; // no problem
    println!("{}", reference3);

    // dangling reference error
    // let reference_to_nothing = dangle();
    let return_value = no_dangle();
    println!("{}", return_value);
}

// Without reference
// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();

//     (s, length)
// }

// With reference
fn calculate_length(s: &String) -> usize {
    // s.push_str(", world"); //with throw error
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// dangling reference error
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// fix dangling using move
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
