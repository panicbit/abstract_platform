#![unstable(feature = "abstract_platform", issue = "0")]
#![no_std]

#![feature(staged_api)]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(fixed_size_array)]
#![feature(const_fn)]
#![feature(doc_spotlight)]
#![feature(fused)]
#![feature(never_type)]
#![feature(unicode)]
#![feature(try_from)]
#![feature(array_error_internals)]
#![feature(char_error_internals)]
#![feature(toowned_clone_into)]
#![feature(str_internals)]
#![feature(rustc_attrs)]
#![feature(nonzero)]
#![feature(int_error_internals)]
#![feature(optin_builtin_traits)]
#![feature(generic_associated_types)]
#![feature(exact_size_is_empty)]

extern crate alloc;
extern crate std_unicode;

pub use core::any;
pub use core::cell;
pub use core::clone;
pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::hash;
// pub use core::intrinsics;
pub use core::iter;
pub use core::marker;
pub use core::mem;
pub use core::ops;
pub use core::ptr;
// pub use core::raw;
pub use core::result;
pub use core::option;
pub use core::isize;
pub use core::i8;
pub use core::i16;
pub use core::i32;
pub use core::i64;
// pub use core::i128;
pub use core::usize;
pub use core::u8;
pub use core::u16;
pub use core::u32;
pub use core::u64;
pub use alloc::boxed;
pub use alloc::rc;
pub use alloc::borrow;
pub use alloc::fmt;
pub use alloc::slice;
pub use alloc::str;
pub use alloc::string;
pub use alloc::vec;
pub use std_unicode::char;
// pub use core::u128;

pub mod error;
pub mod io;
pub mod os;
pub mod env;
pub mod traits;
pub mod sys_common;
pub mod ffi;
pub mod path;
// pub mod sync;
pub mod memchr;
pub mod fs;
pub mod time;
pub mod util;
pub mod num;

// Copied 1:1 from std (except for use prelude::*)
pub mod ascii;

pub mod prelude;
