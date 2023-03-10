/* usaul calc of rect's area */
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} suqare pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

/* use tuple to calc of rect's area */
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} suqare pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

/* use struct to calc of rect's area more clearly*/
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
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // dbg! will return the ownership of the value.
        height: 50,
    };

    println!(
        "The area of the rectangle is {} suqare pixels.",
        rect1.area()
    );

    println!("rect1 is {:#?} by println", rect1);

    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}