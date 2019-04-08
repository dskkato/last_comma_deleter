use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter, Write};

fn main() -> io::Result<()> {
    let fname: Vec<String> = env::args().skip(1).collect();
    if fname.len() != 2 {
        panic!("sample SRC OUT");
    };

    let (src, out) = (&fname[0], &fname[1]);
    if src == out {
        panic!("SRC and OUT need to be different names");
    }

    let src = File::open(src)?;
    let reader = BufReader::new(src);

    let out = File::create(out)?;
    let mut writer = BufWriter::new(out);

    for line in reader.lines() {
        let line = line.unwrap();
        let index = line.rfind(',').unwrap();
        writeln!(writer, "{}", &line[..index])?;
    }

    Ok(())
}
