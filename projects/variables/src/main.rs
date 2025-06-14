fn main() {
    // Variable declaration and initialization
    let tuple: (i32, f64, u8) = (177, 2.3, 3);
    let (x, y, z) = tuple;
    println!("x: {}, y: {}, z: {}", x, y, z);
    println!("tuple.0: {:?}", tuple.0); // Accessing tuple elements directly
    println!("tuple.1: {:?}", tuple.1);

    let array = [4, 5, 6];
    let [a, b, c] = array;
    println!("a: {}, b: {}, c: {}", a, b, c);

    let (first, second, third) = (7, 8, 9);
    println!("first: {}, second: {}, third: {}", first, second, third);

    let (x, y) = (10, 11);
    println!("x: {}, y: {}", x, y);
}
