// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Inspection and manipulation of the process's environment.
//!
//! This module contains functions to inspect various aspects such as
//! environment variables, process arguments, the current directory, and various
//! other important directories.
//!
//! There are several functions and structs in this module that have a
//! counterpart ending in `os`. Those ending in `os` will return an [`OsString`]
//! and those without will be returning a [`String`].
//!
//! [`OsString`]: ../../std/ffi/struct.OsString.html
//! [`String`]: ../string/struct.String.html

// #![stable(feature = "env", since = "1.0.0")]

use prelude::*;
use traits::Std;

use error::Error;
use ffi::{OsStr, OsString};
use fmt;
use io;
use path::{Path, PathBuf};

/// Returns the current working directory as a [`PathBuf`].
///
/// # Errors
///
/// Returns an [`Err`] if the current working directory value is invalid.
/// Possible cases:
///
/// * Current directory does not exist.
/// * There are insufficient permissions to access the current directory.
///
/// [`PathBuf`]: ../../std/path/struct.PathBuf.html
/// [`Err`]: ../../std/result/enum.Result.html#method.err
///
/// # Examples
///
/// ```
/// use std::env;
///
/// // We assume that we are in a valid directory.
/// let path = env::current_dir().unwrap();
/// println!("The current directory is {}", path.display());
/// ```
#[stable(feature = "env", since = "1.0.0")]
pub fn current_dir<STD: Std>() -> io::Result<PathBuf<STD>, STD> {
    STD::current_dir()
}

/// Changes the current working directory to the specified path.
///
/// Returns an [`Err`] if the operation fails.
///
/// [`Err`]: ../../std/result/enum.Result.html#method.err
///
/// # Examples
///
/// ```
/// use std::env;
/// use std::path::Path;
///
/// let root = Path::new("/");
/// assert!(env::set_current_dir(&root).is_ok());
/// println!("Successfully changed working directory to {}!", root.display());
/// ```
#[stable(feature = "env", since = "1.0.0")]
pub fn set_current_dir<STD: Std, P: AsRef<Path<STD>>>(path: P) -> io::Result<(), STD> {
    STD::set_current_dir(path.as_ref())
}

/// An iterator over a snapshot of the environment variables of this process.
///
/// This structure is created by the [`std::env::vars`] function. See its
/// documentation for more.
///
/// [`std::env::vars`]: fn.vars.html
#[stable(feature = "env", since = "1.0.0")]
pub struct Vars<STD: Std> { inner: VarsOs<STD> }

/// An iterator over a snapshot of the environment variables of this process.
///
/// This structure is created by the [`std::env::vars_os`] function. See
/// its documentation for more.
///
/// [`std::env::vars_os`]: fn.vars_os.html
#[stable(feature = "env", since = "1.0.0")]
pub struct VarsOs<STD: Std> { inner: STD::VarsOs }

/// Returns an iterator of (variable, value) pairs of strings, for all the
/// environment variables of the current process.
///
/// The returned iterator contains a snapshot of the process's environment
/// variables at the time of this invocation. Modifications to environment
/// variables afterwards will not be reflected in the returned iterator.
///
/// # Panics
///
/// While iterating, the returned iterator will panic if any key or value in the
/// environment is not valid unicode. If this is not desired, consider using the
/// [`env::vars_os`] function.
///
/// [`env::vars_os`]: fn.vars_os.html
///
/// # Examples
///
/// ```
/// use std::env;
///
/// // We will iterate through the references to the element returned by
/// // env::vars();
/// for (key, value) in env::vars() {
///     println!("{}: {}", key, value);
/// }
/// ```
#[stable(feature = "env", since = "1.0.0")]
pub fn vars<STD: Std>() -> Vars<STD> {
    Vars { inner: vars_os() }
}

/// Returns an iterator of (variable, value) pairs of OS strings, for all the
/// environment variables of the current process.
///
/// The returned iterator contains a snapshot of the process's environment
/// variables at the time of this invocation. Modifications to environment
/// variables afterwards will not be reflected in the returned iterator.
///
/// # Examples
///
/// ```
/// use std::env;
///
/// // We will iterate through the references to the element returned by
/// // env::vars_os();
/// for (key, value) in env::vars_os() {
///     println!("{:?}: {:?}", key, value);
/// }
/// ```
#[stable(feature = "env", since = "1.0.0")]
pub fn vars_os<STD: Std>() -> VarsOs<STD> {
    VarsOs { inner: STD::vars_os() }
}

#[stable(feature = "env", since = "1.0.0")]
impl<STD: Std> Iterator for Vars<STD> {
    type Item = (String, String);
    fn next(&mut self) -> Option<(String, String)> {
        self.inner.next().map(|(a, b)| {
            (a.into_string().unwrap(), b.into_string().unwrap())
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) { self.inner.size_hint() }
}

#[stable(feature = "std_debug", since = "1.16.0")]
impl<STD: Std> fmt::Debug for Vars<STD> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("Vars { .. }")
    }
}

#[stable(feature = "env", since = "1.0.0")]
impl<STD: Std> Iterator for VarsOs<STD> {
    type Item = (OsString<STD>, OsString<STD>);
    fn next(&mut self) -> Option<(OsString<STD>, OsString<STD>)> { self.inner.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.inner.size_hint() }
}

#[stable(feature = "std_debug", since = "1.16.0")]
impl<STD: Std> fmt::Debug for VarsOs<STD> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("VarsOs { .. }")
    }
}

/// Fetches the environment variable `key` from the current process.
///
/// # Errors
///
/// * Environment variable is not present
/// * Environment variable is not valid unicode
///
/// # Examples
///
/// ```
/// use std::env;
///
/// let key = "HOME";
/// match env::var(key) {
///     Ok(val) => println!("{}: {:?}", key, val),
///     Err(e) => println!("couldn't interpret {}: {}", key, e),
/// }
/// ```
#[stable(feature = "env", since = "1.0.0")]
pub fn var<STD: Std, K: AsRef<OsStr<STD>>>(key: K) -> Result<String, VarError<STD>> {
    _var(key.as_ref())
}

fn _var<STD: Std>(key: &OsStr<STD>) -> Result<String, VarError<STD>> {
    match var_os(key) {
        Some(s) => s.into_string().map_err(VarError::NotUnicode),
        None => Err(VarError::NotPresent),
    }
}

/// Fetches the environment variable `key` from the current process, returning
/// [`None`] if the variable isn't set.
///
/// [`None`]: ../option/enum.Option.html#variant.None
///
/// # Examples
///
/// ```
/// use std::env;
///
/// let key = "HOME";
/// match env::var_os(key) {
///     Some(val) => println!("{}: {:?}", key, val),
///     None => println!("{} is not defined in the environment.", key)
/// }
/// ```
#[stable(feature = "env", since = "1.0.0")]
pub fn var_os<STD: Std, K: AsRef<OsStr<STD>>>(key: K) -> Option<OsString<STD>> {
    _var_os(key.as_ref())
}

fn _var_os<STD: Std>(key: &OsStr<STD>) -> Option<OsString<STD>> {
    STD::getenv(key).unwrap_or_else(|e| {
        panic!("failed to get environment variable `{:?}`: {}", key, e)
    })
}

/// The error type for operations interacting with environment variables.
/// Possibly returned from the [`env::var`] function.
///
/// [`env::var`]: fn.var.html
#[derive(Debug, PartialEq, Eq, Clone)]
#[stable(feature = "env", since = "1.0.0")]
pub enum VarError<STD> {
    /// The specified environment variable was not present in the current
    /// process's environment.
    #[stable(feature = "env", since = "1.0.0")]
    NotPresent,

    /// The specified environment variable was found, but it did not contain
    /// valid unicode data. The found data is returned as a payload of this
    /// variant.
    #[stable(feature = "env", since = "1.0.0")]
    NotUnicode(#[stable(feature = "env", since = "1.0.0")] OsString<STD>),
}

#[stable(feature = "env", since = "1.0.0")]
impl<STD: Std> fmt::Display for VarError<STD> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VarError::NotPresent => write!(f, "environment variable not found"),
            VarError::NotUnicode(ref s) => {
                write!(f, "environment variable was not valid unicode: {:?}", s)
            }
        }
    }
}

#[stable(feature = "env", since = "1.0.0")]
impl<STD: Std> Error for VarError<STD> {
    fn description(&self) -> &str {
        match *self {
            VarError::NotPresent => "environment variable not found",
            VarError::NotUnicode(..) => "environment variable was not valid unicode",
        }
    }
}

/// Sets the environment variable `k` to the value `v` for the currently running
/// process.
///
/// Note that while concurrent access to environment variables is safe in Rust,
/// some platforms only expose inherently unsafe non-threadsafe APIs for
/// inspecting the environment. As a result extra care needs to be taken when
/// auditing calls to unsafe external FFI functions to ensure that any external
/// environment accesses are properly synchronized with accesses in Rust.
///
/// Discussion of this unsafety on Unix may be found in:
///
///  - [Austin Group Bugzilla](http://austingroupbugs.net/view.php?id=188)
///  - [GNU C library Bugzilla](https://sourceware.org/bugzilla/show_bug.cgi?id=15607#c2)
///
/// # Panics
///
/// This function may panic if `key` is empty, contains an ASCII equals sign
/// `'='` or the NUL character `'\0'`, or when the value contains the NUL
/// character.
///
/// # Examples
///
/// ```
/// use std::env;
///
/// let key = "KEY";
/// env::set_var(key, "VALUE");
/// assert_eq!(env::var(key), Ok("VALUE".to_string()));
/// ```
#[stable(feature = "env", since = "1.0.0")]
pub fn set_var<STD: Std, K: AsRef<OsStr<STD>>, V: AsRef<OsStr<STD>>>(k: K, v: V) {
    _set_var(k.as_ref(), v.as_ref())
}

fn _set_var<STD: Std>(k: &OsStr<STD>, v: &OsStr<STD>) {
    STD::setenv(k, v).unwrap_or_else(|e| {
        panic!("failed to set environment variable `{:?}` to `{:?}`: {}",
               k, v, e)
    })
}

/// Removes an environment variable from the environment of the currently running process.
///
/// Note that while concurrent access to environment variables is safe in Rust,
/// some platforms only expose inherently unsafe non-threadsafe APIs for
/// inspecting the environment. As a result extra care needs to be taken when
/// auditing calls to unsafe external FFI functions to ensure that any external
/// environment accesses are properly synchronized with accesses in Rust.
///
/// Discussion of this unsafety on Unix may be found in:
///
///  - [Austin Group Bugzilla](http://austingroupbugs.net/view.php?id=188)
///  - [GNU C library Bugzilla](https://sourceware.org/bugzilla/show_bug.cgi?id=15607#c2)
///
/// # Panics
///
/// This function may panic if `key` is empty, contains an ASCII equals sign
/// `'='` or the NUL character `'\0'`, or when the value contains the NUL
/// character.
///
/// # Examples
///
/// ```
/// use std::env;
///
/// let key = "KEY";
/// env::set_var(key, "VALUE");
/// assert_eq!(env::var(key), Ok("VALUE".to_string()));
///
/// env::remove_var(key);
/// assert!(env::var(key).is_err());
/// ```
#[stable(feature = "env", since = "1.0.0")]
pub fn remove_var<STD: Std, K: AsRef<OsStr<STD>>>(k: K) {
    _remove_var(k.as_ref())
}

fn _remove_var<STD: Std>(k: &OsStr<STD>) {
    STD::unsetenv(k).unwrap_or_else(|e| {
        panic!("failed to remove environment variable `{:?}`: {}", k, e)
    })
}

/// An iterator that splits an environment variable into paths according to
/// platform-specific conventions.
///
/// This structure is created by the [`std::env::split_paths`] function. See its
/// documentation for more.
///
/// [`std::env::split_paths`]: fn.split_paths.html
#[stable(feature = "env", since = "1.0.0")]
pub struct SplitPaths<'a, STD: Std> { inner: STD::SplitPaths<'a> }

/// Parses input according to platform conventions for the `PATH`
/// environment variable.
///
/// Returns an iterator over the paths contained in `unparsed`.
///
/// # Examples
///
/// ```
/// use std::env;
///
/// let key = "PATH";
/// match env::var_os(key) {
///     Some(paths) => {
///         for path in env::split_paths(&paths) {
///             println!("'{}'", path.display());
///         }
///     }
///     None => println!("{} is not defined in the environment.", key)
/// }
/// ```
#[stable(feature = "env", since = "1.0.0")]
pub fn split_paths<STD: Std, T: AsRef<OsStr<STD>> + ?Sized>(unparsed: &T) -> SplitPaths<STD> {
    SplitPaths { inner: STD::split_paths(unparsed.as_ref()) }
}

#[stable(feature = "env", since = "1.0.0")]
impl<'a, STD: Std> Iterator for SplitPaths<'a, STD> {
    type Item = PathBuf<STD>;
    fn next(&mut self) -> Option<PathBuf<STD>> { self.inner.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.inner.size_hint() }
}

#[stable(feature = "std_debug", since = "1.16.0")]
impl<'a, STD: Std> fmt::Debug for SplitPaths<'a, STD> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("SplitPaths { .. }")
    }
}

/// The error type for operations on the `PATH` variable. Possibly returned from
/// the [`env::join_paths`] function.
///
/// [`env::join_paths`]: fn.join_paths.html
#[derive(Debug)]
#[stable(feature = "env", since = "1.0.0")]
pub struct JoinPathsError<STD: Std> {
    inner: STD::JoinPathsError
}

/// Joins a collection of [`Path`]s appropriately for the `PATH`
/// environment variable.
///
/// # Errors
///
/// Returns an [`Err`][err] (containing an error message) if one of the input
/// [`Path`]s contains an invalid character for constructing the `PATH`
/// variable (a double quote on Windows or a colon on Unix).
///
/// [`Path`]: ../../std/path/struct.Path.html
/// [`OsString`]: ../../std/ffi/struct.OsString.html
/// [err]: ../../std/result/enum.Result.html#variant.Err
///
/// # Examples
///
/// Joining paths on a Unix-like platform:
///
/// ```
/// # if cfg!(unix) {
/// use std::env;
/// use std::ffi::OsString;
/// use std::path::Path;
///
/// let paths = [Path::new("/bin"), Path::new("/usr/bin")];
/// let path_os_string = env::join_paths(paths.iter()).unwrap();
/// assert_eq!(path_os_string, OsString::from("/bin:/usr/bin"));
/// # }
/// ```
///
/// Joining a path containing a colon on a Unix-like platform results in an error:
///
/// ```
/// # if cfg!(unix) {
/// use std::env;
/// use std::path::Path;
///
/// let paths = [Path::new("/bin"), Path::new("/usr/bi:n")];
/// assert!(env::join_paths(paths.iter()).is_err());
/// # }
/// ```
///
/// Using `env::join_paths` with `env::spit_paths` to append an item to the `PATH` environment
/// variable:
///
/// ```
/// use std::env;
/// use std::path::PathBuf;
///
/// if let Some(path) = env::var_os("PATH") {
///     let mut paths = env::split_paths(&path).collect::<Vec<_>>();
///     paths.push(PathBuf::from("/home/xyz/bin"));
///     let new_path = env::join_paths(paths).unwrap();
///     env::set_var("PATH", &new_path);
/// }
/// ```
#[stable(feature = "env", since = "1.0.0")]
pub fn join_paths<STD: Std, I, T>(paths: I) -> Result<OsString<STD>, JoinPathsError<STD>>
    where I: IntoIterator<Item=T>, T: AsRef<OsStr<STD>>
{
    STD::join_paths(paths.into_iter()).map_err(|e| {
        JoinPathsError { inner: e }
    })
}

#[stable(feature = "env", since = "1.0.0")]
impl<STD: Std> fmt::Display for JoinPathsError<STD> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

#[stable(feature = "env", since = "1.0.0")]
impl<STD: Std> Error for JoinPathsError<STD> {
    fn description(&self) -> &str { self.inner.description() }
}

/// Returns the path of the current user's home directory if known.
///
/// # Unix
///
/// Returns the value of the 'HOME' environment variable if it is set
/// and not equal to the empty string. Otherwise, it tries to determine the
/// home directory by invoking the `getpwuid_r` function on the UID of the
/// current user.
///
/// # Windows
///
/// Returns the value of the 'HOME' environment variable if it is
/// set and not equal to the empty string. Otherwise, returns the value of the
/// 'USERPROFILE' environment variable if it is set and not equal to the empty
/// string. If both do not exist, [`GetUserProfileDirectory`][msdn] is used to
/// return the appropriate path.
///
/// [msdn]: https://msdn.microsoft.com/en-us/library/windows/desktop/bb762280(v=vs.85).aspx
///
/// # Examples
///
/// ```
/// use std::env;
///
/// match env::home_dir() {
///     Some(path) => println!("{}", path.display()),
///     None => println!("Impossible to get your home dir!"),
/// }
/// ```
#[stable(feature = "env", since = "1.0.0")]
pub fn home_dir<STD: Std>() -> Option<PathBuf<STD>> {
    STD::home_dir()
}

/// Returns the path of a temporary directory.
///
/// # Unix
///
/// Returns the value of the `TMPDIR` environment variable if it is
/// set, otherwise for non-Android it returns `/tmp`. If Android, since there
/// is no global temporary folder (it is usually allocated per-app), it returns
/// `/data/local/tmp`.
///
/// # Windows
///
/// Returns the value of, in order, the `TMP`, `TEMP`,
/// `USERPROFILE` environment variable if any are set and not the empty
/// string. Otherwise, `temp_dir` returns the path of the Windows directory.
/// This behavior is identical to that of [`GetTempPath`][msdn], which this
/// function uses internally.
///
/// [msdn]: https://msdn.microsoft.com/en-us/library/windows/desktop/aa364992(v=vs.85).aspx
///
/// ```no_run
/// use std::env;
/// use std::fs::File;
///
/// fn main() -> std::io::Result<(), STD> {
///     let mut dir = env::temp_dir();
///     dir.push("foo.txt");
///
///     let f = File::create(dir)?;
///     Ok(())
/// }
/// ```
#[stable(feature = "env", since = "1.0.0")]
pub fn temp_dir<STD: Std>() -> PathBuf<STD> {
    STD::temp_dir()
}

/// Returns the full filesystem path of the current running executable.
///
/// # Platform-specific behavior
///
/// If the executable was invoked through a symbolic link, some platforms will
/// return the path of the symbolic link and other platforms will return the
/// path of the symbolic link’s target.
///
/// # Errors
///
/// Acquiring the path of the current executable is a platform-specific operation
/// that can fail for a good number of reasons. Some errors can include, but not
/// be limited to, filesystem operations failing or general syscall failures.
///
/// # Security
///
/// The output of this function should not be used in anything that might have
/// security implications. For example:
///
/// ```
/// fn main() {
///     println!("{:?}", std::env::current_exe());
/// }
/// ```
///
/// On Linux systems, if this is compiled as `foo`:
///
/// ```bash
/// $ rustc foo.rs
/// $ ./foo
/// Ok("/home/alex/foo")
/// ```
///
/// And you make a hard link of the program:
///
/// ```bash
/// $ ln foo bar
/// ```
///
/// When you run it, you won’t get the path of the original executable, you’ll
/// get the path of the hard link:
///
/// ```bash
/// $ ./bar
/// Ok("/home/alex/bar")
/// ```
///
/// This sort of behavior has been known to [lead to privilege escalation] when
/// used incorrectly.
///
/// [lead to privilege escalation]: https://securityvulns.com/Wdocument183.html
///
/// # Examples
///
/// ```
/// use std::env;
///
/// match env::current_exe() {
///     Ok(exe_path) => println!("Path of this executable is: {}",
///                              exe_path.display()),
///     Err(e) => println!("failed to get current exe path: {}", e),
/// };
/// ```
#[stable(feature = "env", since = "1.0.0")]
pub fn current_exe<STD: Std>() -> io::Result<PathBuf<STD>, STD> {
    STD::current_exe()
}

/// An iterator over the arguments of a process, yielding a [`String`] value for
/// each argument.
///
/// This struct is created by the [`std::env::args`] function. See its
/// documentation for more.
///
/// The first element is traditionally the path of the executable, but it can be
/// set to arbitrary text, and may not even exist. This means this property
/// should not be relied upon for security purposes.
///
/// [`String`]: ../string/struct.String.html
/// [`std::env::args`]: ./fn.args.html
#[stable(feature = "env", since = "1.0.0")]
pub struct Args<STD: Std> { inner: ArgsOs<STD> }

/// An iterator over the arguments of a process, yielding an [`OsString`] value
/// for each argument.
///
/// This struct is created by the [`std::env::args_os`] function. See its
/// documentation for more.
///
/// The first element is traditionally the path of the executable, but it can be
/// set to arbitrary text, and may not even exist. This means this property
/// should not be relied upon for security purposes.
///
/// [`OsString`]: ../ffi/struct.OsString.html
/// [`std::env::args_os`]: ./fn.args_os.html
#[stable(feature = "env", since = "1.0.0")]
pub struct ArgsOs<STD: Std> { inner: STD::ArgsOs }

/// Returns the arguments which this program was started with (normally passed
/// via the command line).
///
/// The first element is traditionally the path of the executable, but it can be
/// set to arbitrary text, and may not even exist. This means this property should
/// not be relied upon for security purposes.
///
/// On Unix systems shell usually expands unquoted arguments with glob patterns
/// (such as `*` and `?`). On Windows this is not done, and such arguments are
/// passed as-is.
///
/// # Panics
///
/// The returned iterator will panic during iteration if any argument to the
/// process is not valid unicode. If this is not desired,
/// use the [`args_os`] function instead.
///
/// # Examples
///
/// ```
/// use std::env;
///
/// // Prints each argument on a separate line
/// for argument in env::args() {
///     println!("{}", argument);
/// }
/// ```
///
/// [`args_os`]: ./fn.args_os.html
#[stable(feature = "env", since = "1.0.0")]
pub fn args<STD: Std>() -> Args<STD> {
    Args { inner: args_os() }
}

/// Returns the arguments which this program was started with (normally passed
/// via the command line).
///
/// The first element is traditionally the path of the executable, but it can be
/// set to arbitrary text, and it may not even exist, so this property should
/// not be relied upon for security purposes.
///
/// # Examples
///
/// ```
/// use std::env;
///
/// // Prints each argument on a separate line
/// for argument in env::args_os() {
///     println!("{:?}", argument);
/// }
/// ```
#[stable(feature = "env", since = "1.0.0")]
pub fn args_os<STD: Std>() -> ArgsOs<STD> {
    ArgsOs { inner: STD::args_os() }
}

#[stable(feature = "env_unimpl_send_sync", since = "1.25.0")]
impl<STD> !Send for Args<STD> {}

#[stable(feature = "env_unimpl_send_sync", since = "1.25.0")]
impl<STD> !Sync for Args<STD> {}

#[stable(feature = "env", since = "1.0.0")]
impl<STD: Std> Iterator for Args<STD> {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        self.inner.next().map(|s| s.into_string().unwrap())
    }
    fn size_hint(&self) -> (usize, Option<usize>) { self.inner.size_hint() }
}

#[stable(feature = "env", since = "1.0.0")]
impl<STD: Std> ExactSizeIterator for Args<STD> {
    fn len(&self) -> usize { self.inner.len() }
    fn is_empty(&self) -> bool { self.inner.is_empty() }
}

#[stable(feature = "env_iterators", since = "1.12.0")]
impl<STD: Std> DoubleEndedIterator for Args<STD> {
    fn next_back(&mut self) -> Option<String> {
        self.inner.next_back().map(|s| s.into_string().unwrap())
    }
}

#[stable(feature = "std_debug", since = "1.16.0")]
impl<STD: Std> fmt::Debug for Args<STD> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Args")
            .field("inner", &self.inner.inner.inner_debug())
            .finish()
    }
}

#[stable(feature = "env_unimpl_send_sync", since = "1.25.0")]
impl<STD> !Send for ArgsOs<STD> {}

#[stable(feature = "env_unimpl_send_sync", since = "1.25.0")]
impl<STD> !Sync for ArgsOs<STD> {}

#[stable(feature = "env", since = "1.0.0")]
impl<STD: Std> Iterator for ArgsOs<STD> {
    type Item = OsString<STD>;
    fn next(&mut self) -> Option<OsString<STD>> { self.inner.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.inner.size_hint() }
}

#[stable(feature = "env", since = "1.0.0")]
impl<STD: Std> ExactSizeIterator for ArgsOs<STD> {
    fn len(&self) -> usize { self.inner.len() }
    fn is_empty(&self) -> bool { self.inner.is_empty() }
}

#[stable(feature = "env_iterators", since = "1.12.0")]
impl<STD: Std> DoubleEndedIterator for ArgsOs<STD> {
    fn next_back(&mut self) -> Option<OsString<STD>> { self.inner.next_back() }
}

#[stable(feature = "std_debug", since = "1.16.0")]
impl<STD: Std> fmt::Debug for ArgsOs<STD> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ArgsOs")
            .field("inner", &self.inner.inner_debug())
            .finish()
    }
}

/// Constants associated with the current target
#[stable(feature = "env", since = "1.0.0")]
pub mod consts {
    use traits::Std;
    /// A string describing the architecture of the CPU that is currently
    /// in use.
    ///
    /// Some possible values:
    ///
    /// - x86
    /// - x86_64
    /// - arm
    /// - aarch64
    /// - mips
    /// - mips64
    /// - powerpc
    /// - powerpc64
    /// - s390x
    /// - sparc64
    #[stable(feature = "env", since = "1.0.0")]
    pub const fn arch<STD: Std>() -> &'static str { STD::ARCH }

    /// The family of the operating system. Example value is `unix`.
    ///
    /// Some possible values:
    ///
    /// - unix
    /// - windows
    #[stable(feature = "env", since = "1.0.0")]
    pub const fn family<STD: Std>() -> &'static str { STD::FAMILY }

    /// A string describing the specific operating system in use.
    /// Example value is `linux`.
    ///
    /// Some possible values:
    ///
    /// - linux
    /// - macos
    /// - ios
    /// - freebsd
    /// - dragonfly
    /// - bitrig
    /// - netbsd
    /// - openbsd
    /// - solaris
    /// - android
    /// - windows
    #[stable(feature = "env", since = "1.0.0")]
    pub const fn os<STD: Std>() -> &'static str { STD::OS }

    /// Specifies the filename prefix used for shared libraries on this
    /// platform. Example value is `lib`.
    ///
    /// Some possible values:
    ///
    /// - lib
    /// - `""` (an empty string)
    #[stable(feature = "env", since = "1.0.0")]
    pub const fn dll_prefix<STD: Std>() -> &'static str { STD::DLL_PREFIX }

    /// Specifies the filename suffix used for shared libraries on this
    /// platform. Example value is `.so`.
    ///
    /// Some possible values:
    ///
    /// - .so
    /// - .dylib
    /// - .dll
    #[stable(feature = "env", since = "1.0.0")]
    pub const fn dll_suffix<STD: Std>() -> &'static str { STD::DLL_SUFFIX }

    /// Specifies the file extension used for shared libraries on this
    /// platform that goes after the dot. Example value is `so`.
    ///
    /// Some possible values:
    ///
    /// - so
    /// - dylib
    /// - dll
    #[stable(feature = "env", since = "1.0.0")]
    pub const fn dll_extension<STD: Std>() -> &'static str { STD::DLL_EXTENSION }

    /// Specifies the filename suffix used for executable binaries on this
    /// platform. Example value is `.exe`.
    ///
    /// Some possible values:
    ///
    /// - .exe
    /// - .nexe
    /// - .pexe
    /// - `""` (an empty string)
    #[stable(feature = "env", since = "1.0.0")]
    pub const fn exe_suffix<STD: Std>() -> &'static str { STD::EXE_SUFFIX }

    /// Specifies the file extension, if any, used for executable binaries
    /// on this platform. Example value is `exe`.
    ///
    /// Some possible values:
    ///
    /// - exe
    /// - `""` (an empty string)
    #[stable(feature = "env", since = "1.0.0")]
    pub const fn exe_extension<STD: Std>() -> &'static str { STD::EXE_EXTENSION }
}

#[cfg(target_arch = "x86")]
mod arch {
    pub const ARCH: &'static str = "x86";
}

#[cfg(target_arch = "x86_64")]
mod arch {
    pub const ARCH: &'static str = "x86_64";
}

#[cfg(target_arch = "arm")]
mod arch {
    pub const ARCH: &'static str = "arm";
}

#[cfg(target_arch = "aarch64")]
mod arch {
    pub const ARCH: &'static str = "aarch64";
}

#[cfg(target_arch = "mips")]
mod arch {
    pub const ARCH: &'static str = "mips";
}

#[cfg(target_arch = "mips64")]
mod arch {
    pub const ARCH: &'static str = "mips64";
}

#[cfg(target_arch = "powerpc")]
mod arch {
    pub const ARCH: &'static str = "powerpc";
}

#[cfg(target_arch = "powerpc64")]
mod arch {
    pub const ARCH: &'static str = "powerpc64";
}

#[cfg(target_arch = "s390x")]
mod arch {
    pub const ARCH: &'static str = "s390x";
}

#[cfg(target_arch = "sparc64")]
mod arch {
    pub const ARCH: &'static str = "sparc64";
}

#[cfg(target_arch = "le32")]
mod arch {
    pub const ARCH: &'static str = "le32";
}

#[cfg(target_arch = "asmjs")]
mod arch {
    pub const ARCH: &'static str = "asmjs";
}

#[cfg(target_arch = "wasm32")]
mod arch {
    pub const ARCH: &'static str = "wasm32";
}

#[cfg(test)]
mod tests {
    use super::*;

    use path::Path;

    #[test]
    #[cfg_attr(target_os = "emscripten", ignore)]
    fn test_self_exe_path() {
        let path = current_exe();
        assert!(path.is_ok());
        let path = path.unwrap();

        // Hard to test this function
        assert!(path.is_absolute());
    }

    #[test]
    fn test() {
        assert!((!Path::new("test-path").is_absolute()));

        current_dir().unwrap();
    }

    #[test]
    #[cfg(windows)]
    fn split_paths_windows() {
        use path::PathBuf;

        fn check_parse(unparsed: &str, parsed: &[&str]) -> bool {
            split_paths(unparsed).collect::<Vec<_>>() ==
                parsed.iter().map(|s| PathBuf::from(*s)).collect::<Vec<_>>()
        }

        assert!(check_parse("", &mut [""]));
        assert!(check_parse(r#""""#, &mut [""]));
        assert!(check_parse(";;", &mut ["", "", ""]));
        assert!(check_parse(r"c:\", &mut [r"c:\"]));
        assert!(check_parse(r"c:\;", &mut [r"c:\", ""]));
        assert!(check_parse(r"c:\;c:\Program Files\",
                            &mut [r"c:\", r"c:\Program Files\"]));
        assert!(check_parse(r#"c:\;c:\"foo"\"#, &mut [r"c:\", r"c:\foo\"]));
        assert!(check_parse(r#"c:\;c:\"foo;bar"\;c:\baz"#,
                            &mut [r"c:\", r"c:\foo;bar\", r"c:\baz"]));
    }

    #[test]
    #[cfg(unix)]
    fn split_paths_unix() {
        use path::PathBuf;

        fn check_parse(unparsed: &str, parsed: &[&str]) -> bool {
            split_paths(unparsed).collect::<Vec<_>>() ==
                parsed.iter().map(|s| PathBuf::from(*s)).collect::<Vec<_>>()
        }

        assert!(check_parse("", &mut [""]));
        assert!(check_parse("::", &mut ["", "", ""]));
        assert!(check_parse("/", &mut ["/"]));
        assert!(check_parse("/:", &mut ["/", ""]));
        assert!(check_parse("/:/usr/local", &mut ["/", "/usr/local"]));
    }

    #[test]
    #[cfg(unix)]
    fn join_paths_unix() {
        use ffi::OsStr;

        fn test_eq(input: &[&str], output: &str) -> bool {
            &*join_paths(input.iter().cloned()).unwrap() ==
                OsStr::new(output)
        }

        assert!(test_eq(&[], ""));
        assert!(test_eq(&["/bin", "/usr/bin", "/usr/local/bin"],
                         "/bin:/usr/bin:/usr/local/bin"));
        assert!(test_eq(&["", "/bin", "", "", "/usr/bin", ""],
                         ":/bin:::/usr/bin:"));
        assert!(join_paths(["/te:st"].iter().cloned()).is_err());
    }

    #[test]
    #[cfg(windows)]
    fn join_paths_windows() {
        use ffi::OsStr;

        fn test_eq(input: &[&str], output: &str) -> bool {
            &*join_paths(input.iter().cloned()).unwrap() ==
                OsStr::new(output)
        }

        assert!(test_eq(&[], ""));
        assert!(test_eq(&[r"c:\windows", r"c:\"],
                        r"c:\windows;c:\"));
        assert!(test_eq(&["", r"c:\windows", "", "", r"c:\", ""],
                        r";c:\windows;;;c:\;"));
        assert!(test_eq(&[r"c:\te;st", r"c:\"],
                        r#""c:\te;st";c:\"#));
        assert!(join_paths([r#"c:\te"st"#].iter().cloned()).is_err());
    }

    #[test]
    fn args_debug() {
        assert_eq!(
            format!("Args {{ inner: {:?} }}", args().collect::<Vec<_>>()),
            format!("{:?}", args()));
        assert_eq!(
            format!("ArgsOs {{ inner: {:?} }}", args_os().collect::<Vec<_>>()),
            format!("{:?}", args_os()));
    }
}