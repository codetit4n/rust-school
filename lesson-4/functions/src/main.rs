fn main() {
    println!("Main function.");

    another_function();

    parametrized_function(70);
    parametrized_function2(70, 'L');

    println!("add function result: {:?}", add(4, 5));
    // println!("add function result: {:?}", add2(4, 5)); //throws error
    println!("add function result: {:?}", add3(4, 5)); //will work
}

fn another_function() {
    println!("Another function.");
}

fn parametrized_function(x: i32) {
    println!("Value sent from main(): {x}")
}

fn parametrized_function2(x: i32, y: char) {
    println!("Values sent from main(): {x}, {y}")
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// fn add2(a: i32, b: i32) -> i32 {
//     a + b; //this will fail
// }

fn add3(a: i32, b: i32) -> i32 {
    return a + b;
}
