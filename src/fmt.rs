#[cfg(all(feature = "color", not(target_os = "windows")))]
use ansi_term::ANSIString;

#[cfg(all(feature = "color", not(target_os = "windows")))]
use ansi_term::Colour::{Green, Red, Yellow};

#[cfg(feature = "color")]
use libc;
use std::fmt;

#[cfg(all(feature = "color", not(target_os = "windows")))]
const STDERR: i32 = libc::STDERR_FILENO;
#[cfg(all(feature = "color", not(target_os = "windows")))]
const STDOUT: i32 = libc::STDOUT_FILENO;

#[cfg(target_os = "windows")]
const STDERR: i32 = 0;
#[cfg(target_os = "windows")]
const STDOUT: i32 = 0;

#[doc(hidden)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ColorWhen {
    Auto,
    Always,
    Never,
}

#[cfg(feature = "color")]
pub fn is_a_tty(stderr: bool) -> bool {
    debugln!("fn=is_a_tty;");
    debugln!("Use stderr...{:?}", stderr);
    let fd = if stderr { STDERR } else { STDOUT };
    unsafe { libc::isatty(fd) != 0 }
}

#[cfg(not(feature = "color"))]
pub fn is_a_tty(_: bool) -> bool {
    debugln!("fn=is_a_tty;");
    false
}

#[doc(hidden)]
pub struct Colorizer {
    pub use_stderr: bool,
    pub when: ColorWhen,
}

macro_rules! color {
    ($_self:ident, $c:ident, $m:expr) => {
        match $_self.when {
            ColorWhen::Auto => if is_a_tty($_self.use_stderr) {
                Format::$c($m)
            } else {
                Format::None($m)
            },
            ColorWhen::Always => Format::$c($m),
            ColorWhen::Never => Format::None($m),
        }
    };
}

impl Colorizer {
    pub fn good<T>(&self, msg: T) -> Format<T>
        where T: fmt::Display + AsRef<str>
    {
        debugln!("fn=good;");
        color!(self, Good, msg)
    }

    pub fn warning<T>(&self, msg: T) -> Format<T>
        where T: fmt::Display + AsRef<str>
    {
        debugln!("fn=warning;");
        color!(self, Warning, msg)
    }

    pub fn error<T>(&self, msg: T) -> Format<T>
        where T: fmt::Display + AsRef<str>
    {
        debugln!("fn=error;");
        color!(self, Error, msg)
    }

    pub fn none<T>(&self, msg: T) -> Format<T>
        where T: fmt::Display + AsRef<str>
    {
        debugln!("fn=none;");
        Format::None(msg)
    }
}

impl Default for Colorizer {
    fn default() -> Self {
        Colorizer {
            use_stderr: true,
            when: ColorWhen::Auto,
        }
    }
}

/// Defines styles for different types of error messages. Defaults to Error=Red, Warning=Yellow,
/// and Good=Green
#[derive(Debug)]
#[doc(hidden)]
pub enum Format<T> {
    /// Defines the style used for errors, defaults to Red
    Error(T),
    /// Defines the style used for warnings, defaults to Yellow
    Warning(T),
    /// Defines the style used for good values, defaults to Green
    Good(T),
    /// Defines no formatting style
    None(T),
}

#[cfg(all(feature = "color", not(target_os = "windows")))]
impl<T: AsRef<str>> Format<T> {
    fn format(&self) -> ANSIString {
        match *self {
            Format::Error(ref e) => Red.bold().paint(e.as_ref()),
            Format::Warning(ref e) => Yellow.paint(e.as_ref()),
            Format::Good(ref e) => Green.paint(e.as_ref()),
            Format::None(ref e) => ANSIString::from(e.as_ref()),
        }
    }
}

#[cfg(any(not(feature = "color"), target_os = "windows"))]
#[cfg_attr(feature="lints", allow(match_same_arms))]
impl<T: fmt::Display> Format<T> {
    fn format(&self) -> &T {
        match *self {
            Format::Error(ref e) => e,
            Format::Warning(ref e) => e,
            Format::Good(ref e) => e,
            Format::None(ref e) => e,
        }
    }
}


#[cfg(all(feature = "color", not(target_os = "windows")))]
impl<T: AsRef<str>> fmt::Display for Format<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", &self.format()) }
}

#[cfg(any(not(feature = "color"), target_os = "windows"))]
impl<T: fmt::Display> fmt::Display for Format<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", &self.format()) }
}

#[cfg(all(test, feature = "color", not(target_os = "windows")))]
mod test {
    use ansi_term::ANSIString;
    use ansi_term::Colour::{Green, Red, Yellow};
    use super::Format;

    #[test]
    fn colored_output() {
        let err = Format::Error("error");
        assert_eq!(&*format!("{}", err),
                   &*format!("{}", Red.bold().paint("error")));
        let good = Format::Good("good");
        assert_eq!(&*format!("{}", good), &*format!("{}", Green.paint("good")));
        let warn = Format::Warning("warn");
        assert_eq!(&*format!("{}", warn), &*format!("{}", Yellow.paint("warn")));
        let none = Format::None("none");
        assert_eq!(&*format!("{}", none),
                   &*format!("{}", ANSIString::from("none")));
    }
}
