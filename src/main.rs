use std::f64::consts::PI;
use std::io::Write;
use std::{fs, io};

#[repr(C)]
struct WAVHeader {
    riff: [u8; 4],
    file_length: i32,
    wave: [u8; 4],
    fmt: [u8; 4],
    chunk_size: i32,
    format_tag: i16,
    num_channels: i16,
    sample_rate: i32,
    bytes_per_second: i32,
    bytes_per_sample: i16,
    bits_per_sample: i16,
    data: [u8; 4],
    data_length: i32,
}

const HEADER_LENGTH: i32 = 44;

impl WAVHeader {
    pub const fn new(config: &WAVConfig) -> Self {
        let bytes_per_sample: i32 = ((config.bits_per_sample / 8) * config.num_channels) as i32;
        let buffer_size = config.sample_rate * config.duration_seconds;
        let data_length = buffer_size * bytes_per_sample;

        Self {
            riff: [b'R', b'I', b'F', b'F'],
            wave: [b'W', b'A', b'V', b'E'],
            fmt: [b'f', b'm', b't', b' '],
            data: [b'd', b'a', b't', b'a'],
            file_length: data_length + HEADER_LENGTH,
            chunk_size: config.chunk_size,
            format_tag: config.format_tag,
            num_channels: config.num_channels,
            sample_rate: config.sample_rate,
            bits_per_sample: config.bits_per_sample,
            bytes_per_second: config.sample_rate * bytes_per_sample,
            bytes_per_sample: bytes_per_sample as i16,
            data_length,
        }
    }
}

struct WAVConfig {
    num_channels: i16,
    sample_rate: i32,
    bits_per_sample: i16,
    chunk_size: i32,
    format_tag: i16,
    duration_seconds: i32,
}

const DEFAULT_CONFIG: WAVConfig = WAVConfig {
    num_channels: 1,
    sample_rate: 8000,
    bits_per_sample: 16,
    chunk_size: 16,
    format_tag: 1,
    duration_seconds: 2,
};

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}

const BUFFER_SIZE: usize = (DEFAULT_CONFIG.sample_rate * DEFAULT_CONFIG.duration_seconds) as usize;

fn write_to_file(name: &str, buffer: [i16; BUFFER_SIZE], wavh: WAVHeader) -> io::Result<()> {
    let mut file = fs::OpenOptions::new().create(true).write(true).open(name)?;
    file.write(unsafe { any_as_u8_slice(&wavh) })?;
    file.write(unsafe { any_as_u8_slice(&buffer) })?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() <= 1 {
        println!("USAGE: {} FILE", args[0]);
        std::process::exit(1);
    }
    let wavh = WAVHeader::new(&DEFAULT_CONFIG);
    let mut buffer: [i16; BUFFER_SIZE] = [0; BUFFER_SIZE];
    const FREQ: f64 = 256.0;
    const AMPLITUDE: f64 = 30000.0;
    for i in 0..BUFFER_SIZE {
        buffer[i] = (f64::cos((2.0 * PI * FREQ * i as f64) / DEFAULT_CONFIG.sample_rate as f64)
            * AMPLITUDE) as i16;
    }
    println!(
        "dlength: {}, flength: {}",
        wavh.data_length, wavh.file_length
    );
    write_to_file(&args[1], buffer, wavh)?;

    Ok(())
}
