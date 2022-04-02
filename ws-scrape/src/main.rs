use tungstenite::Message;

const ENDPOINT: &str = "wss://gql-realtime.reddit.com/query";

fn main() {
    let (mut ws, _) = tungstenite::connect(ENDPOINT).unwrap();

    // TODO: auth and send connection init message with bearer token
    ws.write_message(Message::Text(include_str!("start.json").to_string())).unwrap();

    // receive messages over websocket
    loop {
        let msg = ws.read_message().unwrap();
        if let Message::Text(txt) = msg {

        } else {
            println!("BAD MESSAGE TYPE");
        }
    }
}
