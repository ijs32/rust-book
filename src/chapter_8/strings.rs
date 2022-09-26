use std::fmt::format;

pub fn main() {
    
    let data = "This is some data";
    
    let _s: String = String::from("this is some data");
    let _s: String = data.to_string();              // these are equivalent
    let _s: String = "this is some data".to_string();

    let mut s: String = String::from("Foo");
    s.push_str("Bar"); // you can use push without _str to do a single character 

    let s1: String = String::from("this is a string");
    let s2: String = String::from("this is also a string");
    let _s3 = s1 + &s2; // s1 has moved here and can no longer be used

    // fn add(self, s: &str) -> String { // although &s2 is a type &String, the add method is able to coerce it to a &str
    // well learn more about 'deref coercion' later
    // for adding multiple strings we can use the format! macro

    let s1: String = String::from("Hello,");
    let s2: String = String::from(" World");
    let s3: String = String::from("!");

    let _s: String = format!("{}{}{}", s1, s2, s3); // works the same as println! but returns the contents for assignment
    // Rust strings do not support indexing
    // remember strings are utf-8 encoded
    let _len = String::from("hola").len(); // this would return 4, not because there are 4 letters, but because hola requires 4 bytes to store

    let hello: String = "hello".to_string();
    let h = &hello[0..1]; // this is not best practices, it can cause your program to crash

    println!("this is h: {}", h);

    for char in hello.chars() {
        println!("this is char: {}", char) // for characters
    }
    
    for byte in hello.bytes() {
        println!("this is char: {}", byte) // for bytes
    }

}