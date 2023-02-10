fn main() {
    // Variables

    let x = 1; //immutable variable
    println!("The value of x is: {x}");
    // x = 2; //will give error

    let mut y = 1; //mutable variable
    println!("The value of y is: {y}");
    y = 4;
    println!("The new value of y is: {y}");

    // Data Types

    let a = 58u8; // Allowed
    println!("The value of a is: {a}");
    // let b = 22.4u8; //not allowed since the number is of floating point

    let c = 22.4f64; // will work
    println!("The value of c is: {c}");

    let d = 1_000; //will be same as 1000
    println!("The value of d is: {d}");

    // other types of numbers represented in Rust
    // let decimal = 98_222;
    // let hex = 0xff;
    // let octal = 0o77;
    // let binary = 0b1111_0000;
    // let byte = b'A';

    // let mut overflow = 255u8; //this will throw error in debug mode only
    // overflow = overflow + 1; // Panics in debug, and wraps in release mode
    // println!("Value of overflow is: {overflow}");

    // Inferred Floating point variable
    let floating_num = 2.5; //f64
    println!("Value of floating_num is: {floating_num}");

    // Type annotated Floating point variable
    let another_floating_num: f32 = 4.3; //f32
    println!("Value of another_floating_num is: {another_floating_num}");

    // Boolean type
    let boolean_var = true;
    println!("Value of boolean_var is: {boolean_var}");

    // Type annotated boolean variable
    let another_boolean_var: bool = false;
    println!("Value of another_boolean_var is: {another_boolean_var}");

    // Character Type
    let p = 'p';
    let z: char = 'z';
    let emoji = '😄';
    println!("Values of p, z, emoji: {p}, {z}, {emoji}");

    // Numeric Operations
    // addition
    let sum = 5 + 10;
    println!("sum: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product: {product}");

    // division
    let quotient = 5.0 / 3.0; // Results in 1.6666666666666667 because numerator and denominator are decimals
    println!("quotient: {quotient}");
    let truncated = -5 / 3; // Results in -1 because numerator and denominator are integers
    println!("truncated: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {remainder}");

    // Constants

    const TWO: u32 = 2;
    println!("The value of TWO is: {TWO}");
}
