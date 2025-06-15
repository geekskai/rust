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

    // let mut number = 3;
    // while number != 0 {
    //     println!("{}!", number);
    //     number -= 1;
    // }
    // println!("LIFTOFF!!!");
    // This is a simple while loop that counts down from 3 to 1 and then prints "LIFTOFF!!!".

    // ----------------------------------
    // let array = [10, 20, 30, 40, 50];
    // for element in array {
    //     println!("Element: {}", element);
    // }

    // -----------------------------------
    // for number in (1..4).rev() {
    //     println!("rev: {}!", number);
    // }
    // println!("LIFTOFF!!!");

    //  相互转换摄氏与华氏温度。
    // let celsius = 100.0;
    // let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    // println!("{}°C is {}°F", celsius, fahrenheit);
    // let fahrenheit = 212.0;
    // let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    // println!("{}°F is {}°C", fahrenheit, celsius);

    // • 生成第 n 个斐波那契数。

    let n = 10;
    let mut fib = vec![0, 1];
    for i in 2..n {
        let next_fib = fib[i - 1] + fib[i - 2];
        fib.push(next_fib);
    }
    println!("The {}th Fibonacci number is: {}", n, fib[n - 1]);
}
