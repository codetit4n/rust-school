fn main() {
    // Tuple type
    let tup1 = (450, 6.2, 1);
    println!("{:?}", tup1);

    // Tuple type with type annotations
    let tup2: (i32, f64, u8) = (450, 6.2, 1);
    let (x, y, z) = tup2;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    // Another way of accessing tuple elements
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let a = tuple.0;
    let b = tuple.1;
    let c = tuple.2;

    println!("The value of a is: {a}");
    println!("The value of b is: {b}");
    println!("The value of c is: {c}");
}
