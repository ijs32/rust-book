// extracting duplicating code into functions
pub fn largest_thing<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &num in list.iter() {
        if num > largest {
            largest = num; // now we can use this function to find the largest number in
        } // many lists instead of duplicating the code -- duh
    }
    largest
}

struct Point<T> {
    // now we can use point with any type
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x // this would return the x value of point as a type T
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
struct DiversePoint<U, T> {
    // now we can use point with any type
    x: T,
    y: U,
}

pub fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 5.5, y: 10.9 };

    // let float = Point {x: 5.5, y: 10 }; this wont work because the times are different
    let float = DiversePoint { x: 5.5, y: 10 };
    // keep in mind using lots of generics makes code harder to read, if youre using lots of generics
    // it could mean your code is just bad lol, rewrite that trash

    // the Option<T> and Result<T, E> enums should make more sense now
    // Rust compiles generic code into code that specifies the type, this keeps runtime performance impacts negligible

    let nums = vec![1, 2, 3, 4, 5, 6, 1000];
    let chars = vec!['a', 'b', 'c', 'd', 'z'];
    let result = largest_thing(&nums);
    println!("the largest number is {}", result);
}
