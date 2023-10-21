use fuse_mt;
use log::debug;
// use std::{io::{self, prelude::*}, fs, os::unix::prelude::FileExt};

mod crypt_fs;

use crypt_fs::{CryptFS, Mode};

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, _metadata: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        println!("{}: {}: {}", record.target(), record.level(), record.args());
    }

    fn flush(&self) {}
}

static LOGGER: ConsoleLogger = ConsoleLogger;


// main function
fn main() -> std::io::Result<()> {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    debug!("Starting up");

    // let key: &[u8; 16] = b"0123456789abcdef";
    // let src_file_path = "plaintext.txt";
    // let dst_file_path = "ciphertext.txt";
    // encrypt_file(key, src_file_path, dst_file_path)?;


    let src_path = "src_dir";
    let mnt_path = "mnt_dir";
    let key = "012345689abcdefg";
    let crypt_fs = CryptFS::new(String::from(key), String::from(src_path), crypt_fs::Mode::Encrypt);

    // // test read file
    // let filepath = Path::new("plaintext.txt");
    // // get attributes
    // let metadata = std::fs::metadata(filepath)?;
    // println!("{:o}", metadata.st_mode());
    // println!("{}", metadata.st_dev());

    // print!("{}", crypt_fs.get_real_path(Path::new("/test.txt")).display());
    fuse_mt::mount(fuse_mt::FuseMT::new(crypt_fs, 1), &mnt_path, &[])?;

    return Ok(());
}


// fn encrypt_file(key: &[u8], src_file_path: &str, dst_file_path: &str) -> std::io::Result<()> {
//     let cipher = Cipher::aes_128_cbc();
//     let pt = fs::read(src_file_path)?;
//     let ct = encrypt(cipher, key, None, &pt)?;

//     let mut enc_file = fs::File::create(dst_file_path)?;
//     enc_file.write_all(&ct)?;

//     return Ok(());
// }
