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

    // let n = 10;
    // let mut fib = vec![0, 1];
    // for i in 2..n {
    //     let next_fib = fib[i - 1] + fib[i - 2];
    //     fib.push(next_fib);
    // }
    // println!("The {}th Fibonacci number is: {}", n, fib[n - 1]);

    let s1 = String::from("hello");
    // let s2 = s1; // s1 的所有权被转移到 s2，s1 现在无效
    // println!("{}", s1); // 这行会导致编译错误，因为 s1 已经无效
    // println!("{}", s2); // 这行是有效的，因为 s2 拥有

    // s1 的所有权
    // 解决方法是使用克隆来创建 s1 的副本
    println!("s1 is: {}", s1);

    let (s2, len) = calculate_length(s1); // s1 的所有权被转移到 calculate_length 函数
    println!("The length of '{}' is {}.", s2, len);
    // println!("after s1 is: {}", s1); // 这行会导致编译错误，因为 s1 的所有权已经被转移
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
