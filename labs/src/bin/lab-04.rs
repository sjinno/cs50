use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use failure::Error;
use labs::input;
use std::fs::File;
use std::io::{Cursor, Read, Write};

const HEADER_SIZE: usize = 44;

fn main() -> Result<(), Error> {
    // Create the output buffer
    let mut output_buf = File::create("audio/output.wav")?;
    let factor = 2.5;

    // Open file
    let mut buf_reader = input::get_reader("audio/input.wav")?;

    // Read the first 44 bytes which is the header of the WAV file
    let mut header = vec![0; HEADER_SIZE];
    buf_reader.read_exact(&mut header)?;

    // Write to the output file
    output_buf.write_all(&header)?;

    // Read the rest of the file each sample being 16 bits (2 bytes)
    // and write it to the output file
    let mut samples = Vec::new();
    buf_reader.read_to_end(&mut samples)?;
    eprintln!("lenth of samples {}", samples.len());

    let mut reader = Cursor::new(&samples);
    while let Ok(val) = reader.read_i16::<LittleEndian>() {
        output_buf.write_i16::<LittleEndian>(adjust_volume(val, factor))?
    }

    Ok(())
}

fn adjust_volume(v: i16, factor: f32) -> i16 {
    ((v as f32) * factor).round() as i16
}
