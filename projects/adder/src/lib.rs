struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

pub fn add_two(a: usize) -> usize {
    a + 3
}

pub fn greeting(name: &str) -> String {
    String::from("Hello ") + name + "!"
}

mod tests {
    use super::*;

    #[test]
    fn it_greets() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain the name, value was `{}`",
            result
        );
    }

    // #[test]
    // fn it_adds_two() {
    //     assert_eq!(add_two(2), 4);
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 5,
            height: 5,
        };
        let smaller = Rectangle {
            width: 2,
            height: 2,
        };
        assert!(larger.can_hold(&smaller));
    }

    // #[test]
    // fn smaller_cannot_hold_larger() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };
    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 1,
    //     };
    //     assert!(!smaller.can_hold(&larger));
    // }
}
