fn main() {
    // if Expressions
    let number = 3;

    if number < 5 {
        println!("condition true");
    } else {
        println!("condition false");
    }

    // this will throw error
    // if number {
    //     println!("number was three")
    // }

    // else if expressions

    let another_number = 6;

    if another_number % 4 == 0 {
        println!("number is divisible by 4");
    } else if another_number % 3 == 0 {
        println!("number is divisible by 3");
    } else if another_number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // nested if else
    let a = 201;

    if a > 99 {
        if a > 200 {
            println!("Huge number");
        } else {
            println!("Big number");
        }
    } else {
        println!("Small number");
    }

    // if in a let statement
    let condition = true;
    let the_number = if condition { 5 } else { 6 };

    println!("The value of number is: {the_number}");

    let condition = true;
    let the_number = if condition { 5 } else { 6 };

    println!("The value of the_number is: {the_number}");

    // Will throw error
    // let new_number = if condition { 5 } else { "six" };
}
