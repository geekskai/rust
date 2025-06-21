#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 3;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    // println!("The area of the rectangle is: {}", area(&rect1));
    // println!("rect1's area is: {rect1:#?}");
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
