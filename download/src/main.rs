use std::{net::TcpListener, io::{BufReader, BufRead}, fs::File};

use flate2::{write::GzEncoder, Compression};
use tar::{Header, Builder};

fn create_builder() -> Builder<GzEncoder<File>> {
    let millis = chrono::Utc::now().timestamp_millis();
    let n = format!("/mnt/hdd/place2/{}.tar.gz", millis);
    let gz = File::create(n);
    let enc = GzEncoder::new(gz.unwrap(), Compression::default());

    return Builder::new(enc);
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let (ctx, crx) = std::sync::mpsc::channel::<String>();
    let (ttx, mut trx) = tokio::sync::mpsc::channel::<(Vec<u8>, String)>(32);
    let (ntx, mut nrx) = tokio::sync::mpsc::channel::<isize>(32);

    std::thread::spawn(move || {
        let mut count = 0;
        loop {
            let r = nrx.blocking_recv().unwrap();
            count += r;
            println!("buf size: {}", count);
        }
    });

    let ntxc = ntx.clone();
    std::thread::spawn(move || {
        'outer: loop {
            // sleep if connection error
            std::thread::sleep(std::time::Duration::from_secs(2));
            println!("starting conn");

            let tl: TcpListener = match TcpListener::bind(("127.0.0.1", 8001)) {
                Ok(v) => v,
                Err(_) => continue
            };

            let (stream, addr) = match tl.accept() {
                Ok(v) => v,
                Err(_) => continue
            };

            stream.set_read_timeout(Some(std::time::Duration::from_secs(10))).unwrap();

            println!("accepted connection: {}", addr);

            if stream.set_read_timeout(None).is_err() { continue; }
            let mut buf = String::new();
            let mut br = BufReader::new(stream);

            loop {
                let bytes = match br.read_line(&mut buf) {
                    Ok(v) => v,
                    Err(_) => continue 'outer
                };

                if bytes > 0 {
                    ctx.send(String::from_utf8(buf.trim_end().as_bytes().to_vec()).unwrap()).unwrap();
                    ntxc.blocking_send(1).unwrap();
                } else {
                    continue 'outer
                }

                buf.clear();
            }
        }
    });

    std::thread::spawn(move || {
        let mut in_tar = 0;
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

            in_tar += 1;

            if in_tar > 1024 {
                tb.finish().unwrap();
                tb = create_builder();
                in_tar = 0;

                println!("finished archive!");
            }
        }
    });

    loop {
        let mut tasks = Vec::new();
        for _ in 0 .. 16 {
            let buf = crx.recv().unwrap();

            let ttxc = ttx.clone();
            let ntxc = ntx.clone();
            tasks.push(tokio::spawn(async move {
                println!("go: {}", &buf);
                let res = reqwest::get(&buf)
                    .await
                    .unwrap()
                    .bytes()
                    .await
                    .unwrap();

                let buftrim = &buf.split("/").last().unwrap();
                ttxc.send((res.to_vec(), buftrim.to_string())).await.unwrap();
                ntxc.send(-1).await.unwrap();
            }));
        }

        futures::future::join_all(tasks).await;
    }
}
