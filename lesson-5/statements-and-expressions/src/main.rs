fn main() {
    // Statements
    let x = 6;
    println!("Value of x: {x}");
    // let x = (let y = 6);//will throw error

    // Expressions
    let z = 5 + 6;
    println!("Value of z: {z}");

    // Block example:
    let k = {
        let w = 3;
        w + 1
    }; //removing this ; will cause error

    println!("The value of k is: {k}");
}
