fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[2];

    // println!("The third element is {} !", third);

    // let third: Option<&i32> = v.get(2);

    // match third {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }

    // let not_exists = v.get(100);
    let not_exists = &v[100]; // This will panic at runtime if uncommented

    println!("The value of not_exists is {:?}", not_exists);
}
