fn main() {
    // Strings
    let s = "Hello";
    println!("String literal: {s}");
    // String type from string literal
    let s1 = String::from("Hello there!");
    println!("String from string literal: {s1}");
    // Mutation in String type
    let mut s2 = String::from("Hello");
    s2.push_str(", world!");
    println!("{s2}");

    // Memory Allocation
    {
        let s = String::from("Hello");
        println!("s: {s}")
    } // s goes out of scope here and the drop is performed

    // Understanding allocation
    // fixed sized variables - move is not an issue here
    let x = 6;
    let y = x;
    println!("x: {x} | y: {y}");

    // clone move

    // move
    let str1 = String::from("Hello");
    let str2 = str1; //move happening here
                     // println!("{str1}, world!"); //will throw error

    // clone
    let str3 = String::from("Hi");
    let str4 = str3.clone(); //clone happening here
    println!("str3: {str3} | str4: {str4}");

    // Ownership in Functions
    let num = 4;

    print_num(num);
    print_num(num); //compiler will allow because of Copy

    let str5 = String::from("hey!");

    print_str5(str5);
    // print_str5(str5); //compiler will not allow because of move

    // Return Values and Ownership
    let returned_value = give_ownership();
    println!("returned_value: {returned_value}");

    let to_pass = String::from("hello");
    let got = takes_and_gives_back(to_pass);
    println!("Returned: {got}");
}

fn print_num(some_integer: i32) {
    println!("{some_integer}");
}

fn print_str5(some_string: String) {
    println!("{some_string}");
}

fn give_ownership() -> String {
    let some_string = String::from("hey!");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
