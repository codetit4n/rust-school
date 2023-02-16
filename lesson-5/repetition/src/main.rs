fn main() {
    // Non terminating loop - Don't forget to terminate it using ctrl-c
    // loop {
    //     println!("again!");
    // }

    // break example
    let mut counter = 0;
    loop {
        counter += 1;

        if counter == 10 {
            break;
        }
    }
    println!("The value of counter is {counter}");
    // continue example
    let mut number = 0;
    loop {
        number = number + 1;
        if number == 10 {
            break;
        }
        if number % 2 == 0 {
            continue;
        }
        println!("{number}");
    }
    // Returning value from a loop
    let mut new_counter = 0;

    let result = loop {
        new_counter += 1;

        if new_counter == 10 {
            break new_counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while loops
    let mut val = 3;

    while val != 0 {
        println!("{val}");

        val -= 1;
    }

    // for loop example

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // same example using while
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
