use std::{net::TcpStream, io::Write, fs::File};

use tungstenite::Message;

const ENDPOINT: &str = "wss://gql-realtime-2.reddit.com/query";

#[tokio::main]
async fn main() {
    /*
    let client = reqwest::Client::new();
    let resp = client.get(format!(
            "https://www.reddit.com/api/v1/access_token?grant_type=password&username={}&password={}",
            include_str!("../username.auth"), include_str!("../password.auth")
        ))
        .basic_auth(
            include_str!("../oauth.auth"),
            Some(include_str!("../secret.auth"))
        ).send().await.unwrap();
    println!("{:?}", resp.text().await.unwrap());
    return;*/

    let (mut ws, _) = tungstenite::connect(ENDPOINT).unwrap();

    // TODO: auth and send connection init message with bearer token
    ws.write_message(Message::Text(format!(
        "{{\"type\": \"connection_init\", \"payload\": {{\"Authorization\": \"{}\"}}}}",
        include_str!("../token.auth")
    ))).unwrap();
    ws.write_message(Message::Text(include_str!("start.json").to_string())).unwrap();

    //let mut tc: TcpStream = TcpStream::connect(("127.0.0.1", 8001)).unwrap();
    let mut log: File = File::create("/mnt/hdd/place.log").unwrap();

    // receive messages over websocket
    loop {
        let msg = ws.read_message().unwrap();
        if let Message::Text(txt) = msg {
            let j = serde_json::to_value(txt).unwrap();
            let data = j.as_object()
                        .unwrap()
                        .get("payload")
                        .unwrap()
                        .as_object()
                        .unwrap()
                        .get("data")
                        .unwrap()
                        .as_object()
                        .unwrap()
                        .get("subscribe")
                        .unwrap()
                        .as_object()
                        .unwrap()
                        .get("data")
                        .unwrap()
                        .as_object()
                        .unwrap();
            
            let typename = data.get("__typename").unwrap().as_str().unwrap();
            match typename {
                "ConfigurationMessageData" => {
                    // TODO
                },
                "DiffFrameMessageData" => {
                    let url = data.get("name").unwrap().as_str().unwrap();
                    let time = data.get("currentTimestamp").unwrap().as_u64().unwrap();
                    //tc.write_all(url.as_bytes()).unwrap();

                    let msg = format!("diff {} {}", time, url);
                    log.write_all(msg.as_bytes()).unwrap();
                    println!("{}", msg);
                },
                "FullFrameMessageData" => {
                    let url = data.get("name").unwrap().as_str().unwrap();
                    let time = data.get("currentTimestamp").unwrap().as_u64().unwrap();
                    //tc.write_all(url.as_bytes()).unwrap();

                    let msg = format!("full {} {}", time, url);
                    log.write_all(msg.as_bytes()).unwrap();
                    println!("{}", msg);
                },
                a => {
                    println!("bad message type: {}", a);
                    log.write_all(format!("bad_message {}", a).as_bytes()).unwrap();
                }
            }
        } else {
            log.write_all(b"bad_message_json").unwrap();
        }
    }
}
