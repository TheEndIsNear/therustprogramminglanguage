#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    New_Mexico,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let m = Message::Write(String::from("hello"));
    m.call();

    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    let five = Some(5);
    let six = add_one(five);
    let none = add_one(None);

    println!("{}", six.unwrap());

    let u8_value = 5u8;
    match u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("sevent"),
        _ => ()
    }

    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    if let(Coin::Quarter(state)) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}


fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
