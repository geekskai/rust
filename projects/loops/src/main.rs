fn main() {
    // loop {
    //     println!("loop again!");
    // }

    // let mut count = 0;
    // let result = loop {
    //     count += 1;
    //     if count == 10 {
    //         break count * 2; // Breaks the loop and returns count * 2
    //     }
    // };
    // println!("The result is: {}", result);

    // -------------------------------

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {}", count);
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {}", remaining);
    //         if remaining == 9 {
    //             break; // Breaks the inner loop
    //         }
    //         if count == 2 {
    //             break 'counting_up; // Breaks the outer loop
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    // println!("End count = {}", count);

    // ----------------------------------

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    // This is a simple while loop that counts down from 3 to 1 and then prints "LIFTOFF!!!".
}
