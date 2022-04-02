use std::{net::TcpListener, io::Read, fs::File};

use chrono::Timelike;
use flate2::{write::GzEncoder, Compression};
use tar::{Header, Builder};

fn create_builder() -> Builder<GzEncoder<File>> {
    let millis = chrono::Utc::now().timestamp_millis();
    let n = format!("/mnt/hdd/place/{}.tar.gz", millis);
    let gz = File::create(n);
    let enc = GzEncoder::new(gz.unwrap(), Compression::default());

    return Builder::new(enc);
}

#[tokio::main]
async fn main() {
    let (ctx, crx) = std::sync::mpsc::channel::<String>();
    let (ttx, mut trx) = tokio::sync::mpsc::channel::<(Vec<u8>, String)>(32);

    std::thread::spawn(move || {
        let tl: TcpListener = TcpListener::bind(("127.0.0.1", 8001)).unwrap();
        let (mut stream, addr) = tl.accept().unwrap();
        println!("accepted connection: {}", addr);

        stream.set_read_timeout(None).unwrap();
        loop {
            let mut buf: Vec<u8> = Vec::with_capacity(128);
            buf.resize(128, 0);

            let bytes = stream.read(&mut buf).unwrap();
            if bytes == 0 {
                println!("proxy disconnected");
                return;
            }

            ctx.send(String::from_utf8(buf[..bytes].to_vec()).unwrap()).unwrap();
        }
    });

    std::thread::spawn(move || {
        let mut last_minute = chrono::Utc::now().minute();
        let mut tb = create_builder();

        loop {
            let (res, path) = trx.blocking_recv().unwrap();
            let mut header = Header::new_gnu();
            header.set_path(path).unwrap();
            header.set_username("placeclient").unwrap();
            header.set_mtime(chrono::Utc::now().timestamp() as u64);
            header.set_mode(511);

            header.set_size(res.len() as u64);
            header.set_cksum();

            tb.append(&header, &res.to_vec()[..]).unwrap();

            let current_minute = chrono::Utc::now().minute();
            if current_minute != last_minute {
                tb.finish().unwrap();
                tb = create_builder();
                last_minute = current_minute;

                println!("finished archive!");
            }
        }
    });

    loop {
        let buf = crx.recv().unwrap();

        let ttxc = ttx.clone();
        tokio::spawn(async move {
            println!("go: {}", &buf);
            let res = reqwest::get(&buf)
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap();

            let buftrim = &buf.split("/").last().unwrap();
            ttxc.send((res.to_vec(), buftrim.to_string())).await.unwrap();
        }).await.unwrap();
    }
}
