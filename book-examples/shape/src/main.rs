#[derive(Debug)]
struct Shape {
    height: i32,
    width: i32,
}

impl Shape {
    fn area(&self) -> i32 {
        self.height * self.width
    }
    fn perimeter(&self) -> i32 {
        2 * (self.height + self.width)
    }
    fn create_square(side: i32) -> Shape {
        Shape {
            height: side,
            width: side,
        }
    }
    fn can_hold(&self, other: &Shape) -> bool {
        if self.height >= other.height && self.width >= other.width {
            return true;
        }
        return false;
    }
}

fn main() {
    let rectangle1 = Shape {
        height: 3,
        width: 4,
    };
    println!("{:#?}", rectangle1);
    println!("Area of rectangle1 : {}", rectangle1.area());
    println!("Perimeter of rectangle1 : {}", rectangle1.perimeter());

    let square = Shape::create_square(3);
    println!("{:#?}", square);
    println!("Area of square : {}", square.area());
    println!("Perimeter of square : {}", square.perimeter());

    println!(
        "Can rectangle1 holds square: {}",
        rectangle1.can_hold(&square)
    );
}
