// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle { // implementation block with methods
//     fn get_area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// impl Rectangle { // implementation block with associative functions
//     fn create_square(size: u32) -> Rectangle {
//         Rectangle {
//             width: size,
//             height: size
//         }
//     }
// }

// fn main() {
//     let rect = Rectangle {
//         width: 30,
//         height: 50
//     };

//     let rect1 =Rectangle::create_square(25);

//     println!("rect: {:#?}", rect);

//     println!("The area is {} px", rect.get_area());
//     println!("Rect can hold rect1? {}", rect.can_hold(&rect1))
// }

#[derive(Debug)]
struct Person {
    name: String,
    email: String,
    age: u8
}

fn main() {
    let p1 = Person {
        name: String::from("Caleb"),
        email: String::from("caleb@email.com"),
        age: 25
    };

    let p2 = Person {
        name: String::from("Jose"),
        email: String::from("caleb@email.com"),
        ..p1
    };

    println!("person 1: {:#?}", p1);
    println!("person 2: {:#?}", p);
}