// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Vector {
    x: u16,
    y: u16
}

#[derive(Debug)]
enum Message {
    //Move { x: u16, y: u16 },
    Move(Vector),
    Echo(String),
    ChangeColor(u16, u16, u16),
    Quit

}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let v = Vector { x: 2, y: 3 };
    let messages = [
        Message::Move(Vector{x: 10, y: 30 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
