/* 参考
https://github.com/snapview/tungstenite-rs/blob/master/examples/client.rs
*/
use tungstenite::{Message, connect};

fn main() {
    let (mut socket, response) =
        connect("ws://localhost:7000/ws").expect("Can't connect to the server");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (header, _value) in response.headers() {
        println!("* {header}");
    }

    socket
        .send(Message::Text("Hello WebSocket".into()))
        .unwrap();
    loop {
        let msg = socket.read().expect("Error reading message");
        println!("Received: {msg}");
    }

    // socket.close(None);
}
