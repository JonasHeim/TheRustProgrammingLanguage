#[derive(Debug)]
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


fn main() {
    let rectangle1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rectangle2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rectangle3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rectangle1 is {:?}", rectangle1);
    println!("rectangle1 is {:#?}", rectangle1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle1.area() 
    );

    println!("Can rectangle1 hold rectangle2? {}", rectangle1.can_hold(&rectangle2));
    println!("Can rectangle2 hold rectangle3? {}", rectangle2.can_hold(&rectangle3));
}

