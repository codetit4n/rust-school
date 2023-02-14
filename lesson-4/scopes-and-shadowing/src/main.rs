fn main() {
    // Scopes
    {
        let x = 6;
    }
    // println!("Value of x is {x}"); //this will throw error

    // Shadowing
    let y = 1;
    let y = y + 6;
    println!("Value of y: {y}");

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
}
