fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

fn main() {
    let number_list = vec![28, 52, 63, 14, 50];
    let largest = largest(&number_list);
    println!("The largest number is {}", largest);

    let char_list = vec!['w', 'z', 'c', 'd'];
    let largest_char = largest(&char_list);
    println!("The largest character is {}", largest_char);
}
