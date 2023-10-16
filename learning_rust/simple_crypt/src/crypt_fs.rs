use std::ffi::OsStr;
use std::io::{self, prelude::*};
use std::mem::forget;
use std::os::fd::{IntoRawFd, FromRawFd, RawFd};
use std::os::unix::prelude::FileExt;
use std::{fs, os::linux::fs::MetadataExt};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use fuse_mt::*;
use openssl::symm::{Cipher, encrypt};
use openssl::hash::{hash, MessageDigest};
use sha2::Sha256;
use hmac::{Hmac, Mac};
use log::{debug, error};
use libc;

// const BLOCK_SIZE: usize = 16;
const AES_128_KEY_SIZE: usize = 16;
const AES_256_KEY_SIZE: usize = 32;
const AES_BLOCK_SIZE: usize = 16;
const HEADER_SIZE: usize = AES_BLOCK_SIZE*2;      // must be >= AES_BLOCK_SIZE, 32 = SHA256 digest size
const TTL: Duration = Duration::from_secs(1);

type HmacSha256 = Hmac<Sha256>;


pub struct CryptFS   {
    cipher: Cipher,
    key: String,
    src_dir: PathBuf,
}

impl CryptFS {

    pub fn new(key: String, src_dir_path: String) -> Self {
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
        };
    }

    /// read file with offset and size, but will make sure to be block alligned and read whole blocks
    /// returns a tuple with the data and the sice of the data to read (start and end)
    // fn read_file_blocks(file: &std::fs::File, offset: u64, size: u32) -> Result<(Vec<u8>, usize, usize),()>    {
    //     let file_size = file.metadata().unwrap().len();
    //     let mut size = size as u64;     // avoid converting to u64 multiple times

    //     if size > file_size {
    //         size = file_size;
    //     }

    //     if offset >= file_size {
    //         return Err(());    // read past the end of the file
    //     }

    //     let block_offset = offset % (AES_BLOCK_SIZE as u64);
    //     let mut new_size: u64 = size + (size % AES_BLOCK_SIZE as u64) + block_offset;     // adjust size for block allignment, on both sides
    //     let new_offset: u64 = if offset > block_offset { offset - block_offset } else { 0 };    // if in the middle of the first block, read from the beginning of the file

    //     // bounds check
    //     if new_offset + new_size > file_size {
    //         new_size = file_size - new_offset;
    //     }

    //     // read just the requested bytes
    //     let mut buf = Vec::<u8>::with_capacity(new_size as usize);
    //     unsafe { buf.set_len(new_size as usize) };              // set length so the read_exact_at will read everthing into the buffer

    //     file.read_exact_at(buf.as_mut_slice(), new_offset).unwrap();

    //     return Ok((buf, (new_offset - offset) as usize, (offset + size) as usize));

    // }

    fn check_path(&self, path: &Path) -> Result<(), libc::c_int> {
        if (path.is_dir()|| path.is_file()) && !path.is_symlink() {
            // check if file a regular file or directory
            return Ok(());
        } else {
            return Err(libc::ENOENT);
        }
    }

    fn check_dir(&self, path: &Path) -> Result<(), libc::c_int> {
        if path.is_dir() && !path.is_symlink() {
            // check if file a regular file or directory
            return Ok(());
        } else {
            return Err(libc::ENOTDIR);
        }
    }

    fn get_real_path(&self, path: &Path) -> PathBuf {
        let mut real_path = self.src_dir.clone();
        real_path.push(path.strip_prefix("/").unwrap());
        return real_path;
    }

    // // This function assumes the path exists
    // fn check_permissions(&self, _req: RequestInfo, _path: &Path, _mask:u16) -> Result<(), libc::c_int> {

    //     let metadata = match fs::metadata(_path) {
    //         Ok(metadata) => metadata,
    //         Err(_) => return Err(libc::ENOENT),
    //     };

    //     let file_mode = metadata.st_mode() as u16;
    //     let file_uid = metadata.st_uid();
    //     let file_gid = metadata.st_gid();

    // }

    // fn check_ro_permissions(&self, _req: RequestInfo, _path: &Path, _fh: Option<u64>) -> Result<(), libc::c_int> {
    //     // check if file is read only
    //     return self.check_permissions(_req, _path, _fh, false);
    // }

}


impl FilesystemMT for CryptFS {
    fn init(&self, _req: RequestInfo) -> ResultEmpty {
        debug!("init() called");
        return Ok(());
    }

    fn destroy(&self) {
        debug!("destroy() called");
    }

    fn getattr(&self, _req: RequestInfo, _path: &Path, _fh: Option<u64>) -> ResultEntry {
        debug!("getattr() called");
        let real_path = self.get_real_path(_path);
        self.check_path(&real_path)?;       // don't follow symlinks, or special files

        match fs::metadata(real_path) {
            Ok(metadata) => {
                let size = metadata.len();
                let mut new_size = size;

                if metadata.is_file() && size > 0 {
                    let aes_padding = AES_BLOCK_SIZE as u64 - (size % AES_BLOCK_SIZE as u64);
                    new_size = HEADER_SIZE as u64 + size + aes_padding;
                }

                let f_attr = FileAttr {
                    size: new_size,
                    blocks: metadata.st_blocks(),
                    atime: metadata.accessed().unwrap(),
                    mtime: metadata.modified().unwrap(),
                    ctime: metadata.created().unwrap(),
                    crtime: metadata.accessed().unwrap(),       // linux doesn't have creation time
                    kind: if metadata.is_dir() { FileType::Directory } else { FileType::RegularFile },
                    perm: (metadata.st_mode() & 0xffff) as u16,
                    nlink: metadata.st_nlink() as u32,
                    uid: metadata.st_uid(),
                    gid: metadata.st_gid(),
                    rdev: metadata.st_rdev() as u32,
                    flags: 0        // macOS only, not supported on linux
                };

                return Ok((TTL, f_attr));
            },
            Err(_) => {
                return Err(libc::ENOENT);
            }
        }
        
    }

    fn chmod(&self, _req: RequestInfo, _path: &Path, _fh: Option<u64>, _mode: u32) -> ResultEmpty {
        debug!("chmod() called");
        // read only filesystem
        return Err(libc::EROFS);
    }

    fn chown(&self, _req: RequestInfo, _path: &Path, _fh: Option<u64>, _uid: Option<u32>, _gid: Option<u32>) -> ResultEmpty {
        debug!("chown() called");
        // read only filesystem
        return Err(libc::EROFS);
    }

    fn truncate(&self, _req: RequestInfo, _path: &Path, _fh: Option<u64>, _size: u64) -> ResultEmpty {
        debug!("truncate() called");
        // read only filesystem
        return Err(libc::EROFS);
    }

    fn utimens(&self, _req: RequestInfo, _path: &Path, _fh: Option<u64>, _atime: Option<SystemTime>, _mtime: Option<SystemTime>) -> ResultEmpty {
        debug!("utimens() called");
        // read only filesystem
        return Err(libc::EROFS);
    }

    fn utimens_macos(&self, _req: RequestInfo, _path: &Path, _fh: Option<u64>, _crtime: Option<std::time::SystemTime>, _chgtime: Option<std::time::SystemTime>, _bkuptime: Option<std::time::SystemTime>, _flags: Option<u32>) -> ResultEmpty {
        debug!("utimens_macos() called");
        return Err(libc::EROFS)     //read only filesystem & macOS only
    }

    fn readlink(&self, _req: RequestInfo, _path: &Path) -> ResultData {
        debug!("readlink() called");
        // there should be no symlinks in this filesystem
        return Err(libc::EINVAL);
    }

    fn mknod(&self, _req: RequestInfo, _parent: &Path, _name: &std::ffi::OsStr, _mode: u32, _rdev: u32) -> ResultEntry {
        debug!("mknod() called");
        // read only filesystem
        return Err(libc::EROFS);
    }

    fn mkdir(&self, _req: RequestInfo, _parent: &Path, _name: &std::ffi::OsStr, _mode: u32) -> ResultEntry {
        debug!("mkdir() called");
        // read only filesystem
        return Err(libc::EROFS);
    }

    fn unlink(&self, _req: RequestInfo, _parent: &Path, _name: &std::ffi::OsStr) -> ResultEmpty {
        debug!("unlink() called");
        // read only filesystem
        return Err(libc::EROFS);
    }

    fn rmdir(&self, _req: RequestInfo, _parent: &Path, _name: &std::ffi::OsStr) -> ResultEmpty {
        debug!("rmdir() called");
        // read only filesystem
        return Err(libc::EROFS);
    }

    fn symlink(&self, _req: RequestInfo, _parent: &Path, _name: &std::ffi::OsStr, _target: &Path) -> ResultEntry {
        debug!("symlink() called");
        // read only filesystem
        return Err(libc::EROFS);
    }

    fn rename(&self, _req: RequestInfo, _parent: &Path, _name: &OsStr, _newparent: &Path, _newname: &OsStr) -> ResultEmpty {
        debug!("rename() called");
        // read only filesystem
        return Err(libc::EROFS);
    }

    fn link(&self, _req: RequestInfo, _path: &Path, _newparent: &Path, _newname: &std::ffi::OsStr) -> ResultEntry {
        debug!("link() called");
        // read only filesystem
        return Err(libc::EROFS);
    }

    fn open(&self, _req: RequestInfo, _path: &Path, _flags: u32) -> ResultOpen {
        debug!("open() called");

        let real_path = self.get_real_path(_path);
        self.check_path(&real_path)?;
        
        // if _flags == (libc::O_CREAT as u32 || libc::O_EXCL as u32) {
        //     // file creation not supported
        //     return Err(libc::EROFS);
        // }

        // get file handle
        let fd = fs::OpenOptions::new().read(true).open(real_path).unwrap().into_raw_fd() as u64;
        let flags = libc::O_RDONLY as u32;

        return Ok((fd, flags));
    }

    fn read(&self, _req: RequestInfo, _path: &Path, _fh: u64, _offset: u64, _size: u32, callback: impl FnOnce(ResultSlice<'_>) -> CallbackResult) -> CallbackResult {
        debug!("read() called");
        
        let file = unsafe { fs::File::from_raw_fd(_fh as i32) };
        let file_size = file.metadata().unwrap().len();

        let buf_size;
        if file_size > 0 {
            buf_size = file_size + HEADER_SIZE as u64;
        } else {
            return callback(Ok(&[]));
        }
        
        let mut buf = Vec::<u8>::with_capacity(buf_size as usize);
        unsafe { buf.set_len(buf_size as usize) };

        file.read_exact_at(&mut buf[HEADER_SIZE..], 0).unwrap();    // read the whole file

        // get iv from file hash, this makes it repeatable for the same file
        let iv = &hash(MessageDigest::md5(), &buf[HEADER_SIZE..]).unwrap()[..];

        // encrypt the file
        let mut enc_buf = encrypt(self.cipher, self.key.as_bytes(), Some(iv), &buf).unwrap();

        // cmpute mac
        let mut mac = HmacSha256::new_from_slice(self.key.as_bytes()).unwrap();
        mac.update(&enc_buf[HEADER_SIZE..]);
        let mac = mac.finalize().into_bytes();

        // replace dummy header with mac
        enc_buf[..HEADER_SIZE].copy_from_slice(&mac[..HEADER_SIZE]);


        // if we don't want to read the whole file, adjust the buffer
        let _size = if _size as usize > enc_buf.len() {enc_buf.len()} else { _size  as usize};
        let begin = _offset as usize;
        let end = (_offset as usize + _size) as usize;

        forget(file);  
        return callback(Ok(&enc_buf[begin..end]));

    }

    fn write(&self, _req: RequestInfo, _path: &Path, _fh: u64, _offset: u64, _data: Vec<u8>, _flags: u32) -> ResultWrite {
        debug!("write() called");
        // read only filesystem
        return Err(libc::EROFS);
    }

    fn flush(&self, _req: RequestInfo, _path: &Path, _fh: u64, _lock_owner: u64) -> ResultEmpty {
        debug!("flush() called");
        return Ok(());  // TODO: implement locking, maybe...
    }

    fn release(&self, _req: RequestInfo, _path: &Path, _fh: u64, _flags: u32, _lock_owner: u64, _flush: bool) -> ResultEmpty {
        debug!("release() called");
        
        // convert fd to file
        let file = unsafe { fs::File::from_raw_fd(_fh as i32) };    // rust will close the file when it goes out of scope

        return Ok(());
    }

    fn fsync(&self, _req: RequestInfo, _path: &Path, _fh: u64, _datasync: bool) -> ResultEmpty {
        debug!("fsync() called");
        // read only filesystem, so nothing to do
        return Ok(())
    }

    fn opendir(&self, _req: RequestInfo, _path: &Path, _flags: u32) -> ResultOpen {
        debug!("opendir() called");
        let real_path = self.get_real_path(_path);
        self.check_dir(&real_path)?;
        
        let handle = fs::File::open(real_path).unwrap().into_raw_fd() as u64;
        return Ok((handle, 0));
    }

    fn readdir(&self, _req: RequestInfo, _path: &Path, _fh: u64) -> ResultReaddir {
        // It would be better to use the libc::readdir() function, but for now I'll just use rust's fs::read_dir()
        debug!("readdir() called");

        let real_path = self.get_real_path(_path);
        self.check_dir(&real_path)?;

        let mut entries: Vec<DirectoryEntry> = Vec::new();

        for entry in fs::read_dir(real_path.as_path()).unwrap()  {
            let entry = entry.unwrap();
            let path = entry.path();

            // make sure is either regular file or directory
            match self.check_path(&path) {
                Ok(_) => {},
                Err(_) => { continue; }
            }

            entries.push(DirectoryEntry {
                name: entry.file_name(),
                kind: if path.is_dir() { FileType::Directory } else { FileType::RegularFile }
            });
        }
        
        return Ok(entries);

    }

    fn releasedir(&self, _req: RequestInfo, _path: &Path, _fh: u64, _flags: u32) -> ResultEmpty {
        debug!("releasedir() called");
        
        let f = unsafe{ fs::File::from_raw_fd(_fh as RawFd) };
        drop(f);
        return Ok(());
    }

    fn fsyncdir(&self, _req: RequestInfo, _path: &Path, _fh: u64, _datasync: bool) -> ResultEmpty {
        debug!("fsyncdir() called");
        return Ok(());  // nothing to do
    }

    fn statfs(&self, _req: RequestInfo, _path: &Path) -> ResultStatfs {
        debug!("statfs() called");
        // TODO: implement
        return Err(libc::ENOSYS);
    }

    fn setxattr(&self, _req: RequestInfo, _path: &Path, _name: &OsStr, _value: &[u8], _flags: u32, _position: u32) -> ResultEmpty {
        debug!("setxattr() called");
        // read only filesystem
        return Err(libc::EROFS);
    }

    fn listxattr(&self, _req: RequestInfo, _path: &Path, _size: u32) -> ResultXattr {
        debug!("listxattr() called");
        // not implemented
        return Err(libc::ENOSYS);
    }

    fn getxattr(&self, _req: RequestInfo, _path: &Path, _name: &std::ffi::OsStr, _size: u32) -> ResultXattr {
        debug!("getxattr() called");
        // not implemented
        return Err(libc::ENOSYS);
    }

    fn removexattr(&self, _req: RequestInfo, _path: &Path, _name: &OsStr) -> ResultEmpty {
        debug!("removexattr() called");
        // read only filesystem
        return Err(libc::EROFS);
    }

    fn access(&self, _req: RequestInfo, _path: &Path, _mask: u32) -> ResultEmpty {
        debug!("access() called");
        // TODO: see if this is needed or if cloning the file permission is enough
        return Err(libc::ENOSYS)
    }

    fn create(&self, _req: RequestInfo, _parent: &Path, _name: &OsStr, _mode: u32, _flags: u32) -> ResultCreate {
        debug!("create() called");
        // read only filesystem
        return Err(libc::EROFS);
    }
    

}
