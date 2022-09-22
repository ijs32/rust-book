// const EXAMPLE: i32 = 123;

// pub fn variables() {
//     // variables are immutable in Rust by default.
//     let mut x: i32 = 5;
//     println!("x is equal to {}", x);
//     x = 6; // this will not work if x is not defined as mutable
//     println!("x is equal to {}", x);
//     println!("this is an example of a constant {}", EXAMPLE);

//     // -- shadowing a variable -- 
//     // you can shadow a variable by simply reassigning it
//     let x: i32 = 7;
//     // this doesnt require mutability because we are recreating the variable
//     let x: bool = x % 2 == 0;
    
//     println!("original x is even: {}", x);
//     // we can add an underscore before a variable we arent using
//     let _y: i32 = 34243;

//     data_types()
// }

// fn data_types() {
//     // Scalar types
//     let integer: i32 = 4;
//     let char: char = 'a';
//     let boolean: bool = true;
//     let float: f32 = 2.0324;

//     println!("an integer is a number: {}.", integer);
//     println!("a char is a letter: {}.", char);
//     println!("a boolean is a true or false value: {}.", boolean);
//     println!("a float is a floating point number: {}.", float);

//     // be careful of integer overflow, something like an i8 can contain numbers -127 to 128
//     // an unsigned integer of u8 on the other hand, can handle numbers as large as 255

//     //Compound types

//     // tuples have a fixed length, they cannot grow or shrink
//     // unlike arrays they can hold different data types together
    
//     // type annotations are not neccessary
//     let tup: (i32, char, f32) = (82399, 'a', 9.012);

//     // we can use pattern matching to destructure tuples like so:

//     let (_int, _char, _float) = tup;

//     // I could print these out but im not gonna
//     // we can access values in a tuple with simple dot notation

//     let _int = tup.0;
//     let _char = tup.1;
//     let _float = tup.2;

//     // arrays, arrays have a fixed length like tuples but can only contain one data type
//     // array data is allocated on the stack since, this is important to note for
//     // efficiency reasons

//     let array: [i32; 5] = [1, 2, 3, 4, 5];

//     // same syntax as most languages for accessing array values
//     let _first_value = array[0];
//     // let _first_value = array[7]; this would throw a very helpful index
//     // out of bounds compiler error
//     function();
// }

// fn function() {
//     // calling a function with a parameter
//     fn print_number(x: i32) {
//         println!("this is the function param: {}", x);
//     }
//     // calling a function with a parameter and expecting it to return something
//     fn return_number(x: i32) -> i32 {
//         x
//     }

//     print_number(32);
//     let x: i32 = return_number(0);
//     println!("this is the return value of return number: {}", x);

//     // expressions vs statements
//     // pretty simple expressions evaluate to something statements do not

//     control_flow();
// }

// fn control_flow() {
//     // if expressions
//     // going to skip down because if-else if-else control flow
//     // is identical to pretty much any other language
//     // i will mention this:
//     let x: i32 = 5;
    
//     /* 
//     if x {
//         println!("this is an invalid conditional!")
//     }

//     rust will not try to convert ints to bools like other
//     scripting languages would
    
//     this will work:  */
//     if x != 0 {
//         println!("perfectly fine conditional!");
//     }
//     // when we convert x into a bool expression ourselves, the conditional will work
//     // we can also use if in a let statement
//     let _number: i32 = if x!= 0 {
//         x
//     } else { // make sure both values are the same data type
//         0
//     };

//     loops();
// }

// fn loops() {
//     // loops are simple in rust

//     let mut x: i32 = 0;

//     let mut more_than_five: i32 = loop {
//         if x > 5 { break x }
//         println!("this is x: {}", x);
//         x += 1;
//     };

//     println!("this is more_than_five: {}", more_than_five);
//     // we can also do a simple conditional while loop
//     while more_than_five > 0 {
//         println!("still greater than 0: {}", more_than_five);
//         more_than_five -= 1;
//     }
//     // looping over an array with a for loop
//     let array: [i32; 5] = [1, 2, 3, 4, 5];
//     for num in array.iter() {
//         println!("this is num: {}", num);
//     }

// }