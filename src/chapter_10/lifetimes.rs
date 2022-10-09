use std::fmt::Display;

pub fn main() {
    let r;

    // {
    //     let x = 5;
    //     r = &x; // -- borrow occurs here
    // } // x dropped here while still borrowed
    // this fails, lets fix it below
    // println!("r: {}", r);

    {
        let x = 5;
        r = &x; // -- borrow occurs here
        println!("r: {}", r);
    }
    // generic lifetimes in functions

    // fn longest_str(x: &str, y: &str) -> &str { // expected lifetime parameter
    //     if x.len() > y.len() { // this function returns a borrowed value but doesnt specify wether its borrowed from x or y
    //         x
    //     } else {
    //         y
    //     }
    // }

    // since the value youre returning is borrowed, Rust needs to know which value its being borrowed from
    // the problem is, we ourselves dont know if x or y will be returned
    // the borrow checker is confused, we can add generic lifetime parameters to fix this confusion

    // Lifetime Annotation Syntax
    // lifetime annotations dont change how long references live. just as functions can accept any type when
    // the signature specifies a generic type, functions can accept references with any lifetime by specifying
    // a generic lifetime parameter

    // &i32 - a reference
    // &'a i32 - a reference with an explicit lifetime
    // &'a mut i32 - a mutable reference with an explicit lifetime

    // lifetime annotations indicate that references first and second must both live as long as that
    // generic lifetime
    fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    // by passing in borrowed values with generic lifetimes, the return value will take the shorter of the lifetime
    // to ensure it isnt able to be called out of scope, for example:
    let string1 = String::from("hello there");

    {
        let string2 = String::from("hey!");
        let result = longest_str(&string1, &string2);
        println!("the longest string is: {}", result); // this works as both borrowed params are still in scope
    }

    // println!("the longest string is: {}", result); // this is not valid because string2 could be out of scope

    fn lifetime_ex<'a>(x: &'a str, y: &str) -> &'a str {
        // if we plan to always return the first string, we dont need to specify y's lifetime
        x
        // let result = String::from("really long string"); // this wont work as it would be a dangling reference
        // result.as_str();
    }

    // Lifetime Annotations in Struct Definitions

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }

        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    let novel: String = String::from("Call me Ian. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    }; // this means ImportantExcerpt cannot outlive the lifetime of first_sentence

    // fn example(s: &str) -> &str {
    //     s
    // }

    // an example of a function with specific generic types, trait bounds, and lifetimes, all at once
    fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
