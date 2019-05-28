#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle1 = Rectangle {
        width: 50,
        height: 30,
    };

    println!("rectangle1 is {:?}", rectangle1);
    println!("rectangle1 is {:#?}", rectangle1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rectangle1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
