// #[derive(Debug, Clone, Copy)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let x: u32 = 3;
    let y: u32 = 4;
    // x and y are copied when they are passed to area2
    let a = area2(x, y);
    println!("{} * {} = {}", x, y, a);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let a = area(rect1);
    // rect1 was moved by area because rect does not have copy implemented. However we can easily
    // derive Copy (which needs clone to work) see commented section at top.
    println!("{} * {} = {}", rect1.width, rect1.height, a);
}
fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
fn area2(x: u32, y: u32) -> u32 {
    x * y
}
