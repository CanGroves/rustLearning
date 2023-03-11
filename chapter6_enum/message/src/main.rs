#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

/* variants above in strcut way:
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
*/

impl Message {

    fn call(&self) {
        match self {
            Message::Quit => println!("message is: quit"),
            Message::Move{x, y} => println!("message is: move: x: {}, y: {}", x, y),
            Message::Write(v) => println!("message is: write: {}", v),
            Message::ChangeColor(v1, v2, v3) => println!("message is: changeColor: {}, {}, {}", v1, v2, v3),
        }
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
