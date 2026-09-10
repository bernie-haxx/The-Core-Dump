#[derive(Debug)]
enum IPAddrKind {
    V4,
    V6,
}

#[allow(dead_code)]
enum IPAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;

    route(four);
    route(six);

    let _home = IPAddr::V4(127, 0, 0, 1);
    let _loopback = IPAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let _some_number = Some(5);
    let _some_char = Some('e');

    let absent_number: Option<i32> = None;

    println!("{absent_number:?}");
}

fn route(_ip_kind: IPAddrKind) {}
