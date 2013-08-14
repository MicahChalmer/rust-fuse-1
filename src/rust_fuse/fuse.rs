/* 
automatically generated by rust-bindgen, with some manual changes needed:
  * Replace Struct_stat with libc::stat
  * use a bindgen'd statvfs
*/

use std::libc::*;
mod statvfs;
mod fcntl;
pub struct Struct_iovec {
    iov_base: *c_void,
    iov_len: size_t
}
pub struct Struct_fuse_opt {
    pub templ: *c_schar,
    pub offset: c_ulong,
    pub value: c_int,
}
pub struct Struct_fuse_args {
    pub argc: c_int,
    pub argv: *mut *mut c_schar,
    pub allocated: c_int,
}
pub type fuse_opt_proc_t = *u8;
pub struct Struct_fuse_file_info {
    pub flags: c_int,
    pub fh_old: c_ulong,
    pub writepage: c_int,
    pub direct_io: c_uint,
    pub keep_cache: c_uint,
    pub flush: c_uint,
    pub nonseekable: c_uint,
    pub flock_release: c_uint,
    pub padding: c_uint,
    pub fh: uint64_t,
    pub lock_owner: uint64_t,
}
pub struct Struct_fuse_conn_info {
    pub proto_major: c_uint,
    pub proto_minor: c_uint,
    pub async_read: c_uint,
    pub max_write: c_uint,
    pub max_readahead: c_uint,
    pub capable: c_uint,
    pub want: c_uint,
    pub max_background: c_uint,
    pub congestion_threshold: c_uint,
    pub reserved: [c_uint, ..23u],
}
pub type Struct_fuse_session = c_void;
pub type Struct_fuse_chan = c_void;
pub type Struct_fuse_pollhandle = c_void;
pub type Enum_fuse_buf_flags = c_uint;
pub static FUSE_BUF_IS_FD: c_uint = 2;
pub static FUSE_BUF_FD_SEEK: c_uint = 4;
pub static FUSE_BUF_FD_RETRY: c_uint = 8;
pub type Enum_fuse_buf_copy_flags = c_uint;
pub static FUSE_BUF_NO_SPLICE: c_uint = 2;
pub static FUSE_BUF_FORCE_SPLICE: c_uint = 4;
pub static FUSE_BUF_SPLICE_MOVE: c_uint = 8;
pub static FUSE_BUF_SPLICE_NONBLOCK: c_uint = 16;
pub struct Struct_fuse_buf {
    pub size: size_t,
    pub flags: Enum_fuse_buf_flags,
    pub mem: *mut c_void,
    pub fd: c_int,
    pub pos: off_t,
}
pub struct Struct_fuse_bufvec {
    pub count: size_t,
    pub idx: size_t,
    pub off: size_t,
    pub buf: [Struct_fuse_buf, ..1u],
}
pub type fuse_ino_t = c_ulong;
pub type Struct_fuse_req = c_void;
pub type fuse_req_t = *mut Struct_fuse_req;
pub struct Struct_fuse_entry_param {
    pub ino: fuse_ino_t,
    pub generation: c_ulong,
    pub attr: stat,
    pub attr_timeout: c_double,
    pub entry_timeout: c_double,
}
pub struct Struct_fuse_ctx {
    pub uid: uid_t,
    pub gid: gid_t,
    pub pid: pid_t,
    pub umask: mode_t,
}
pub struct Struct_fuse_forget_data {
    pub ino: uint64_t,
    pub nlookup: uint64_t,
}
pub struct Struct_fuse_lowlevel_ops {
    pub init: *u8,
    pub destroy: *u8,
    pub lookup: *u8,
    pub forget: *u8,
    pub getattr: *u8,
    pub setattr: *u8,
    pub readlink: *u8,
    pub mknod: *u8,
    pub mkdir: *u8,
    pub unlink: *u8,
    pub rmdir: *u8,
    pub symlink: *u8,
    pub rename: *u8,
    pub link: *u8,
    pub open: *u8,
    pub read: *u8,
    pub write: *u8,
    pub flush: *u8,
    pub release: *u8,
    pub fsync: *u8,
    pub opendir: *u8,
    pub readdir: *u8,
    pub releasedir: *u8,
    pub fsyncdir: *u8,
    pub statfs: *u8,
    pub setxattr: *u8,
    pub getxattr: *u8,
    pub listxattr: *u8,
    pub removexattr: *u8,
    pub access: *u8,
    pub create: *u8,
    pub getlk: *u8,
    pub setlk: *u8,
    pub bmap: *u8,
    pub ioctl: *u8,
    pub poll: *u8,
    pub write_buf: *u8,
    pub retrieve_reply: *u8,
    pub forget_multi: *u8,
    pub flock: *u8,
    pub fallocate: *u8,
}
pub type fuse_interrupt_func_t = *u8;
pub struct Struct_fuse_session_ops {
    pub process: *u8,
    pub exit: *u8,
    pub exited: *u8,
    pub destroy: *u8,
}
pub struct Struct_fuse_chan_ops {
    pub receive: *u8,
    pub send: *u8,
    pub destroy: *u8,
}
#[link_args = "-lfuse"]
extern "C" {
    pub fn fuse_opt_parse(args: *mut Struct_fuse_args, data: *mut c_void,
                          opts: *Struct_fuse_opt, proc: fuse_opt_proc_t) ->
     c_int;
    pub fn fuse_opt_add_opt(opts: *mut *mut c_schar, opt: *c_schar) -> c_int;
    pub fn fuse_opt_add_opt_escaped(opts: *mut *mut c_schar, opt: *c_schar) ->
     c_int;
    pub fn fuse_opt_add_arg(args: *mut Struct_fuse_args, arg: *c_schar) ->
     c_int;
    pub fn fuse_opt_insert_arg(args: *mut Struct_fuse_args, pos: c_int,
                               arg: *c_schar) -> c_int;
    pub fn fuse_opt_free_args(args: *mut Struct_fuse_args);
    pub fn fuse_opt_match(opts: *Struct_fuse_opt, opt: *c_schar) -> c_int;
    pub fn fuse_mount(mountpoint: *c_schar, args: *mut Struct_fuse_args) ->
     *mut Struct_fuse_chan;
    pub fn fuse_unmount(mountpoint: *c_schar, ch: *mut Struct_fuse_chan);
    pub fn fuse_parse_cmdline(args: *mut Struct_fuse_args,
                              mountpoint: *mut *mut c_schar,
                              multithreaded: *mut c_int,
                              foreground: *mut c_int) -> c_int;
    pub fn fuse_daemonize(foreground: c_int) -> c_int;
    pub fn fuse_version() -> c_int;
    pub fn fuse_pollhandle_destroy(ph: *mut Struct_fuse_pollhandle);
    pub fn fuse_buf_size(bufv: *Struct_fuse_bufvec) -> size_t;
    pub fn fuse_buf_copy(dst: *mut Struct_fuse_bufvec,
                         src: *mut Struct_fuse_bufvec,
                         flags: Enum_fuse_buf_copy_flags) -> ssize_t;
    pub fn fuse_set_signal_handlers(se: *mut Struct_fuse_session) -> c_int;
    pub fn fuse_remove_signal_handlers(se: *mut Struct_fuse_session);
    pub fn fuse_reply_err(req: fuse_req_t, err: c_int) -> c_int;
    pub fn fuse_reply_none(req: fuse_req_t);
    pub fn fuse_reply_entry(req: fuse_req_t, e: *Struct_fuse_entry_param) ->
     c_int;
    pub fn fuse_reply_create(req: fuse_req_t, e: *Struct_fuse_entry_param,
                             fi: *Struct_fuse_file_info) -> c_int;
    pub fn fuse_reply_attr(req: fuse_req_t, attr: *stat,
                           attr_timeout: c_double) -> c_int;
    pub fn fuse_reply_readlink(req: fuse_req_t, link: *c_schar) -> c_int;
    pub fn fuse_reply_open(req: fuse_req_t, fi: *Struct_fuse_file_info) ->
     c_int;
    pub fn fuse_reply_write(req: fuse_req_t, count: size_t) -> c_int;
    pub fn fuse_reply_buf(req: fuse_req_t, buf: *c_schar, size: size_t) ->
     c_int;
    pub fn fuse_reply_data(req: fuse_req_t, bufv: *mut Struct_fuse_bufvec,
                           flags: Enum_fuse_buf_copy_flags) -> c_int;
    pub fn fuse_reply_iov(req: fuse_req_t, iov: *Struct_iovec, count: c_int)
     -> c_int;
    pub fn fuse_reply_statfs(req: fuse_req_t, stbuf: *statvfs::Struct_statvfs) ->
     c_int;
    pub fn fuse_reply_xattr(req: fuse_req_t, count: size_t) -> c_int;
    pub fn fuse_reply_lock(req: fuse_req_t, lock: *fcntl::Struct_flock) -> c_int;
    pub fn fuse_reply_bmap(req: fuse_req_t, idx: uint64_t) -> c_int;
    pub fn fuse_add_direntry(req: fuse_req_t, buf: *mut c_schar,
                             bufsize: size_t, name: *c_schar,
                             stbuf: *stat, off: off_t) -> size_t;
    pub fn fuse_reply_ioctl_retry(req: fuse_req_t, in_iov: *Struct_iovec,
                                  in_count: size_t, out_iov: *Struct_iovec,
                                  out_count: size_t) -> c_int;
    pub fn fuse_reply_ioctl(req: fuse_req_t, result: c_int, buf: *c_void,
                            size: size_t) -> c_int;
    pub fn fuse_reply_ioctl_iov(req: fuse_req_t, result: c_int,
                                iov: *Struct_iovec, count: c_int) -> c_int;
    pub fn fuse_reply_poll(req: fuse_req_t, revents: c_uint) -> c_int;
    pub fn fuse_lowlevel_notify_poll(ph: *mut Struct_fuse_pollhandle) ->
     c_int;
    pub fn fuse_lowlevel_notify_inval_inode(ch: *mut Struct_fuse_chan,
                                            ino: fuse_ino_t, off: off_t,
                                            len: off_t) -> c_int;
    pub fn fuse_lowlevel_notify_inval_entry(ch: *mut Struct_fuse_chan,
                                            parent: fuse_ino_t,
                                            name: *c_schar, namelen: size_t)
     -> c_int;
    pub fn fuse_lowlevel_notify_delete(ch: *mut Struct_fuse_chan,
                                       parent: fuse_ino_t, child: fuse_ino_t,
                                       name: *c_schar, namelen: size_t) ->
     c_int;
    pub fn fuse_lowlevel_notify_store(ch: *mut Struct_fuse_chan,
                                      ino: fuse_ino_t, offset: off_t,
                                      bufv: *mut Struct_fuse_bufvec,
                                      flags: Enum_fuse_buf_copy_flags) ->
     c_int;
    pub fn fuse_lowlevel_notify_retrieve(ch: *mut Struct_fuse_chan,
                                         ino: fuse_ino_t, size: size_t,
                                         offset: off_t, cookie: *mut c_void)
     -> c_int;
    pub fn fuse_req_userdata(req: fuse_req_t) -> *mut c_void;
    pub fn fuse_req_ctx(req: fuse_req_t) -> *Struct_fuse_ctx;
    pub fn fuse_req_getgroups(req: fuse_req_t, size: c_int, list: *mut gid_t)
     -> c_int;
    pub fn fuse_req_interrupt_func(req: fuse_req_t,
                                   func: fuse_interrupt_func_t,
                                   data: *mut c_void);
    pub fn fuse_req_interrupted(req: fuse_req_t) -> c_int;
    pub fn fuse_lowlevel_is_lib_option(opt: *c_schar) -> c_int;
    pub fn fuse_lowlevel_new(args: *mut Struct_fuse_args,
                             op: *Struct_fuse_lowlevel_ops, op_size: size_t,
                             userdata: *mut c_void) ->
     *mut Struct_fuse_session;
    pub fn fuse_session_new(op: *mut Struct_fuse_session_ops,
                            data: *mut c_void) -> *mut Struct_fuse_session;
    pub fn fuse_session_add_chan(se: *mut Struct_fuse_session,
                                 ch: *mut Struct_fuse_chan);
    pub fn fuse_session_remove_chan(ch: *mut Struct_fuse_chan);
    pub fn fuse_session_next_chan(se: *mut Struct_fuse_session,
                                  ch: *mut Struct_fuse_chan) ->
     *mut Struct_fuse_chan;
    pub fn fuse_session_process(se: *mut Struct_fuse_session, buf: *c_schar,
                                len: size_t, ch: *mut Struct_fuse_chan);
    pub fn fuse_session_process_buf(se: *mut Struct_fuse_session,
                                    buf: *Struct_fuse_buf,
                                    ch: *mut Struct_fuse_chan);
    pub fn fuse_session_receive_buf(se: *mut Struct_fuse_session,
                                    buf: *mut Struct_fuse_buf,
                                    chp: *mut *mut Struct_fuse_chan) -> c_int;
    pub fn fuse_session_destroy(se: *mut Struct_fuse_session);
    pub fn fuse_session_exit(se: *mut Struct_fuse_session);
    pub fn fuse_session_reset(se: *mut Struct_fuse_session);
    pub fn fuse_session_exited(se: *mut Struct_fuse_session) -> c_int;
    pub fn fuse_session_data(se: *mut Struct_fuse_session) -> *mut c_void;
    pub fn fuse_session_loop(se: *mut Struct_fuse_session) -> c_int;
    pub fn fuse_session_loop_mt(se: *mut Struct_fuse_session) -> c_int;
    pub fn fuse_chan_new(op: *mut Struct_fuse_chan_ops, fd: c_int,
                         bufsize: size_t, data: *mut c_void) ->
     *mut Struct_fuse_chan;
    pub fn fuse_chan_fd(ch: *mut Struct_fuse_chan) -> c_int;
    pub fn fuse_chan_bufsize(ch: *mut Struct_fuse_chan) -> size_t;
    pub fn fuse_chan_data(ch: *mut Struct_fuse_chan) -> *mut c_void;
    pub fn fuse_chan_session(ch: *mut Struct_fuse_chan) ->
     *mut Struct_fuse_session;
    pub fn fuse_chan_recv(ch: *mut *mut Struct_fuse_chan, buf: *mut c_schar,
                          size: size_t) -> c_int;
    pub fn fuse_chan_send(ch: *mut Struct_fuse_chan, iov: *Struct_iovec,
                          count: size_t) -> c_int;
    pub fn fuse_chan_destroy(ch: *mut Struct_fuse_chan);
}
