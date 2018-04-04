// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use prelude::*;
use os::raw::c_char;
use io;
use io::Write;
use traits;
use core::hash::Hash;
use fmt::{Debug, Display};
use borrow::Cow;
use rc::Rc;
use alloc::arc::Arc;
use path;
use ffi;
use time::Duration;

pub trait Std: Sized + Debug + Send + Sync + PartialEq + Eq + PartialOrd + Ord + Copy + Clone + Hash + 'static {
    type c_char: Copy + Hash + 'static;
    type c_double: Copy + 'static;
    type c_float: Copy + 'static;
    type c_int: Copy + Hash + 'static;
    type c_long: Copy + Hash + 'static;
    type c_longlong: Copy + Hash + 'static;
    type c_schar: Copy + Hash + 'static;
    type c_short: Copy + Hash + 'static;
    type c_uchar: Copy + Hash + 'static;
    type c_uint: Copy + Hash + 'static;
    type c_ulong: Copy + Hash + 'static;
    type c_ulonglong: Copy + Hash + 'static;
    type c_ushort: Copy + Hash + 'static;

    type Mutex: traits::Mutex;

    type OsString: traits::OsString<Self> + Clone;
    type OsStr: traits::OsStr<Self> + ?Sized;

    type SystemTime: SystemTime;
    type Instant: Instant;

    type File: File<Self>;
    type FileAttr: FileAttr<Self>;
    type ReadDir: Iterator<Item = io::Result<Self::DirEntry, Self>>;
    type OpenOptions: OpenOptions;
    type Permissions: Permissions;
    type FileType: FileType;
    type DirBuilder: DirBuilder<Self>;
    type DirEntry: DirEntry<Self>;

    // env
    type VarsOs: Iterator<Item = (ffi::OsString<Self>, ffi::OsString<Self>)>;
    type ArgsOs: Iterator<Item = ffi::OsString<Self>>;
    type SplitPaths<'a>;
    type JoinPathsError;
    const ARCH: &'static str;
    const FAMILY: &'static str;
    const OS: &'static str;
    const DLL_PREFIX: &'static str;
    const DLL_SUFFIX: &'static str;
    const DLL_EXTENSION: &'static str;
    const EXE_SUFFIX: &'static str;
    const EXE_EXTENSION: &'static str;
    fn vars_os() -> Self::VarsOs;
    fn args_os() -> Self::ArgsOs;
    fn current_dir() -> io::Result<path::PathBuf<Self>, Self>;
    fn set_current_dir(path: &path::Path<Self>) -> io::Result<path::PathBuf<Self>, Self>;

    type Stderr: Stdio<Self> + Write<Self>;

    const UNIX_EPOCH: Self::SystemTime;

    /// Usually defined as `&[0]`
    fn empty_cstr() -> &'static [c_char<Self>];

    fn last_os_error() -> i32;
    fn error_string(code: i32) -> String;

    fn init();
    unsafe fn abort_internal() -> !;
    unsafe fn strlen(cs: *const c_char<Self>) -> usize;
    fn decode_error_kind(errno: i32) -> io::ErrorKind;
    unsafe fn thread_guard_init() -> Option<usize>;

    fn is_path_sep_byte(b: u8) -> bool;
    fn is_verbatim_path_sep(b: u8) -> bool {
        Self::is_path_sep_byte(b)
    }
    fn parse_path_prefix(path: &ffi::OsStr<Self>) -> Option<path::Prefix<Self>>;
    const MAIN_PATH_SEP_STR: &'static str;
    const MAIN_PATH_SEP: char;

    fn readdir(p: &path::Path<Self>) -> io::Result<Self::ReadDir, Self>;
    fn unlink(p: &path::Path<Self>) -> io::Result<(), Self>;
    fn stat(p: &path::Path<Self>) -> io::Result<Self::FileAttr, Self>;
    fn lstat(p: &path::Path<Self>) -> io::Result<Self::FileAttr, Self>;
    fn rename(old: &path::Path<Self>, new: &path::Path<Self>) -> io::Result<(), Self>;
    fn copy(from: &path::Path<Self>, to: &path::Path<Self>) -> io::Result<u64, Self>;
    fn link(src: &path::Path<Self>, dst: &path::Path<Self>) -> io::Result<(), Self>;
    fn symlink(src: &path::Path<Self>, dst: &path::Path<Self>) -> io::Result<(), Self>;
    fn readlink(p: &path::Path<Self>) -> io::Result<path::PathBuf<Self>, Self>;
    fn canonicalize(p: &path::Path<Self>) -> io::Result<path::PathBuf<Self>, Self>;
    fn set_perm(p: &path::Path<Self>, perm: Self::Permissions) -> io::Result<(), Self>;
    fn rmdir(p: &path::Path<Self>) -> io::Result<(), Self>;
    fn remove_dir_all(p: &path::Path<Self>) -> io::Result<(), Self>;

    fn memchr(needle: u8, haystack: &[u8]) -> Option<usize>;
    fn memrchr(needle: u8, haystack: &[u8]) -> Option<usize>;

    // rand
    // fn hashmap_random_keys() -> (u64, u64);

    /// One-time global initialization of command line arguments.
    unsafe fn args_init(argc: isize, argv: *const *const u8);
}

pub trait Mutex: Sync {
    /// Creates a new mutex for use.
    ///
    /// Behavior is undefined if the mutex is moved after it is
    /// first used with any of the functions below.
    const NEW: Self;

    /// Prepare the mutex for use.
    ///
    /// This should be called once the mutex is at a stable memory address.
    #[inline]
    unsafe fn init(&mut self);

    /// Locks the mutex blocking the current thread until it is available.
    ///
    /// Behavior is undefined if the mutex has been moved between this and any
    /// previous function call.
    #[inline]
    unsafe fn lock(&self);

    /// Attempts to lock the mutex without blocking, returning whether it was
    /// successfully acquired or not.
    ///
    /// Behavior is undefined if the mutex has been moved between this and any
    /// previous function call.
    #[inline]
    unsafe fn try_lock(&self) -> bool;

    /// Unlocks the mutex.
    ///
    /// Behavior is undefined if the current thread does not actually hold the
    /// mutex.
    #[inline]
    unsafe fn unlock(&self);

    /// Deallocates all resources associated with this mutex.
    ///
    /// Behavior is undefined if there are current or will be future users of
    /// this mutex.
    #[inline]
    unsafe fn destroy(&self);
}

pub trait OsString<STD: Std>: Sized {
    fn from_string(s: String) -> Self;
    fn into_string(self) -> Result<String, Self>;
    fn push_slice(&mut self, s: &STD::OsStr);
    fn with_capacity(capacity: usize) -> Self;
    fn clear(&mut self);
    fn capacity(&self) -> usize;
    fn reserve(&mut self, additional: usize);
    fn reserve_exact(&mut self, additional: usize);
    fn shrink_to_fit(&mut self);
    fn into_box(self) -> Box<STD::OsStr>;
    fn as_slice(&self) -> &STD::OsStr;
    fn from_box(boxed: Box<STD::OsStr>) -> Self;
    fn into_arc(&self) -> Arc<STD::OsStr>;
    fn into_rc(&self) -> Rc<STD::OsStr>;
}

pub trait OsStr<STD: Std>: Debug + Display {
    fn to_str(&self) -> Option<&str>;
    fn to_string_lossy(&self) -> Cow<str>;
    fn to_owned(&self) -> STD::OsString;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn as_bytes(&self) -> &[u8];
    fn into_box(&self) -> Box<Self>;
    fn into_arc(&self) -> Arc<Self>;
    fn into_rc(&self) -> Rc<Self>;
    fn empty_box() -> Box<Self>;
    fn from_str(s: &str) -> &Self;
    fn from_bytes(b: &[u8]) -> &Self;
}

pub trait Instant: Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Hash + Debug {
    fn now() -> Self;
    fn sub_instant(&self, earlier: &Self) -> Duration;
    fn add_duration(&self, other: &Duration) -> Self;
    fn sub_duration(&self, other: &Duration) -> Self;
}

pub trait SystemTime: Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Hash + Debug {
    fn now() -> Self;
    fn sub_time(&self, earlier: &Self) -> Result<Duration, Duration>;
    fn add_duration(&self, other: &Duration) -> Self;
    fn sub_duration(&self, other: &Duration) -> Self;
}

pub trait File<STD: Std>: Sized + Debug {
    fn open(path: &path::Path<STD>, opts: &STD::OpenOptions) -> io::Result<STD::File, STD>;
    fn read(&self, buf: &mut [u8]) -> io::Result<usize, STD>;
    fn write(&self, buf: &[u8]) -> io::Result<usize, STD>;
    fn flush(&self) -> io::Result<(), STD>;
    fn seek(&self, pos: io::SeekFrom) -> io::Result<u64, STD>;
    fn fsync(&self) -> io::Result<(), STD>;
    fn datasync(&self) -> io::Result<(), STD>;
    fn truncate(&self, size: u64) -> io::Result<(), STD>;
    fn file_attr(&self) -> io::Result<STD::FileAttr, STD>;
    fn duplicate(&self) -> io::Result<Self, STD>;
    fn set_permissions(&self, perms: STD::Permissions) -> io::Result<(), STD>;
}

pub trait FileAttr<STD: Std>: Sized {
    fn file_type(&self) -> STD::FileType;
    fn size(&self) -> u64;
    fn perm(&self) -> STD::Permissions;
    fn modified(&self) -> io::Result<STD::SystemTime, STD>;
    fn accessed(&self) -> io::Result<STD::SystemTime, STD>;
    fn created(&self) -> io::Result<STD::SystemTime, STD>;
}

pub trait OpenOptions: Sized {
    fn new() -> Self;
    fn read(&mut self, read: bool);
    fn write(&mut self, write: bool);
    fn append(&mut self, append: bool);
    fn truncate(&mut self, truncate: bool);
    fn create(&mut self, create: bool);
    fn create_new(&mut self, create_new: bool);
}

pub trait Permissions: Sized + Debug {
    fn readonly(&self) -> bool;
    fn set_readonly(&mut self, readonly: bool);
}

pub trait FileType: Sized + Debug {
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;
    fn is_symlink(&self) -> bool;
}

pub trait DirBuilder<STD: Std>: Sized {
    fn new() -> Self;
    fn mkdir(&self, p: &path::Path<STD>) -> io::Result<(), STD>;
}

pub trait DirEntry<STD: Std>: Sized {
    fn path(&self) -> path::PathBuf<STD>;
    fn file_type(&self) -> io::Result<STD::FileType, STD>;
    fn file_name(&self) -> ffi::OsString<STD>;
    fn metadata(&self) -> io::Result<STD::FileAttr, STD>;
}

pub trait Stdio<STD: Std>: Sized {
    fn new() -> io::Result<Self, STD>;
}
