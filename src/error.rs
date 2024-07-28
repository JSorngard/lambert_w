use core::fmt;
use std::backtrace::Backtrace;
use std::error::Error;

/// The error returned by the Lambert W_0 functions when the input is less than -1/e.
#[derive(Debug)]
pub struct LambertW0Error(Backtrace);

impl LambertW0Error {
    pub(crate) fn new() -> Self {
        Self(Backtrace::capture())
    }

    /// Returns a [`Backtrace`] to where the error was created.
    ///
    /// This backtrace was captured with [`Backtrace::capture`],
    /// see it for more information about how to make this display information when printed.
    pub fn backtrace(&self) -> &Backtrace {
        &self.0
    }
}

impl fmt::Display for LambertW0Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "argument out of range")
    }
}

impl Error for LambertW0Error {}

/// The error returned by the Lambert W_-1 functions when the input is positive or less than -1/e.
#[derive(Debug)]
pub struct LambertWm1Error {
    backtrace: Backtrace,
    reason: LambertWm1ErrorReason,
}

/// The reason for the error in the Lambert W_-1 functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LambertWm1ErrorReason {
    ArgumentOutOfRange,
    PositiveArgument,
}

impl LambertWm1Error {
    /// Returns a [`Backtrace`] to where the error was created.
    ///
    /// This backtrace was captured with [`Backtrace::capture`],
    /// see it for more information about how to make this display information when printed.
    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }

    /// Returns the reason for the error.
    pub fn reason(&self) -> LambertWm1ErrorReason {
        self.reason
    }

    pub(crate) fn new(reason: LambertWm1ErrorReason) -> Self {
        Self {
            backtrace: Backtrace::capture(),
            reason,
        }
    }
}

impl fmt::Display for LambertWm1Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.reason {
            LambertWm1ErrorReason::ArgumentOutOfRange => write!(f, "argument out of range"),
            LambertWm1ErrorReason::PositiveArgument => write!(f, "positive argument"),
        }
    }
}

impl Error for LambertWm1Error {}
