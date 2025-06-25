// #[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }
    // fn height(&self) -> bool {
    //     self.height > 0
    // }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

enum IpAddress {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("Message called");
    }
}

fn main() {
    let msg = Message::Write(String::from("Hello, world!"));
    msg.call();

    // let scale = 3;

    // let home = IpAddress::V4(String::from("127.0.0.1"));

    // let loopback = IpAddress::V6(String::from("::1"));

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // let rect2 = Rectangle {
    //     width: 10,
    //     height: 40,
    // };
    // let rect3 = Rectangle {
    //     width: 60,
    //     height: 45,
    // };

    // println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    // println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3));

    // if rect1.height() {
    //     println!("rect1's {}", rect1.width);
    // } else {
    //     println!("rect1's height is not greater than 0");
    // }
    // println!("rect1 is {}", rect1.area());
    // rect1.area();
    // let rect1 = Rectangle {
    //     width: dbg!(30 * scale),
    //     height: 50,
    // };

    // dbg!(&rect1);

    // println!("The area of the rectangle is: {}", area(&rect1));
    // println!("rect1's area is: {rect1:#?}");
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
