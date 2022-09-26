use std::collections::HashMap;

pub fn main() {
    // HashMap<K, V>
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // its important to note that hash maps take ownership of values placed in them
    let blue_scores = scores.get("Blue");
    println!("this is the blue teams score: {:#?}", blue_scores); // keep in mind this will return a Some() Type 

    for (key, value) in &scores {
        println!("{} : {}", key, value);
    }

    // if we insert a value on a key that already has a value, the previously held value will be overwritten
    scores.insert("Blue".to_string(), 25); // blue score is now 25 instead of 10
    // to check if a key has a value assigned we can use .entry
    scores.entry(String::from("Blue")).or_insert(45); // this will do nothing as blue already has a value

    let text: String = String::from("HellO world world hey world");

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // we either insert 0 or receive 0 and add 1 to it
    }

    

}