use std::{net::TcpStream, io::Write, fs::{File, OpenOptions}};

use websocket::header::Headers;

const ENDPOINT: &str = "wss://gql-realtime-2.reddit.com/query";

#[tokio::main]
async fn main() {
    loop {
        match go().await { _ => () };

        // sleep 10sec if connection fails
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}

async fn go() -> Result<(), Box<dyn std::error::Error>> {
    /*
    let client = reqwest::Client::new();
    let resp = client.post(format!(
            "https://www.reddit.com/api/v1/access_token?grant_type=password&username={}&password={}",
            include_str!("../username.auth"), include_str!("../password.auth")
        ))
        .basic_auth(
            include_str!("../oauth.auth"),
            Some(include_str!("../secret.auth"))
        ).send().await.unwrap();
    println!("{:?}", resp.text().await.unwrap());
    return;*/

    let mut headers = Headers::new();
    headers.set_raw("Origin", vec![b"https://www.reddit.com".to_vec()]);
    headers.set_raw("Host", vec![b"gql-realtime-2.reddit.com".to_vec()]);

    let mut ws = websocket::ClientBuilder::new(ENDPOINT)
        .unwrap()
        .custom_headers(&headers)
        .connect_secure(None)
        .unwrap();

    let t = format!(
        "{{ \"type\": \"connection_init\", \"payload\": {{ \"Authorization\": \"{}\"}} }}",
        include_str!("../token.auth").strip_suffix("\n").unwrap());

    ws.send_message(&websocket::Message::text(t)).unwrap();

    ws.send_message(&websocket::Message::text(
            include_str!("start.json").to_string()
    )).unwrap();

    let mut tc: TcpStream = TcpStream::connect(("127.0.0.1", 8001)).unwrap();
    let mut log: File = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/mnt/hdd/place.log").unwrap();
    log.write_all(format!("start session {}\n", chrono::Utc::now().timestamp_millis()).as_bytes()).unwrap();

    // receive messages over websocket
    loop {
        let msg = ws.recv_message()?;
        if let websocket::OwnedMessage::Text(txt) = msg {
            let j: serde_json::Value = serde_json::from_str(&txt).unwrap();
            match j.get("type") {
                Some(v) => match v.as_str() {
                    Some(v) => match v {
                        "connection_ack" => continue,
                        "connection_error" => return Ok(()),
                        "ka" => continue,
                        "data" => (),
                        a => println!("wtf {}", a)
                    },
                    _ => continue
                },
                _ => continue
            };

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
                    let canvases = data.get("canvasConfigurations")
                        .unwrap()
                        .as_array()
                        .unwrap();

                    for i in canvases {
                        let tag = i.get("index").unwrap().as_i64().unwrap();
                        let mut msg = include_str!("canvas.json").to_string();
                        msg = msg.replace("PUTTHETAGHERE", &tag.to_string());
                        ws.send_message(&websocket::Message::text(msg)).unwrap();

                        let msg = format!(
                            "canvas {} {} {}\n",
                            tag,
                            i.get("dx").unwrap().as_i64().unwrap(),
                            i.get("dy").unwrap().as_i64().unwrap(),
                        );

                        log.write_all(msg.as_bytes()).unwrap();
                    }
                },
                "DiffFrameMessageData" => {
                    let mut url = data.get("name").unwrap().as_str().unwrap().to_string();
                    url.push('\n');
                    
                    let time = data.get("currentTimestamp").unwrap().as_f64().unwrap();

                    if url.ends_with("image") { continue; }
                    tc.write_all(url.as_bytes()).unwrap();

                    let msg = format!("diff {} {}", time, url.split("/").last().unwrap());
                    log.write_all(msg.as_bytes()).unwrap();
                    println!("{}", msg.strip_suffix("\n").unwrap());
                },
                "FullFrameMessageData" => {
                    let mut url = data.get("name").unwrap().as_str().unwrap().to_string();
                    url.push('\n');

                    let time = data.get("timestamp").unwrap().as_f64().unwrap();

                    if url.ends_with("image") { continue; }
                    tc.write_all(url.as_bytes()).unwrap();

                    let msg = format!("full {} {}", time, url.split("/").last().unwrap());
                    log.write_all(msg.as_bytes()).unwrap();
                    println!("{}", msg.strip_suffix("\n").unwrap());
                },
                a => {
                    println!("bad message type: {}", a);
                    log.write_all(format!("bad_message {}\n", a).as_bytes()).unwrap();
                }
            }
        } else {
            log.write_all(b"bad_message_json\n").unwrap();
        }
    }
}
