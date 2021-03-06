//!
//! Helper to compose arbitrary data structures into packets of binary data.
//!

use std::{cast, mem, ptr, slice};
use std::io::{FileType, TypeFile, TypeDirectory, TypeNamedPipe, TypeBlockSpecial, TypeSymlink, TypeUnknown};
use std::libc::{S_IFREG, S_IFDIR, S_IFCHR, S_IFBLK, S_IFLNK};
use fuse::{fuse_file_lock, fuse_entry_out, fuse_attr_out, fuse_open_out};
use fuse::{fuse_write_out, fuse_statfs_out, fuse_getxattr_out, fuse_lk_out};
use fuse::{fuse_init_out, fuse_bmap_out, fuse_out_header, fuse_dirent};
#[cfg(target_os = "macos")]
use fuse::{fuse_getxtimes_out};

/// Trait for types that can be sent as a reply to the FUSE kernel driver
pub trait Sendable {
	/// Returns the byte representation of a type
	fn as_bytegroups<T> (&self, f: |&[&[u8]]| -> T) -> T {
		// Generally send a memory copy of a type (this works for all
		// structs, i.e. fuse_*_out)
		unsafe {
			let len = mem::size_of::<Self>();
			slice::raw::buf_as_slice(self as *Self as *u8, len, |bytes| {
				f([bytes])
			})
		}
	}
}

// Implemente sendable trait for fuse_*_out data types
impl Sendable for fuse_file_lock { }
impl Sendable for fuse_entry_out { }
impl Sendable for fuse_attr_out { }
impl Sendable for fuse_open_out { }
impl Sendable for fuse_write_out { }
impl Sendable for fuse_statfs_out { }
impl Sendable for fuse_getxattr_out { }
impl Sendable for fuse_lk_out { }
impl Sendable for fuse_init_out { }
impl Sendable for fuse_bmap_out { }
impl Sendable for fuse_out_header { }
#[cfg(target_os = "macos")]
impl Sendable for fuse_getxtimes_out { }

impl Sendable for () {
	fn as_bytegroups<T> (&self, f: |&[&[u8]]| -> T) -> T {
		// A unit value has nothing to send
		f([])
	}
}

impl<S1: Sendable, S2: Sendable> Sendable for (S1, S2) {
	fn as_bytegroups<T> (&self, f: |&[&[u8]]| -> T) -> T {
		match self {
			&(ref s1, ref s2) => {
				s1.as_bytegroups(|d1| {
					s2.as_bytegroups(|d2| {
						f(d1 + d2)
					})
				})
			},
		}
	}
}

impl<'a> Sendable for &'a [u8] {
	fn as_bytegroups<T> (&self, f: |&[&[u8]]| -> T) -> T {
		f([*self])
	}
}

impl Sendable for Vec<u8> {
	fn as_bytegroups<T> (&self, f: |&[&[u8]]| -> T) -> T {
		f([self.as_slice()])
	}
}

impl<'a> Sendable for &'a str {
	fn as_bytegroups<T> (&self, f: |&[&[u8]]| -> T) -> T {
		// Sending a string uses its byte-representation (without trailing NUL)
		f([self.as_bytes()])
	}
}

/// Buffer for replying with a list of directory entries
pub struct DirBuffer {
	priv data: Vec<u8>,
}

impl DirBuffer {
	/// Create a new dir buffer of the given size
	pub fn new (size: uint) -> DirBuffer {
		DirBuffer { data: Vec::with_capacity(size) }
	}

	/// Add an entry to the dir buffer. Returns true if the buffer is full.
	/// A transparent offset value can be provided for each entry. The
	/// kernel uses these value to request the next entries in further
	/// readdir calls
	pub fn fill (&mut self, ino: u64, offset: u64, typ: FileType, name: &PosixPath) -> bool {
		let name = name.as_vec();
		let entlen = mem::size_of::<fuse_dirent>() + name.len();
		let entsize = (entlen + mem::size_of::<u64>() - 1) & !(mem::size_of::<u64>() - 1);	// 64bit align
		let padlen = entsize - entlen;
		if self.data.len() + entsize > self.data.capacity() { return true; }
		unsafe {
			let p = self.data.as_mut_ptr().offset(self.data.len() as int);
			let pdirent: *mut fuse_dirent = cast::transmute(p);
			(*pdirent).ino = ino;
			(*pdirent).off = offset;
			(*pdirent).namelen = name.len() as u32;
			(*pdirent).typ = match typ {
				TypeFile => S_IFREG, TypeDirectory => S_IFDIR, TypeNamedPipe => S_IFCHR,
				TypeBlockSpecial => S_IFBLK, TypeSymlink => S_IFLNK, TypeUnknown => 0,
			} as u32 >> 12;
			let p = p.offset(mem::size_of_val(&*pdirent) as int);
			ptr::copy_memory(p, name.as_ptr(), name.len());
			let p = p.offset(name.len() as int);
			ptr::zero_memory(p, padlen);
			let newlen = self.data.len() + entsize;
			self.data.set_len(newlen);
		}
		false
	}

	/// Returns the size of the data that has been filled into the buffer
	pub fn len (&self) -> uint {
		self.data.len()
	}
}

impl Sendable for DirBuffer {
	fn as_bytegroups<T> (&self, f: |&[&[u8]]| -> T) -> T {
		// Send a dirbuffer by sending its data vector
		self.data.as_bytegroups(f)
	}
}


#[cfg(test)]
mod test {
	use std::io::{TypeFile, TypeDirectory};
	use super::{Sendable, DirBuffer};

	struct TestData { p1: u8, p2: u8, p3: u16 }
	impl Sendable for TestData {}

	#[test]
	fn sendable_struct () {
		let data = TestData { p1: 111, p2: 222, p3: 333 };
		data.as_bytegroups(|bytes| {
			assert!(bytes.len() == 1, "sendable struct should be represented as a single bytes slice");
			assert!(bytes[0] == [0x6f, 0xde, 0x4d, 0x01], "sendable struct should be represented by a bytes slice with the byte representation of the struct");
		});
	}

	#[test]
	fn sendable_null () {
		let data = ();
		data.as_bytegroups(|bytes| {
			assert!(bytes.len() == 0, "sendable empty element should be represented by no bytes slice at all");
		});
	}

	#[test]
	fn sendable_tuple () {
		let data = (TestData { p1: 111, p2: 222, p3: 333 }, TestData { p1: 112, p2: 223, p3: 334 });
		data.as_bytegroups(|bytes| {
			assert!(bytes.len() == 2, "sendable tuple should be represented as multiple bytes slices");
			assert!(bytes[0] == [0x6f, 0xde, 0x4d, 0x01], "sendable tuple should first be represented by a bytes slice with the byte representation of the first element");
			assert!(bytes[1] == [0x70, 0xdf, 0x4e, 0x01], "sendable tuple should second be represented by a bytes slice with the byte representation of the second element");
		});
	}

	#[test]
	fn sendable_byteslice () {
		let data = [11, 22, 33, 44, 55];
		data.as_bytegroups(|bytes| {
			assert!(bytes.len() == 1, "sendabled buffer should be represented as a single bytes slice");
			assert!(bytes[0] == data, "sendable buffer should be represented by a bytes slice with the contents of the buffer");
		});
	}

	#[test]
	fn sendable_bytevector () {
		let data = vec!(11, 22, 33, 44, 55);
		data.as_bytegroups(|bytes| {
			assert!(bytes.len() == 1, "sendabled buffer should be represented as a single bytes slice");
			assert!(bytes[0] == data.as_slice(), "sendable buffer should be represented by a bytes slice with the contents of the buffer");
		});
	}

	#[test]
	fn sendable_string () {
		let data = "hello";
		let expected = [104, 101, 108, 108, 111];	// no trailing NUL
		data.as_bytegroups(|bytes| {
			assert!(bytes.len() == 1, "sendable string should be represented as a single bytes slice");
			assert!(bytes[0] == expected, "sendable string should be represented by a bytes slice with the contents of the string");
		});
	}

	#[test]
	fn sendable_dirbuffer () {
		let mut buf = DirBuffer::new(128);
		buf.fill(111, 222, TypeDirectory, &PosixPath::new("hello"));
		buf.fill(444, 555, TypeFile, &PosixPath::new("world.rs"));
		let expected = [
			111, 0, 0, 0, 0, 0, 0, 0, 222, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 4, 0, 0, 0, 104, 101, 108, 108, 111,  0,   0,   0,
			188, 1, 0, 0, 0, 0, 0, 0,  43, 2, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 119, 111, 114, 108, 100, 46, 114, 115,
		];
		buf.as_bytegroups(|bytes| {
			assert!(bytes.len() == 1, "sendable dirbuffer should be represented as a single bytes slice");
			assert!(bytes[0] == expected, "sendable dirbuffer should be represented by a bytes slice with the contents of the dirbuffer");
		});
	}
}
