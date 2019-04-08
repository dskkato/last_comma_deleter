use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter, Write};

fn main() -> io::Result<()> {
    let fname: Vec<String> = env::args().skip(1).collect();
    if fname.len() < 2 {
        panic!("sample SRC OUT");
    };

    let (src, out) = (&fname[0], &fname[1]);
    if src == out {
        panic!("SRC and OUT need to be different names");
    }

    let src = File::open(src)?;
    let reader = BufReader::new(src);

    let out = OpenOptions::new()
        .write(true)
        .create(true)
        .open(out)
        .unwrap();
    let mut writer = BufWriter::new(out);

    for line in reader.lines() {
        let line = line.unwrap();
        let index = line.rfind(',').unwrap();
        writer.write_all(line[..index].as_bytes())?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}
