#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
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
        rectangle1.area() 
    );
}

