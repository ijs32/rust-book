use std::collections::HashMap;
use std::io;

// solving whiteboarding problems mentioned at end of chapter
// coming back to this
// 1. given a list of integers, use a vector and return the mean, median, and mode
// 2. Convert strings to pig latin
// 3. using a hash map and vectors, create a text interface to allow a user to add employees
// to a department

pub fn main() {
    fn problem_one() {
        let mut vec: Vec<i32> = vec![5, 7, 9, 10, 12, 12, 12, 12, 9, 9, 9];
        {
            {
                let mut total: i32 = 0;

                for num in vec.iter() {
                    total += num;
                }

                let avg = total as f32 / Vec::len(&vec) as f32;
                println!("this is the average: {}", avg)
            }

            {
                vec.sort();

                let median = vec.len() / 2;
                println!("this is the median: {}", median);
            }

            {
                let mut map = HashMap::new();

                for num in vec.iter() {
                    let count = map.entry(num).or_insert(0);
                    *count += 1; // we either insert 0 or receive 0 and add 1 to it
                }

                let mut highest_count: i32 = 0;
                let mut mode: i32 = 0;
                for (key, value) in map.iter() {
                    if value > &highest_count {
                        highest_count = *value;
                        mode = **key;
                    }
                }
                println!("this is mode: {}", mode);
            }
        }
    }

    fn problem_two() {
        let _pig_latin: String = String::from("Helloooo");
        {}
    }

    fn problem_three() {
        let mut departments: HashMap<String, Vec<String>> = HashMap::new();

        loop {
            println!("Type D to add a Department or E to add an employee");

            let mut input: String = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to readline");

            let input: String = match input.trim().parse() {
                Ok(input) => input,
                Err(_) => {
                    println!("Not a word bozo!");
                    continue;
                }
            };

            if &input == &String::from("D") {
                println!("These are the current departments: ");
                for (key, _value) in &departments {
                    println!("{}", key);
                }

                println!("Enter the name for your new department: ");
                let mut new_dept: String = String::new();

                io::stdin()
                    .read_line(&mut new_dept)
                    .expect("Failed to readline");

                let new_dept: String = match new_dept.trim().parse() {
                    Ok(new_dept) => new_dept,
                    Err(_) => {
                        println!("Not a word bozo!");
                        continue;
                    }
                };

                departments.insert(new_dept, vec![]);
            } else if &input == &String::from("E") {
                loop {
                    println!("These are the current departments: ");
                    println!("{:#?}", &departments);

                    println!("Department for new employee?");

                    let mut target_dept: String = String::new();

                    io::stdin()
                        .read_line(&mut target_dept)
                        .expect("Failed to readline");

                    let target_dept: String = match target_dept.trim().parse() {
                        Ok(target_dept) => target_dept,
                        Err(_) => {
                            println!("Not a word bozo!");
                            continue;
                        }
                    };

                    if departments.contains_key(&target_dept) {
                        println!("Employee name?");

                        let mut employee: String = String::new();

                        io::stdin()
                            .read_line(&mut employee)
                            .expect("Failed to readline");

                        let employee: String = match employee.trim().parse() {
                            Ok(employee) => employee,
                            Err(_) => {
                                println!("Not a word bozo!");
                                continue;
                            }
                        };

                        let target_vec = departments.get_mut(&target_dept);

                        let target_vec = match target_vec {
                            Some(inner) => inner,
                            None => {
                                println!("Value is none");
                                continue;
                            }
                        };

                        target_vec.push(employee);
                        break;
                    } else {
                        println!("Enter a valid department bozo!")
                    }
                }
            } else {
                println!("Please enter a valid option.")
            }
        }
    }

    problem_one();
    problem_two();
    problem_three();
}
