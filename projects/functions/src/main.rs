// fn main() {
//     println!("Hello, world!");
//     another_function(5, 'a');
// }

// fn another_function(x: i32, unit_label: char) {
//     // println!("This is another function.");
//     println!("The value of x is: {}", x);
//     println!("The unit label is: {}", unit_label);
// }
// This function is used to demonstrate a simple addition operation.

// fn main() {
//     let y = {
//         let x = 5;
//         x + 1
//     };

//     println!("The value of y is: {}", y);
// }

fn five() -> i32 {
    5
}
fn main() {
    let x = five();
    println!("The value of x is: {}", x);
}
