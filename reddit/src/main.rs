use std::{fs::{self, OpenOptions, File}, io::{BufReader, BufRead, Write, Read}, collections::HashMap, u128};

use byteorder::ReadBytesExt;
use chrono::NaiveDateTime;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("not enough args");
    }

    match args[1].as_str() {
        "decode" => decode_diffs(&args[2]),
        "encode" => encode_diffs(&args[2], &args[3]),
        _ => println!("use encode or decode")
    };
}

#[inline(always)]
fn get_u128(buf: &[u8; 16]) -> u128 {
    let mut r: &[u8] = &buf[..];
    r.read_u128::<byteorder::LittleEndian>().unwrap()
}

fn decode_diffs(inpath: &str) {
    let mut infile = fs::File::open(inpath).unwrap();
    let mut buf: [u8; 16] = [0; 16];

    loop {
        if infile.read(&mut buf).unwrap() == 0 { break; }
        let diff = get_u128(&buf);

        if diff & (1 << 127) == 0 {
            println!("x: {} y: {} color: {} time: {} name: {}",
                (diff >> 116) & 0x7FF,
                (diff >> 105) & 0x7FF,
                (diff >> 78) & 0x1F,
                (diff >> 36) & 0xFFFFFFFF,
                (diff >> 4) & 0xFFFFFFFF
            );
        } else {
            println!("x: {} y: {} x2: {} y2: {} color: {} time: {} name: {}",
                (diff >> 116) & 0x7FF,
                (diff >> 105) & 0x7FF,
                (diff >> 94) & 0x7FF,
                (diff >> 83) & 0x7FF,
                (diff >> 78) & 0x1F,
                (diff >> 36) & 0xFFFFFFFF,
                (diff >> 4) & 0xFFFFFFFF
            );
        }
    }
}

fn encode_diffs(inpath: &str, outpath: &str) {
    let infile = fs::File::open(inpath).unwrap();
    let mut outfile = OpenOptions::new().write(true).open(outpath).unwrap();

    let mut names: HashMap<String, u32> = HashMap::new();
    let colors: HashMap<&str, u32> = HashMap::from([
        ( "#6D001A", 0 ),
        ( "#BE0039", 1 ),
        ( "#FF4500", 2 ),
        ( "#FFA800", 3 ),
        ( "#FFD635", 4 ),
        ( "#FFF8B8", 5 ),
        ( "#00A368", 6 ),
        ( "#00CC78", 7 ),
        ( "#7EED56", 8 ),
        ( "#00756F", 9 ),
        ( "#009EAA", 10 ),
        ( "#00CCC0", 11 ),
        ( "#2450A4", 12 ),
        ( "#3690EA", 13 ),
        ( "#51E9F4", 14 ),
        ( "#493AC1", 15 ),
        ( "#6A5CFF", 16 ),
        ( "#94B3FF", 17 ),
        ( "#811E9F", 18 ),
        ( "#B44AC0", 19 ),
        ( "#E4ABFF", 20 ),
        ( "#DE107F", 21 ),
        ( "#FF3881", 22 ),
        ( "#FF99AA", 23 ),
        ( "#6D482F", 24 ),
        ( "#9C6926", 25 ),
        ( "#FFB470", 26 ),
        ( "#000000", 27 ),
        ( "#515252", 28 ),
        ( "#898D90", 29 ),
        ( "#D4D7D9", 30 ),
        ( "#FFFFFF", 31 )
    ]);

    let inread = BufReader::new(infile);

    for line in inread.lines().skip(1) {
        let l = line.unwrap();
        let res = read(&l, &mut outfile, &colors, &mut names);

        if res.is_none() {
            panic!("{}", l);
        }
    }

    outfile.flush().unwrap();
}

#[inline]
fn read(line: &str, file: &mut File, colors: &HashMap<&str, u32>, names: &mut HashMap<String, u32>) -> Option<()> {
    const START: i64 = 1648810000000;
    let len = names.len() as u32;

    let splits: Vec<&str> = line.split(',').collect();
    let time = NaiveDateTime::parse_from_str(splits[0], "%Y-%m-%d %H:%M:%S%.3f UTC").unwrap();
    let name = names.entry(splits[1].to_string()).or_insert(len);
    let color = *colors.get(splits[2])?;

    if splits.len() == 5 {
        let x = splits[3].strip_prefix('"')?.parse::<u16>().unwrap();
        let y = splits[4].strip_suffix('"')?.parse::<u16>().unwrap();

        let mut diff: u128 = 0;

        diff |= (x as u128 & 0xFFF) << 116;
        diff |= (y as u128 & 0xFFF) << 105;
        diff |= (color as u128) << 78;
        diff |= (time.timestamp_millis() as u128 - START as u128) << 36;
        diff |= (*name as u128) << 4;
        file.write(&diff.to_le_bytes()).unwrap();
    } else {
        let x = splits[3].strip_prefix('"')?.parse::<u16>().unwrap();
        let y = splits[4].parse::<u16>().unwrap();
        let i = splits[5].parse::<u16>().unwrap();
        let j = splits[6].strip_suffix('"')?.parse::<u16>().unwrap();

        let mut diff: u128 = 1 << 127;

        diff |= (x as u128 & 0xFFF) << 116;
        diff |= (y as u128 & 0xFFF) << 105;
        diff |= (i as u128 & 0xFFF) << 94;
        diff |= (j as u128 & 0xFFF) << 83;
        diff |= (color as u128) << 78;
        diff |= (time.timestamp_millis() as u128 - START as u128) << 36;
        diff |= (*name as u128) << 4;
        file.write(&diff.to_le_bytes()).unwrap();
    }

    Some(())
}
