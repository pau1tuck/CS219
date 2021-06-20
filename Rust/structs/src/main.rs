#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(width: u32, height: u32) -> Rectangle { // Associated function/constructor
        Rectangle {
            width,
            height
        }
    }
    fn area(&self) -> u32 { // Method
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool { // Method
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle::square(35, 45);
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 50,
    };
    
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.", rect1.area()
    );

    println!("Rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("Rect1 can hold rect3: {}", rect1.can_hold(&rect3));
}
/*
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/


/* WITH VARIABLES
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
*/

/* WITH TUPLE
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
*/