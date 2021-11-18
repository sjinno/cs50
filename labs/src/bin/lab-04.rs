use cli::Args;
use failure::Error;
use labs::input;

fn main() -> Result<(), Error> {
    let args = cli::process_args()?;
    wav::modify_volume(&args)?;
    Ok(())
}

mod cli {
    use super::Error;

    #[derive(Debug)]
    pub struct Args {
        pub input: String,
        pub output: String,
        pub factor: f32,
    }

    pub fn process_args() -> Result<Args, Error> {
        let num_of_args = std::env::args().skip(1).count();
        if num_of_args != 3 {
            println!(
                "Takes three arguments, but given {} arguments.\n",
                num_of_args
            );
            println!("Usage:   cargo run <input-file> <output-file> <factor>");
            println!("Example: cargo run input.wav output.wav 2.0");
            std::process::exit(1);
        }

        let mut args = std::env::args().skip(1);
        Ok(Args {
            input: args.next().unwrap(),
            output: args.next().unwrap(),
            factor: args.next().unwrap().trim().parse::<f32>()?,
        })
    }
}

mod wav {
    use super::*;

    const HEADER_SIZE: usize = 44;

    pub fn modify_volume(args: &Args) -> Result<(), Error> {
        use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
        use std::fs::File;
        use std::io::{Cursor, Read, Write};

        let Args {
            input,
            output,
            factor,
        } = args;

        // Create the output buffer
        let mut output_buf = File::create(output)?;

        // Open file
        let mut buf_reader = input::get_reader(input)?;

        // Read the first 44 bytes which is the header of the WAV file
        let mut header = vec![0; HEADER_SIZE];
        buf_reader.read_exact(&mut header)?;

        // Write to the output file
        output_buf.write_all(&header)?;

        // Read the rest of the file each sample being 16 bits (2 bytes)
        // and write it to the output file
        let mut samples = Vec::new();
        buf_reader.read_to_end(&mut samples)?;

        let mut reader = Cursor::new(&samples);
        while let Ok(val) = reader.read_i16::<LittleEndian>() {
            output_buf.write_i16::<LittleEndian>(calculate_vol(val, factor))?
        }

        Ok(())
    }

    pub fn calculate_vol(v: i16, factor: &f32) -> i16 {
        ((v as f32) * *factor).round() as i16
    }
}
