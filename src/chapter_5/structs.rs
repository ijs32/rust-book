// pub fn structs() {
//     struct User {
//         username: String,
//         email: String,
//         sign_in_count: u64, // create a struct like so
//         active: bool,
//     }

//     let mut user1 = User {
//         email: String::from("ian@isilber.dev"),
//         username: String::from("isilber"), // you do not have to define the values in order
//         active: true,
//         sign_in_count: 32,
//     }; // in the case that the struct is mutable, we can change values like so:
//     user1.email = String::from("test@email.com");
//     // we can access values the same way
//     println!("this is user1 username: {}", user1.username);

//     fn build_user(email: String, username: String) -> User {
//         User {
//             username,
//             email,
//             sign_in_count: 1, // create a struct like so
//             active: true,
//         }
//     }

//     let user2 = build_user(String::from("fake@email.com"), String::from("bob"));

//     let user3 = User {
//         email: String::from("an email idc"),
//         username: String::from("an username idc"), // this allows us to create a struct using the same info from a previous instance
//         ..user2
//     };

//     struct Color(i32, i32, i32);
//     struct Point(i32, i32, i32);

//     let black = Color(0, 0, 0); // these are two different types
//     let origin = Point(0, 0, 0); // if a function is expecting a Color type, it wont accept a Point type

//     #[derive(Debug)] // enter debug mode in order to print structs, enums, and unions
//     struct Rectangle {
//         height: i32,
//         width: i32,
//     }

//     impl Rectangle {
//         fn area(&self) -> i32 {
//             self.width * self.height
//         }
//                             // these are both considered methods as they are called by an instance of Rectangle
//         fn can_hold(&self, other: &Rectangle) -> bool {
//             self.height > other.height && self.width > other.width
//         }
//                             // however, we can also create associated functions; functions like String::from();
//         fn square(size: i32) -> Rectangle {
//             Rectangle { height: size, width: size } // called on line 71
//         }

//     }

//     let rect1 = Rectangle { height: 30, width: 45, };
//     let rect2 = Rectangle { height: 30, width: 55, };

//     println!("rect1 is -- {:#?}", rect1);
//     println!("the area of rect1 is: {}", rect1.area());

//     println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));

//     let square = Rectangle::square(30);
// }

// // fn rectangles() {
// //     let width: i32 = 30;
// //     let height: i32 = 30;

// //     println!(
// //         "the area of the rectangle is: {}"
// //         calc_area(width, height)
// //     );                                       // this works, but we can make it more readable with a struct

// //     fn calc_area(width: i32, height: i32) -> i32 {
// //         width * height
// //     }
// // }
