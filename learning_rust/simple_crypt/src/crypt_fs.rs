use std::path::{Path, PathBuf};
use std::os::unix::prelude::FileExt;
use std::fs;
use openssl::symm::{Cipher, encrypt, decrypt};
use openssl::hash::{hash, MessageDigest};
use sha2::Sha256;
use hmac::{Hmac, Mac};

#[allow(unused_imports)]
use log::{debug, info, error};

// const BLOCK_SIZE: usize = 16;
const AES_128_KEY_SIZE: usize = 16;
const AES_256_KEY_SIZE: usize = 32;
const AES_BLOCK_SIZE: usize = 16;
const HEADER_SIZE: usize = AES_BLOCK_SIZE*3;            // must be multiple of AES_BLOCK_SIZE, else first block will be corrupted during decryption
const MAC_OFFSET: usize = 0;
const MAC_SIZE: usize = AES_BLOCK_SIZE*2;               // size of a SHA256 digest
const ORIG_SIZE_OFFSET: usize = AES_BLOCK_SIZE*2;       // siz of original file
const ORIG_SIZE_SIZE: usize = 8;                        // size of original file size

type HmacSha256 = Hmac<Sha256>;

mod fuse;

#[derive(Debug)]
pub enum Mode {
    Encrypt,
    Decrypt,
}

pub struct CryptFS   {
    cipher: Cipher,
    key: String,
    src_dir: PathBuf,
    mode: Mode,
}

impl CryptFS {

    pub fn new(key: String, src_dir_path: String, mode: Mode) -> Self {
        // check directory exists
        if !fs::metadata(src_dir_path.clone()).is_ok() {
            error!("Directory does not exist");
            panic!();
        }

        let src_dir = PathBuf::from(src_dir_path).canonicalize().unwrap();

        let cipher: Cipher;
        if key.as_bytes().len() == AES_128_KEY_SIZE {
            cipher = Cipher::aes_128_cbc();
        } else if key.as_bytes().len() == AES_256_KEY_SIZE {
            cipher = Cipher::aes_256_cbc();
        } else {
            panic!("Invalid key size");
        }
    
        return CryptFS {
            cipher: cipher,
            key: key,
            src_dir: src_dir,
            mode: mode,
        };
    }

    fn get_real_path(&self, path: &Path) -> PathBuf {
        let mut real_path = self.src_dir.clone();
        real_path.push(path.strip_prefix("/").unwrap());
        return real_path;
    }

    fn read_exact_at(&self, file: &fs::File, size: u64, offset: u64) -> Result<Vec<u8>,()> {
        let mut buf = Vec::<u8>::with_capacity(size as usize);
        unsafe { buf.set_len(size as usize) };

        match fs::File::read_exact_at(&file, &mut buf, offset) {
            Ok(_) => return Ok(buf),
            Err(_) => return Err(()),
        }
    }

    fn compute_sha256_hmac(&self, data: &[u8]) -> Vec<u8> {
        let mut mac = HmacSha256::new_from_slice(self.key.as_bytes()).unwrap();
        mac.update(data);
        let mac = mac.finalize().into_bytes();
        return mac.to_vec();
    }
    

    fn get_read_size(&self, file: &fs::File) -> u64 {
        let mut new_size : u64 = 0;
        let file_size = file.metadata().unwrap().len();

        if file_size == 0 {
            return new_size;
        }

        match self.mode {
            Mode::Encrypt => {
                let aes_padding = AES_BLOCK_SIZE as u64 - (file_size % AES_BLOCK_SIZE as u64);
                new_size = HEADER_SIZE as u64 + file_size + aes_padding;
            },
            Mode::Decrypt => {
                if file_size < HEADER_SIZE as u64 {
                    new_size = file_size; // if this happens, the file is corrupted, we will deal with when reading
                } else {
                    let orig_size = self.read_exact_at(file, 8, MAC_OFFSET as u64).unwrap();
                    // convert vector of 8 bytes to u64
                    new_size = u64::from_be_bytes(orig_size[..].try_into().unwrap());
                }
            }
        }

        return new_size;
    }

    #[allow(dead_code)]
    fn read_encrypt(&self, file: &fs::File) -> Result<Vec<u8>,()> {
        let file_size = file.metadata().unwrap().len();

        if file_size == 0 {
            return Ok(vec![]);
        }

        let buf_size = file_size + HEADER_SIZE as u64;
       
        // force vector to report correct size
        let mut buf = Vec::<u8>::with_capacity(buf_size as usize);
        unsafe { buf.set_len(buf_size as usize) };

        file.read_exact_at(&mut buf[HEADER_SIZE..], 0).unwrap();    // read the whole file

        // get iv from file hash, this makes it repeatable for the same file
        let iv = &hash(MessageDigest::md5(), &buf[HEADER_SIZE..]).unwrap()[..];

        // encrypt the file
        let mut enc_buf = encrypt(self.cipher, self.key.as_bytes(), Some(iv), &buf).unwrap();

        // add original file size to header
        enc_buf[ORIG_SIZE_OFFSET..ORIG_SIZE_OFFSET+ORIG_SIZE_SIZE].copy_from_slice(&(file_size as u64).to_be_bytes());

        // compute mac, include original file size
        let mac = self.compute_sha256_hmac(&enc_buf[ORIG_SIZE_OFFSET..]);

        // add mac to header, 
        enc_buf[MAC_OFFSET..MAC_OFFSET+MAC_SIZE].copy_from_slice(&mac.as_slice());

        return Ok(enc_buf);
        
    }

    #[allow(dead_code)]
    fn read_decrypt(&self, file: &fs::File) -> Result<Vec<u8>,()> {
        let enc_size = file.metadata().unwrap().len();

        if enc_size == 0 {
            return Ok(vec![]);
        }

        // force vector to report correct size
        let mut buf = Vec::<u8>::with_capacity(enc_size as usize);
        unsafe { buf.set_len(enc_size as usize) };

        file.read_exact_at(&mut buf, 0).unwrap();    // read the whole file

        let file_mac = &buf[MAC_OFFSET..MAC_OFFSET+MAC_SIZE];
        let computed_mac = self.compute_sha256_hmac(&buf[ORIG_SIZE_OFFSET..]);

        for i in 0..MAC_SIZE {
            if file_mac[i] != computed_mac[i] {
                error!("MAC mismatch");
                return Err(());
            }
        }

        let orig_size = u64::from_be_bytes(buf[ORIG_SIZE_OFFSET..ORIG_SIZE_OFFSET+8].try_into().unwrap());

        let dec_buf = decrypt(self.cipher, self.key.as_bytes(), None, &buf[ORIG_SIZE_OFFSET..]).unwrap();

        // remove first AES Block, it is corrupted during decryption
        let mut dec_buf = &dec_buf[AES_BLOCK_SIZE..];

        // remove padding from last block
        dec_buf = &dec_buf[..orig_size as usize];

        return Ok(dec_buf.to_vec());

    }


}

