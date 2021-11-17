use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use failure::Error;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Write};

const HEADER_SIZE: usize = 44;

fn main() -> Result<(), Error> {
    let factor = 2.0;

    // Open file
    let file = File::open("audio/input.wav")?;
    let mut buf_reader = BufReader::new(file);
    let mut data = Vec::new();
    let _ = buf_reader.read_to_end(&mut data)?;

    // Read the first 44 bytes which is the header of the WAV file
    let header = &data[..HEADER_SIZE];
    println!("{:?}", header);
    eprintln!("{}", String::from_utf8_lossy(&header));

    // Read the rest of the file each sample being 16 bits (2 bytes)
    // and write it to the output file
    eprintln!("{}", &data[44..].len());
    // eprintln!("{:?}", &data[44..4044]);
    let mut samples = Vec::new();
    let mut rdr = Cursor::new(&data[44..]);
    loop {
        match rdr.read_i16::<LittleEndian>() {
            Ok(v) => samples.write_i16::<LittleEndian>(adjust_volume(v, factor))?,
            Err(_) => break,
        }
    }
    // eprintln!("{:?}", &samples[..4000]);
    eprintln!("{}", samples.len());

    let mut output_buf = File::create("audio/output.wav")?;
    let mut header = header.to_owned();
    header.extend_from_slice(&samples);
    let bytes = output_buf.write(&header)?;
    eprintln!("bytes written to file: {}", bytes);

    Ok(())
}

fn adjust_volume(v: i16, factor: f32) -> i16 {
    ((v as f32) * factor).round() as i16
}
