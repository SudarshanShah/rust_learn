fn main() {

    // for 1st way
    // let width1 = 30;
    // let height1 = 50;
    // println!("The area of rectangle is {} square pixels", area(width1, height1));
    
    // for 2nd way
    // let rect1 = (30, 50);
    // println!("The area of rectangle is {} square pixels",area(rect1));

    // 3rd way & 4th way
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("rect1 is {:#?}", rect1);

    // 3rd way
    // println!("The area of rectangle is {} square pixels",area(&rect1));

    // 4th way
    println!("The area of rectangle is {} square pixels", rect1.area());


    let rect2 = Rectangle {
        width: 10,
        height: 40
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(10);
    println!("The square Rectangle -> {:#?}", sq);
}

// 1st way
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// 2nd way
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// 3rd way
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// 4th way
// area() & can_hold() --> associated methods
// square() is associated functions
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self { 
            width: size, 
            height: size 
        }
    }
}

// 3rd way
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }