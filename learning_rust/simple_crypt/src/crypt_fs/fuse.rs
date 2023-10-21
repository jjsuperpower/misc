use std::ffi::OsStr;
use std::io::{self, prelude::*};
use std::mem::forget;
use std::os::fd::{IntoRawFd, FromRawFd, RawFd};
use std::{fs, os::linux::fs::MetadataExt};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use fuse_mt::*;
use log::{debug, error};
use libc;

use super::CryptFS;

const TTL: Duration = Duration::from_secs(1);



fn check_path(path: &Path) -> Result<(), libc::c_int> {
    if (path.is_dir()|| path.is_file()) && !path.is_symlink() {
        // check if file a regular file or directory
        return Ok(());
    } else {
        return Err(libc::ENOENT);
    }
}

fn check_dir(path: &Path) -> Result<(), libc::c_int> {
    if path.is_dir() && !path.is_symlink() {
        // check if file a regular file or directory
        return Ok(());
    } else {
        return Err(libc::ENOTDIR);
    }
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
        check_path(&real_path)?;       // don't follow symlinks, or special files

        let file = match fs::File::open(real_path) {
            Ok(file) => file,
            Err(_) => return Err(libc::ENOENT),
        };
            
        let metadata = match file.metadata() {
            Ok(metadata) => metadata,
            Err(_) => return Err(libc::ENOENT),
        };

        let f_attr = FileAttr {
            size: if metadata.is_file() { self.get_read_size(&file)} else { metadata.len() },
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

        return Ok((TTL,f_attr));

        
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
        // TODO: check requested flags and get file handle accordingly

        let real_path = self.get_real_path(_path);
        check_path(&real_path)?;
        
        // if _flags == (libc::O_CREAT as u32 || libc::O_EXCL as u32) {
        //     // file creation not supported
        //     return Err(libc::EROFS);
        // }

        // get file handle
        let fd = fs::OpenOptions::new().read(true).open(real_path).unwrap().into_raw_fd() as u64;
        let flags = libc::O_RDONLY as u32;
        forget(fd);     // otherwise rust will close the file when it goes out of scope

        return Ok((fd, flags));
    }

    fn read(&self, _req: RequestInfo, _path: &Path, _fh: u64, _offset: u64, _size: u32, callback: impl FnOnce(ResultSlice<'_>) -> CallbackResult) -> CallbackResult {
        debug!("read() called");
        let file = unsafe { fs::File::from_raw_fd(_fh as i32) };
        
        return callback(Ok(&[]));

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
        drop(file);
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
        check_dir(&real_path)?;
        
        let handle = fs::File::open(real_path).unwrap().into_raw_fd() as u64;
        return Ok((handle, 0));
    }

    fn readdir(&self, _req: RequestInfo, _path: &Path, _fh: u64) -> ResultReaddir {
        // It would be better to use the libc::readdir() function, but for now I'll just use rust's fs::read_dir()
        debug!("readdir() called");

        let real_path = self.get_real_path(_path);
        check_dir(&real_path)?;

        let mut entries: Vec<DirectoryEntry> = Vec::new();

        for entry in fs::read_dir(real_path.as_path()).unwrap()  {
            let entry = entry.unwrap();
            let path = entry.path();

            // make sure is either regular file or directory
            match check_path(&path) {
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
