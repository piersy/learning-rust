#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }

    fn can_hold(&self, r: &Rectangle) -> bool {
        self.width >= r.width && self.height >= r.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}
fn main() {
    let mut rect1 = Rectangle {
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

    rect1.set_width(3);

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    // These two following uses of area are equivalent
    println!("rect1 area? {}", Rectangle::area(&rect1));
    println!("rect1 area? {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };
    let _ = rect2.max(other_rect);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&other_rect));
}
