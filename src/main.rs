use clap::{App, Arg};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io;

const BUFF_SZ: usize = 4096;

fn main() -> Result<(), std::io::Error> {
    use std::io::{Read, Write};

    let matches = App::new("ztee is like tee but with gzipped file output")
        .version("0.1.0")
        .author("Jason Dagit <dagitj@gmail.com>")
        .about("Duplicates stdin to stdout and also a compressed file.")
        .arg(
            Arg::with_name("FILE")
                .help("Write compressed stream to FILE")
                .required(true)
                .index(1),
        )
        .get_matches();

    let out_filename = matches.value_of("FILE").expect("FILE required");

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let out = std::fs::File::create(out_filename)?;
    let mut buffer: [u8; BUFF_SZ] = [0; BUFF_SZ];
    let mut encoder = GzEncoder::new(out, Compression::default());
    loop {
        let n = stdin.read(&mut buffer)?;
        match n {
            0 => break, // EOF, such as Ctrl-d causes this
            n => {
                encoder.write_all(&buffer[..n])?;
                stdout.write_all(&buffer[..n])?;
            }
        }
    }
    encoder.finish()?;
    Ok(())
}
