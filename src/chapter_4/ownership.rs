// pub fn ownership() {
//     // the main feature that sets Rust apart

//     /*
//     stack vs heap

//     - stack is stored first in last out
//     - all data must have known fixed size
//     - stack is faster to read and write to
//     - stack-overflow is when you over allocate memory and the program crashes

//     heap

//     - dynamically sized
//     - returns a pointer to where the value is stored

//     ownership helps solve many memory problems related to the stack and heap
//     ownership is crucial to understand
//     more details but lets keep it short

//     Ownership rules
//     1. Each value in rust has a variable that is called its 'owner'
//     2. There can only be one owner at a time
//     3. when the owner goes out of scope the value will be dropped

//     Variable Scope
//     */
//     let x: i32 = 14; // in scope
//     {
//         let _y: i32 = x; // this will work since x is defined in the wider scope
//     };
//     // println!("this is int: {}", y); // unfortunately, we can never access y outside
//     println!("this is int: {}", x);

//     // the string type
//     let mut str: String = String::from("Hello");
//     str.push_str(", World!"); // add to the end of a string

//     // the string data type requires some memory be allocated on the heap
//     // normally we would have to worry about managing this memory, but with Rust
//     // when the variable goes out of scope the memory is automaticcaly returned

//     // with static types on the stack we can easily make copies like so
//     let x: i32 = 5;
//     let _y: i32 = x;
//     // this is because we define the type and size before compilation
//     // when we try the same with strings, we make whats known as a 'move'
//     // both values point to the same location on the heap
//     let str1: String = String::from("hello");
//     let str2: String = str1;
//     // when we do this, str1 is no longer considered valid. this is done
//     // to prevent a double-free error, an error in which both strings try to
//     // free the same location on the heap
//     // this is why in Rust its known as a 'move' instead of a 'shallow copy';
//     // we would recieve the error that str1's value moved to str2
//     // if we want a deep copy we can do the following:
//     let str3: String = str2.clone();

//     println!("this is str2 {}, or was that str3 {}", str2, str3);

//     /*
//     when a non-copy variable is passed to a function the function takes ownership
//     when a copy variable, such as a simple scalar type, is passed to a function
//     the function makes a copy of the variable allowing you to continue to use the variable
//     after the function call

//     in the case of a non-copy variable, the variable will go out of scope at the end
//     of the function that calls it

//     a function that returns a value can return ownership of a variable moving its scope
//     back to where ever the function was called from

//     Referencing
//     */
//     let mut str1: String = String::from("hello");

//     let len: usize = calc_len(&str1); // reference to str1

//     fn calc_len(str1: &String) -> usize {
//         // we cannot modify this value
//         str1.len()
//     }

//     // there can only be one mutable reference at a time
//     // additionally we cannot have both mutable and immutable references to the
//     // same variable at a time

//     modify_str(&mut str1);

//     fn modify_str(str1: &mut String) {
//         str1.push_str(", world!");
//     }

//     println!("this is length before change: {}", len);
//     println!("this is str1 after change   : {}", str1);

//     // dangling errors - when you reference a value that drops out of scope
//     // the solution in the case of a function call is to simply return the value itself

//     // string slices
//     let str1: String = String::from("Hello, World!");

//     let hello = &str1[..5];
//     let world = &str1[7..];

//     println!("{}, {}", hello, world);

//     // slices also apply to arrays

//     let array: [i32; 5] = [1, 2, 3, 4, 5];

//     let slice = &array[3..];

//     println!("slice of array: {:?}", slice);
// }
