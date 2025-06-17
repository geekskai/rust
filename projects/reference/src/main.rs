fn main() {
    let s1 = String::from("hello");

    // let (s2, len) = calculate_length(s1); // s1 的所有权被转移到 calculate_length 函数
    // println!("The length of '{}' is {}.", s2, len);

    // let len = calculate_length1(&s1); // 传递 s1 的引用
    // println!("The length of '{}' is {}.", s1, len);

    change(&s1);
}

fn change(s: &String) {
    s.push_str(", world!"); // 试图修改不可变引用
}

// fn calculate_length1(s: &String) -> usize {
//     let len = s.len(); // 返回字符串的长度
//     len
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let len = s.len(); // 计算字符串的长度
//     (s, len) // 返回字符串和它的长度
// }
