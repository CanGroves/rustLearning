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

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, another_rect: &Rectangle) -> bool {
        self.width >= another_rect.width && self.height >= another_rect.height
    }

    fn suqare(size: u32) -> Rectangle {
        Rectangle { width: size, height: size, }
    }
}

fn main() {
    let scale = 1;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // dbg! will return the ownership of the value.
        height: 50,
    };

    println!(
        "The area of the rectangle is {} suqare pixels.",
        rect1.area()
    );

    println!("rect1 is {:#?} by println", rect1);

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    dbg!(&rect1);

    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

    let rect3 = Rectangle {
        width: 30,
        height: 60,
    };

    println!("Can rect1 hold rect2? : {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? : {}", rect1.can_hold(&rect3));

    let sq = Rectangle::suqare(3);

    println!("sq is {:#?} by println", sq);

}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }