use std::fmt;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rect({}x{})", self.width, self.height)
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let rect4 = Rectangle::square(40);

    println!("The area of the {} is {} square pixels.", rect1, rect1.area());
    println!("Can {} hold {}? {}", rect1, rect2, rect1.can_hold(&rect2));
    println!("Can {} hold {}? {}", rect1, rect3, Rectangle::can_hold(&rect1, &rect3));
    println!("Can {} hold {}? {}", rect1, rect4, rect1.can_hold(&rect4));
}
