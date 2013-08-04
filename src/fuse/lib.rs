#[link(name = "fuse",
uuid = "d37c5c30-fdcd-459d-bfca-ebb8da04b2a0",
url = "https://github.com/MicahChalmer/rust-fuse")];

#[comment = "FUSE bindings"];
#[license = "MIT"];
#[crate_type = "lib"];

use std::libc::{
    c_char,
    c_int,
    c_uint,
    c_ulong,
    c_void,
    mode_t,
    off_t,
    pid_t,
    size_t,
    stat,
    uid_t,
    gid_t
};

use std::ptr;
use std::str;

// A cfuncptr is used here to stand in for a C function pointer.
// For this struct, see fuse.h
type cfuncptr = *u8;
struct c_fuse_operations {
    getattr: cfuncptr,
    readlink: cfuncptr,
    getdir: cfuncptr,
    mknod: cfuncptr,
    mkdir: cfuncptr,
    unlink: cfuncptr,
    rmdir: cfuncptr,
    symlink: cfuncptr,
    rename: cfuncptr,
    link: cfuncptr,
    chmod: cfuncptr,
    chown: cfuncptr,
    truncate: cfuncptr,
    utime: cfuncptr,
    open: cfuncptr,
    read: cfuncptr,
    write: cfuncptr,
    statfs: cfuncptr,
    flush: cfuncptr,
    release: cfuncptr,
    fsync: cfuncptr,
    setxattr: cfuncptr,
    getxattr: cfuncptr,
    listxattr: cfuncptr,
    removexattr: cfuncptr,
    opendir: cfuncptr,
    readdir: cfuncptr,
    releasedir: cfuncptr,
    fsyncdir: cfuncptr,
    init: cfuncptr,
    destroy: cfuncptr,
    access: cfuncptr,
    create: cfuncptr,
    ftruncate: cfuncptr,
    fgetattr: cfuncptr,
    lock: cfuncptr,
    utimens: cfuncptr,
    bmap: cfuncptr,

    flag_nullpath_ok: uint,
    flag_nopath: uint,
    flag_utime_omit_ok: uint,
    flag_reserved: uint,

    ioctl: cfuncptr,
    poll: cfuncptr,
    write_buf: cfuncptr,
    read_buf: cfuncptr,
    flock: cfuncptr
}

// TODO: this should not be public
pub struct fuse_file_info {
    flags: c_int,
    fh_old: c_ulong, // Old file handle, don't use
    writepage: c_int,
    direct_io: c_uint,
    keep_cache: c_uint,
    flush: c_uint,
    nonseekable: c_uint,
    flock_release: c_uint,
    padding: c_uint, // Padding.  Do not use
    fh: u64,
    lock_owner: u64
}

struct c_fuse_context {
    fuse: *c_void,
    uid: uid_t,
    gid: gid_t,
    pid: pid_t,
    private_data: *rust_fuse_data,  // we use this to know what object to call
    umask: mode_t 
}

struct rust_fuse_data {
    ops: ~FuseOperations
}

extern {
    fn fuse_main_real(argc:c_int, argv:**c_char, 
                      op:*c_fuse_operations, op_size: size_t,
                      user_data: *c_void);

    fn fuse_get_context() -> *c_fuse_context;

    // Workaround for the fact that we can't call into c via a function ptr right
    // from rust
    fn call_filer_function(filler: cfuncptr, buf: *c_void, name: *c_char, stbuf: *stat,
                           off: off_t);
}

// Used for return values from FS operations
type errno = c_int;

type fuse_fill_dir_func<'self> = &'self fn (&'self str, Option<stat>, off_t) -> c_int;

pub struct dir_entry {
    name: ~str,
    stat: Option<stat>,
    off: off_t
}

pub trait FuseOperations {
    fn getattr(&self, path:&str, stbuf: &mut stat) -> errno;
    fn readdir(&self, path:&str, info: &fuse_file_info, filler: fuse_fill_dir_func) -> errno;
    fn open(&self, path:&str, info: &mut fuse_file_info) -> errno;  // TODO: don't allow mutation of the whole fuse_file_info
    fn read(&self, path:&str, buf:&mut [u8], size: size_t, offset: off_t, info: &fuse_file_info) -> (errno, size_t);
}

extern fn c_getattr(path: *c_char, stbuf: *mut stat) -> errno {
    unsafe {
        let ops = &((*(*fuse_get_context()).private_data).ops);
        ptr::zero_memory(stbuf, 1);
        ops.getattr(str::raw::from_c_str(path), &mut *stbuf)
    }
}

extern fn c_readdir(path: *c_char, buf: *c_void, filler: cfuncptr,
                    offset: off_t, fi: *fuse_file_info) {
    unsafe {
        
    }
}

pub fn fuse_main<T: FuseOperations>(args: ~[~str], ops: ~T) {
    
}
