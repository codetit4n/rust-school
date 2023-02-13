fn main() {
    // Tuple type
    let tup1 = (450, 6.2, 1);
    println!("Tuple: {:?}", tup1);

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

    // Array Type
    let arr = [1, 2, 3, 4, 5];
    println!("Number array: {:?}", arr);

    let days = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];
    println!("Days: {:?}", days);

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Type annotated array: {:?}", array);

    // Accessing array elements
    let new_array = [1, 2, 3, 4, 5];
    let fist_element = new_array[0];
    let second_element = new_array[1];

    println!("Array: {:?}", new_array);
    println!("fist element is: {fist_element}");
    println!("second element is: {second_element}");
}
