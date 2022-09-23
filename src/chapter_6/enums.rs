pub fn enums() {
    // enum IpAddrKind {
    //     v4,
    //     v6,
    // }

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String
    // }

    // let home = IpAddr {         // we can do this
    //     kind: IpAddrKind::v4,
    //     address: String::from("127.0.0.1")
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::v6,
    //     address: String::from("::1")
    // };

    // let four = IpAddrKind::v4;
    // let six  = IpAddrKind::v6;

    enum IpAddr {
        v4(u8, u8, u8, u8),             // or we can do this
        v6(String),
    }

    let home = IpAddr::v4(127, 0, 0, 1);
    let loopback  = IpAddr::v6(String::from("::1"));

    fn route(ip: IpAddr) -> IpAddr {
        ip
    }

    route(home);
    route(loopback);

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("this is message: {:#?}", self);
        }
    }

    let m = Message::Write(String::from("Hello"));
    m.call();

    let number = Some(5); // the Option<T> enum used to check if a value is some or none
                                       // this is Rust's implementation of 'null'

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        Arkansas, // this might be wrong lol
    }
    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    impl Coin {
        fn match_coin(&self) -> u8 {
            match self {
                Coin::Penny   => {
                    println!("Oooooh lucky penny");
                    1
                },
                Coin::Nickel  => 5,
                Coin::Dime    => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:#?}", state);    
                    25
                },
            }
        }
        fn count_non_quarters(&self, &count: &i32) -> i32 {
            match self{
                Coin::Quarter(state) => {
                    println!("State Quarter from {:#?}", state);
                    count
                }
                _ => count + 1
            }
        }
    }

    let penny = Coin::Penny;
    let alaskan_quarter = Coin::Quarter(UsState::Alaska);
    let coin_value = penny.match_coin();
    println!("this is the value of {:#?}: {}", penny, coin_value);
    alaskan_quarter.match_coin();

    // using the option<T> enum with matching fn

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(num) => Some(num + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // we can use the _ => operator to match all things we dont care about

    // if let control flow

    let some_val = Some(0u8);
    if let Some(3) = some_val {
        println!("3!");
    }
                                    // these two are the same, if let is more concise in scenarios
                                    // where you only need to check one possibility
    match some_val {
        Some(3) => println!("3!"),
        _ => (),
    }

    let mut count = 0;
    count = alaskan_quarter.count_non_quarters(&count);
    count = penny.count_non_quarters(&count);

    println!("this is count: {}", count);

    if let Coin::Quarter(state) = alaskan_quarter {
        println!("its a quarter")
    } else {  // obviously this would make more sense in a function but you get the idea
        count += 1; // this is the alternative to the match statement found int 'count_non_quarters' impl function
    }
}