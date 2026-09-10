#[derive(Debug)]
struct Rectangle {
    width: u64,
    length: u64,
}

impl Rectangle {
    fn area(&self) -> u64 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn square(size: u64) -> Self {
        Self {
            width: size,
            length: size,
        }
    }
}

fn main() {
    // refactoring
    // let width = 30;
    // let height = 50;

    // refactoring two
    // let rect = (30, 50);

    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        length: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        length: 45,
    };

    let rectl = Rectangle {
        width: dbg!(40 * 50),
        length: 50,
    };

    let _square = Rectangle::square(30);

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // println!("rectl is {rect1:#?}");
    dbg!(&rectl);
}

// fn area(rectangle: &Rectangle) -> u64 {
//     rectangle.width * rectangle.length
// }
