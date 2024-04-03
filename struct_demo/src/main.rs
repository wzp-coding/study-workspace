#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(self: &Self) -> i32 {
        self.width * self.height
    }

    fn square(size: i32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn can_hold(self: &Self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 60,
    };

    let rect2 = Rectangle::square(50);

    let rect3 = Rectangle {
        width: 70,
        height: 80,
    };
    println!("rect1:{:?}", rect1);
    println!("rect2:{:?}", rect2);
    println!("rect3:{:?}", rect3);
    println!("rect3.can_hold(rect1):{:?}", rect3.can_hold(&rect1));
    println!("rect3.can_hold(rect2):{:?}", rect3.can_hold(&rect2));
}
