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

use enum_dict::EnumToDict;

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

fn main() {
    let _x = TestConfVar::PC_LINK_MAX;
    testconfvar_names();

    println!("{:#?}", libc::_PC_FILESIZEBITS);
    println!("{:#?}", libc::_PC_LINK_MAX);
    println!("{:#?}", libc::_PC_MAX_CANON);
}
