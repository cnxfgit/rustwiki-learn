#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route(ip_type: IpAddrKind) {
    match ip_type {
        IpAddrKind::V4 => println!("V4"),
        IpAddrKind::V6 => println!("V6"),
    }
}

fn addr() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("{:?} {:?}", home, home.kind);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:?}", loopback);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Self::Quit => println!("Quit"),
            _ => println!("other"),
        }
    }
}

fn option_func() {
    let x: i8 = 5;
    let y: Option<i32> = Some(5);
    let m = Message::Quit;
    m.call();

    println!("{} {} {}", x, plus_one(y), value_in_cents(Coin::Dime));

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("three");
    }

    
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> i32 {
    match x {
        None => 0,
        Some(i) => i + 1,
    }
}

fn main() {
    addr();
    option_func();
}
