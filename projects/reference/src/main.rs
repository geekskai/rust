fn main() {
    // let mut s1 = String::from("hello");

    // let (s2, len) = calculate_length(s1); // s1 的所有权被转移到 calculate_length 函数
    // println!("The length of '{}' is {}.", s2, len);

    // let len = calculate_length1(&s1); // 传递 s1 的引用
    // println!("The length of '{}' is {}.", s1, len);

    // change(&mut s1);
    // println!("s1 after change: {}", s1);

    let s1 = String::from("hello world");
    let word = first_word(&s1); // 获取第一个单词的引用
                                // s1.clear(); // 清空字符串 s1 // 这行会报错，因为 word 仍然引用 s1 的内容
    println!("The first word is: {}", word);
}

// fn change(s: &mut String) {
//     s.push_str(", world!"); // 试图修改不可变引用
// }

fn first_word(s: &String) -> &str {
    // &str 是字符串切片类型，表示字符串的一部分
    let bytes = s.as_bytes(); // 将字符串转换为字节数组

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // 找到第一个空格
            return &s[0..index]; // 返回从开头到空格前的子字符串
        }
    }
    &s[..] // 如果没有空格，返回整个字符串
}

// fn calculate_length1(s: &String) -> usize {
//     let len = s.len(); // 返回字符串的长度
//     len
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let len = s.len(); // 计算字符串的长度
//     (s, len) // 返回字符串和它的长度
// }
