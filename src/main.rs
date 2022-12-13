mod microldeger;

extern crate ed25519_dalek;
extern crate rand;

use rand::rngs::OsRng;
use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;

use std::io::Read;

const BUFFER_SIZE: usize = 4096;
const QUALITY: u32 = 5; // compression level
const WINDOWS_SIZE: u32 = 22;

pub(crate) fn compress_brotli<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, ()> {
    let mut buf = Vec::new();
    let mut compressor = brotli::CompressorReader::new(input.as_ref(), BUFFER_SIZE, QUALITY, WINDOWS_SIZE);
    compressor.read_to_end(&mut buf);
    Ok(buf)
}

pub(crate) fn decompress_brotli<T: AsRef<[u8]> + ?Sized>(input: &T) -> Result<Vec<u8>, ()> {
    let mut decompressor = brotli::Decompressor::new(input.as_ref(), BUFFER_SIZE);
    let mut buf = Vec::new();
    decompressor
        .read_to_end(&mut buf);
    Ok(buf)
}

fn main() {
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    println!("{:?}", keypair.secret.as_bytes());

    let brotled = compress_brotli(keypair.secret.as_bytes()).unwrap_or_default();
    println!("{:?}", brotled);

    let unbrotled = decompress_brotli(&brotled).unwrap_or_default();
    println!("{:?}", unbrotled);
}
