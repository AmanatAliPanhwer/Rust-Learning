#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn create_rect(width: u32, height: u32) -> Self {
        Self { width: width, height: height }
    }
}

fn main() {
    let rect1 = Rectangle::create_rect(50, 80);
    let rect2 = Rectangle::create_rect(30, 50);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(rect2))
}
