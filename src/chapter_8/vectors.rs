pub fn main() {
    // creating a vector
    let vec: Vec<i32> = Vec::new();
    // Vec<T> can hold any type, so specify when creating one
    let mut vec = vec![1, 2, 3]; // rust is able to infer types if you create a vector with values
                                 // we can use push to add values
    vec.push(4); // vec needs to be mutable in order to use push
    vec.push(5);
    vec.push(6);
    vec.push(7);

    // reading a vectors values
    let third: &i32 = &vec[2];
    println!("this is the third value: {}", third);

    match vec.get(2) {
        Some(third) => println!("this is the third value: {}", third),
        None => println!("there is no third index"),
    }

    let _out_of_bounds = &vec[100]; // this will cause the compiler to panic
    let _out_of_bounds = vec.get(100); // this will return an Option<T> value of none
                                       // iterating over a vector
    for num in &vec {
        println!("current num is: {}", num)
    }

    for num in &mut vec {
        // iterate over a vector and make changes
        *num *= 2
    }

    // using an enum to create vectors able to store multiple types
    enum SpreadSheet {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheet::Int(3),
        SpreadSheet::Text(String::from("Hello, World!")), // this works because everything in this vector is type "SpreadSheet"!
        SpreadSheet::Float(3.14),
    ];
}
