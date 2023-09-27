//! This crate simply adds a macro that makes the syntax simpler to end control
//! flow.
//!
//! The macro [or_do] will allow you to return a function early if a value is
//! None or Err.
//!
//! ```
//! use ordoo::or_do;
//!
//! let val: i32 = or_do!(Some(1), return);
//!
//! let val: i32 = or_do!(Ok::<_, std::io::Error>(1), _ => return);
//! ```
//!
//! I may add more QoL macros/functions later.

/// Unwrap an Option or Result or do something.
///
/// There is slightly different syntax for [Option]s and [Result]s. In options,
/// you only need to specify the action. With Results you also need to specify
/// the name of the error.
///
/// ```
/// use ordoo::or_do;
///
/// let val: i32 = or_do!(Some(1), return);
///
/// let val: i32 = or_do!(Ok::<_, std::io::Error>(1), _ => return);
/// ```
///
/// This simply expands to the following
///
/// ```
/// let val: i32 = match Some(1) {
///     Some(v) => v,
///     None => return,
/// };
///
/// let val: i32 = match Ok::<_, std::io::Error>(1) {
///     Ok(v) => v,
///     Err(_) => return,
/// };
/// ```
///
/// While this syntax is still a bit clunky, I find it much nicer than the
/// alternative.
#[macro_export]
macro_rules! or_do {
    ($v:expr, $do:expr) => {
        match $v {
            Some(v) => v,
            None => $do,
        }
    };
    ($v:expr, $err:ident => $do:expr) => {
        match $v {
            Ok(v) => v,
            Err($err) => $do,
        }
    };
    ($v:expr, _ => $do:expr) => {
        match $v {
            Ok(v) => v,
            Err(_) => $do,
        }
    };
}

#[cfg(test)]
mod tests {
    use std::io::{Error, ErrorKind};

    use super::*;

    #[test]
    fn ordo_opt() {
        let val = or_do!(Some(1), panic!("None"));
        assert_eq!(val, 1);

        or_do!(None, return);
        panic!("Should have been None")
    }

    #[test]
    fn ordo_res() {
        let val = or_do!(Ok::<_, Error>(1), e => panic!("{e}"));
        assert_eq!(val, 1);

        or_do!(Err(Error::new(ErrorKind::Other, "Oops")), _ => return);
        panic!("Should have been Err")
    }
}
