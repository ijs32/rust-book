use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};

pub fn main() {
    // panic!("crash and burn!"); //causes program to panic at line 2 column five 'crash and burn'
    let _vec = vec![0, 1, 2, 3, 4, 5];

    // vec[99]; // causes program to panic, index out of bounds

    let file = File::open("hello.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("problem opening the file {:?}", other_error),
        },
    };
    ///////////////////////////////////////////////////////////////////////////////////
    // These two do the same thing, the bottom however, is more concise and readable //
    ///////////////////////////////////////////////////////////////////////////////////
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Error opening the file {:?}", error);
        }
    });

    let file = File::open("hello.txt").unwrap(); // this will call the panic macro for us if it fails.
                                                 // unwrap is basically a match abstraction(I think, I could be wrong)
    let file =
        File::open("hello.txt").expect("oopsies uh oh something went wrong opening 'hello.txt'");
    // expect can be better to use as we can pass it a custom message

    fn read_username_from_file() -> Result<String, Error> {
        let file = File::open("hello.txt");

        let mut file = match file {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    // this works just fine, however theres a much easier way to do it
    fn improved_username_function() -> Result<String, Error> {
        let mut file = File::open("hello.txt")?; // <- adding this question mark allows this line to work the same as our match expression
                                                 // we either set the variable to file, or we return the error to the function caller
        let mut s = String::new();
        file.read_to_string(&mut s)?; // and again here
        Ok(s)
    }

    // !!!! important note: the ? operator can only be used on functions with a return type of "Result"

    fn even_more_improved_username_function() -> Result<String, Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }

    fn more_improved_than_even_more_improved_username_function() -> Result<String, Error> {
        fs::read_to_string("hello.txt")
    }

    let username = read_username_from_file();

    // when panicking is ok

    // when youre still prototyping its a good idea to use simple unwraps or expects in order to leave clear markers of code you need to revisit
    // when you are finally ready to iron out the expected functionality of your program
    // if youre still sort of in the psuedocode-example phase, it can be a bit overwhelming to have detailed error handling getting in the way
    // of reading the actual code

    // when you have more information than the compiler
    // its ok to use unwrap when you know something cant possibly fail, for example:

    // use std::net::IpAddr;
    // let home: IpAddr = "127.0.0.1".parse().unwrap();

    // since the value we are unwrapping is a hardcoded string that we know for a fact is a valid IP address, we can feel comfortable unwrapping it
    // if this was a string that came from user input or some other unknown, we would want to handle the possibility of an error

    // when the program is beyond saving.
    // if a value that doesnt make sense is passed into your function, you should alert the person using it that the value passed makes no sense
    // likewise, when calling external code that returns a value that makes no sense, panic is reasonable
}
