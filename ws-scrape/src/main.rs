use std::{net::TcpStream, io::Write, fs::{File, OpenOptions}, collections::HashSet};

use websocket::header::Headers;

const ENDPOINT: &str = "wss://gql-realtime-2.reddit.com/query";
static CLIENT: &str = include_str!("../client.auth");
static SECRET: &str = include_str!("../secret.auth");

#[tokio::main]
async fn main() {
    loop {
        match go().await { 
            Ok(_) => eprintln!("connection terminated gracefully"),
            Err(e) => {
                eprintln!("connection error: {}", e);
                // sleep 10sec if connection fails
                std::thread::sleep(std::time::Duration::from_secs(10));
            }
        };
    }
}

async fn auth() -> Result<String, Box<dyn std::error::Error>> {
    let req = reqwest::Client::new();

    let text = format!(
        "https://www.reddit.com/api/v1/access_token?grant_type=password&username={}&password={}",
        include_str!("../username.auth").to_string().trim_end(),
        include_str!("../password.auth").to_string().trim_end()
    );

    println!("auth endpoint: {}", text);

    let ca = CLIENT.to_string();
    let sa = SECRET.to_string();
    let client = ca.trim_end();
    let secret = sa.trim_end();

    let resp = req.post(text.clone())
        .basic_auth(client, Some(secret)).send().await?;

    let res = resp.text().await?;

    let j = serde_json::from_str::<serde_json::Value>(&res)?;
    let text = j.as_object().ok_or("fail json->as_object")?
        .get("access_token").ok_or("fail json->access_token")?
        .as_str().ok_or("fail json->access_token->as_str")?;

    Ok(text.to_string())
}

async fn go() -> Result<(), Box<dyn std::error::Error>> {
    let mut token = String::from("Bearer ");
    token.push_str(&auth().await?);
    println!("got token: {}", token);

    let mut headers = Headers::new();
    headers.set_raw("Origin", vec![b"https://www.reddit.com".to_vec()]);
    headers.set_raw("Host", vec![b"gql-realtime-2.reddit.com".to_vec()]);

    let mut grabbed: HashSet<String> = HashSet::new();
    let mut configs: HashSet<i64> = HashSet::new();

    let mut ws = websocket::ClientBuilder::new(ENDPOINT)?
        .custom_headers(&headers)
        .connect_secure(None)?;

    let start_time = std::time::Instant::now();

    let t = format!(
        "{{ \"type\": \"connection_init\", \"payload\": {{ \"Authorization\": \"{}\"}} }}",
        token
    );

    println!("sending auth: {}", t);

    ws.send_message(&websocket::Message::text(t))?;
    ws.send_message(&websocket::Message::text(
        include_str!("../../notes/start.json").to_string()
    ))?;

    let mut tc: TcpStream = TcpStream::connect(("127.0.0.1", 8001))?;
    let mut log: File = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/mnt/hdd/place.log")?;
    log.write_all(format!("start session {}\n", chrono::Utc::now().timestamp_millis()).as_bytes())?;

    // receive messages over websocket
    loop {
        if grabbed.len() > 65536 {
            grabbed.clear();
        }

        if std::time::Instant::now().duration_since(start_time).as_secs() > 600 {
            // the connection has been on for a few minutes, let's restart it
            // to get a new full image
            log.write_all(b"restart connection\n")?;
            println!("restart connection");
            return Ok(());
        }

        let msg = ws.recv_message()?;
        if let websocket::OwnedMessage::Text(txt) = msg {
            let j: serde_json::Value = serde_json::from_str(&txt)?;
            match j.get("type") {
                Some(v) => match v.as_str() {
                    Some(v) => match v {
                        "connection_ack" => continue,
                        "connection_error" => return Err(txt.into()),
                        "ka" => continue,
                        "data" => (),
                        a => eprintln!("wtf {}", a)
                    },
                    _ => continue
                },
                _ => continue
            };

            let data = j.as_object().ok_or("fail j->as_object")?
                        .get("payload").ok_or("fail j->payload")?
                        .as_object().ok_or("fail payload->as_object")?
                        .get("data").ok_or("fail payload->data")?
                        .as_object().ok_or("fail 1 data->as_object")?
                        .get("subscribe").ok_or("fail data->subscribe")?
                        .as_object().ok_or("fail subscribe->as_object")?
                        .get("data").ok_or("fail j->data")?
                        .as_object().ok_or("fail 2 data->as_object")?;
            
            let typename = data.get("__typename").ok_or("fail get __typename")?.as_str().ok_or("fail as_str")?;
            match typename {
                "ConfigurationMessageData" => {
                    let canvases = data.get("canvasConfigurations")
                        .ok_or("fail canvasConfigurations")?
                        .as_array().ok_or("fail canvasConfigurations->as_array")?;

                    for i in canvases {
                        let tag = i.get("index")
                            .ok_or("fail canvas->index")?
                            .as_i64().ok_or("fail canvas->as_i64")?;

                        let msg = format!(
                            "canvas {} {} {}\n",
                            tag,
                            i.get("dx").ok_or("fail dx")?.as_i64().ok_or("fail dx->i64")?,
                            i.get("dy").ok_or("fail dy")?.as_i64().ok_or("fail dy->i64")?,
                        );

                        log.write_all(msg.as_bytes())?;

                        if configs.contains(&tag) {
                            continue;
                        }

                        configs.insert(tag);
                        let mut msg = include_str!("../../notes/canvas.json").to_string();
                        msg = msg.replace("PUTTHETAGHERE", &tag.to_string());
                        ws.send_message(&websocket::Message::text(msg))?;
                    }
                },
                "DiffFrameMessageData" => {
                    let url = data.get("name").ok_or("diff: fail name")?
                        .as_str().ok_or("diff: fail name->as_str")?.to_string();

                    if grabbed.contains(&url) {
                        println!("dupe {}", url);
                        continue;
                    }

                    grabbed.insert(url.clone());

                    let mut url_bytes = url.as_bytes().to_vec();
                    url_bytes.push(b'\n');
                    
                    let time = data.get("currentTimestamp")
                        .ok_or("diff: fail currentTimestamp")?
                        .as_f64().ok_or("diff: fail currentTimestamp->as_f64")?;

                    if url.ends_with("image") { continue; }
                    tc.write_all(&url_bytes)?;

                    let msg = format!("diff {} {}\n", time, url.split("/").last().ok_or("diff fail url split")?);
                    log.write_all(msg.as_bytes())?;
                    println!("{}", msg.trim_end());
                },
                "FullFrameMessageData" => {
                    let url = data.get("name").ok_or("full: fail name")?
                        .as_str().ok_or("full: fail name->as_str")?.to_string();

                    if grabbed.contains(&url) {
                        println!("dupe {}", url);
                        continue;
                    }

                    grabbed.insert(url.clone());

                    let mut url_bytes = url.as_bytes().to_vec();
                    url_bytes.push(b'\n');

                    let time = data.get("timestamp")
                        .ok_or("full: fail timestamp")?
                        .as_f64().ok_or("full: fail timestamp->as_f64")?;

                    if url.ends_with("image") { continue; }
                    tc.write_all(&url_bytes)?;

                    let msg = format!("full {} {}\n", time, url.split("/").last().ok_or("full fail url split")?);
                    log.write_all(msg.as_bytes())?;
                    println!("{}", msg.trim_end());
                },
                a => {
                    eprintln!("bad message type: {}", a);
                    log.write_all(format!("bad_message {}\n", a).as_bytes())?;
                }
            }
        } else {
            log.write_all(b"bad_message_json\n")?;
        }
    }
}
