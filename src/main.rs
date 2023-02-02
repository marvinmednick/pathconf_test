// Generate a `build` method to go from builder to original struct.
//
// This method should require that every one of the fields has been explicitly
// set; it should return an error if a field is missing. The precise error type
// is not important. Consider using Box<dyn Error>, which you can construct
// using the impl From<String> for Box<dyn Error>.
//
//     impl CommandBuilder {
//         pub fn build(&mut self) -> Result<Command, Box<dyn Error>> {
//             ...
//         }
//     }

use enum_dict::{Builder, EnumToDict};

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}

#[derive(Debug)]
enum MTest {
    OptA = 12,
    OptB = 13,
    OptC = 3,
}

#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Debug, EnumToDict)]
pub enum TestConfVar {
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "linux",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "redox"
    ))]
    /// Minimum number of bits needed to represent, as a signed integer value,
    /// the maximum size of a regular file allowed in the specified directory.
    PC_FILESIZEBITS = libc::_PC_FILESIZEBITS,
    /// Maximum number of links to a single file.
    PC_LINK_MAX = libc::_PC_LINK_MAX,
    /// Maximum number of bytes in a terminal canonical input line.
    PC_MAX_CANON = libc::_PC_MAX_CANON,
    /// Minimum number of bytes for which space is available in a terminal input
    /// queue; therefore, the maximum number of bytes a conforming application
    /// may require to be typed as input before reading them.
    PC_MAX_INPUT = libc::_PC_MAX_INPUT,
    /// Maximum number of bytes in a filename (not including the terminating
    /// null of a filename string).
    PC_NAME_MAX = libc::_PC_NAME_MAX,
}

#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum PathconfVar {
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "linux",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "redox"
    ))]
    /// Minimum number of bits needed to represent, as a signed integer value,
    /// the maximum size of a regular file allowed in the specified directory.
    PC_FILESIZEBITS = libc::_PC_FILESIZEBITS,
    /// Maximum number of links to a single file.
    PC_LINK_MAX = libc::_PC_LINK_MAX,
    /// Maximum number of bytes in a terminal canonical input line.
    PC_MAX_CANON = libc::_PC_MAX_CANON,
    /// Minimum number of bytes for which space is available in a terminal input
    /// queue; therefore, the maximum number of bytes a conforming application
    /// may require to be typed as input before reading them.
    PC_MAX_INPUT = libc::_PC_MAX_INPUT,
    /// Maximum number of bytes in a filename (not including the terminating
    /// null of a filename string).
    PC_NAME_MAX = libc::_PC_NAME_MAX,
    /// Maximum number of bytes the implementation will store as a pathname in a
    /// user-supplied buffer of unspecified size, including the terminating null
    /// character. Minimum number the implementation will accept as the maximum
    /// number of bytes in a pathname.
    PC_PATH_MAX = libc::_PC_PATH_MAX,
    /// Maximum number of bytes that is guaranteed to be atomic when writing to
    /// a pipe.
    PC_PIPE_BUF = libc::_PC_PIPE_BUF,
    #[cfg(any(
        target_os = "android",
        target_os = "dragonfly",
        target_os = "illumos",
        target_os = "linux",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "redox",
        target_os = "solaris"
    ))]
    /// Symbolic links can be created.
    PC_2_SYMLINKS = libc::_PC_2_SYMLINKS,
    #[cfg(any(
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "linux",
        target_os = "openbsd",
        target_os = "redox"
    ))]
    /// Minimum number of bytes of storage actually allocated for any portion of
    /// a file.
    PC_ALLOC_SIZE_MIN = libc::_PC_ALLOC_SIZE_MIN,
    #[cfg(any(
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "linux",
        target_os = "openbsd"
    ))]
    /// Recommended increment for file transfer sizes between the
    /// `POSIX_REC_MIN_XFER_SIZE` and `POSIX_REC_MAX_XFER_SIZE` values.
    PC_REC_INCR_XFER_SIZE = libc::_PC_REC_INCR_XFER_SIZE,
    #[cfg(any(
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "linux",
        target_os = "openbsd",
        target_os = "redox"
    ))]
    /// Maximum recommended file transfer size.
    PC_REC_MAX_XFER_SIZE = libc::_PC_REC_MAX_XFER_SIZE,
    #[cfg(any(
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "linux",
        target_os = "openbsd",
        target_os = "redox"
    ))]
    /// Minimum recommended file transfer size.
    PC_REC_MIN_XFER_SIZE = libc::_PC_REC_MIN_XFER_SIZE,
    #[cfg(any(
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "linux",
        target_os = "openbsd",
        target_os = "redox"
    ))]
    ///  Recommended file transfer buffer alignment.
    PC_REC_XFER_ALIGN = libc::_PC_REC_XFER_ALIGN,
    #[cfg(any(
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "illumos",
        target_os = "linux",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "redox",
        target_os = "solaris"
    ))]
    /// Maximum number of bytes in a symbolic link.
    PC_SYMLINK_MAX = libc::_PC_SYMLINK_MAX,
    /// The use of `chown` and `fchown` is restricted to a process with
    /// appropriate privileges, and to changing the group ID of a file only to
    /// the effective group ID of the process or to one of its supplementary
    /// group IDs.
    PC_CHOWN_RESTRICTED = libc::_PC_CHOWN_RESTRICTED,
    /// Pathname components longer than {NAME_MAX} generate an error.
    PC_NO_TRUNC = libc::_PC_NO_TRUNC,
    /// This symbol shall be defined to be the value of a character that shall
    /// disable terminal special character handling.
    PC_VDISABLE = libc::_PC_VDISABLE,
    #[cfg(any(
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "illumos",
        target_os = "linux",
        target_os = "openbsd",
        target_os = "redox",
        target_os = "solaris"
    ))]
    /// Asynchronous input or output operations may be performed for the
    /// associated file.
    PC_ASYNC_IO = libc::_PC_ASYNC_IO,
    #[cfg(any(
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "illumos",
        target_os = "linux",
        target_os = "openbsd",
        target_os = "redox",
        target_os = "solaris"
    ))]
    /// Prioritized input or output operations may be performed for the
    /// associated file.
    PC_PRIO_IO = libc::_PC_PRIO_IO,
    #[cfg(any(
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "illumos",
        target_os = "linux",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "redox",
        target_os = "solaris"
    ))]
    /// Synchronized input or output operations may be performed for the
    /// associated file.
    PC_SYNC_IO = libc::_PC_SYNC_IO,
    #[cfg(any(target_os = "dragonfly", target_os = "openbsd"))]
    /// The resolution in nanoseconds for all file timestamps.
    PC_TIMESTAMP_RESOLUTION = libc::_PC_TIMESTAMP_RESOLUTION,
}

fn main() {
    let mut builder = Command::builder();
    builder.executable("cargo".to_owned());
    builder.args(vec!["build".to_owned(), "--release".to_owned()]);
    builder.env(vec![]);
    builder.current_dir("..".to_owned());

    let command = builder.build().unwrap();
    assert_eq!(command.executable, "cargo");
    println!("{:#?}", PathconfVar::PC_FILESIZEBITS as u32);
    println!("{:#?}", libc::_PC_FILESIZEBITS);
    println!("{:#?}", libc::_PC_LINK_MAX);
    println!("{:#?}", libc::_PC_MAX_CANON);
    println!("{:#?}", MTest::OptA as u32);
    println!("{:#?}", MTest::OptB as u32);
    println!("{:#?}", MTest::OptC as u32);
}
