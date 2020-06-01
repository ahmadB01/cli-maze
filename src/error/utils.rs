use crate::error::GameError;

use crossterm::ErrorKind;
use std::io::Error;
use std::num::ParseIntError;

// really useful basic macro lol
#[macro_export]
macro_rules! build_errs {
    (
        $Result:ident<Err=$Name:ident>:
        $(
            $ErrVar:ident$(($($e:ident: $import:ty),*))? => $st:stmt
        ),+
    ) => {
        use std::fmt;

        pub type $Result<T> = Result<T, $Name>;
        pub enum $Name {
            $($ErrVar$(($($import),*))?),+
        }

        impl fmt::Debug for $Name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let c = match self {
                    $(
                        $Name::$ErrVar$(($($e),*))? => { $st }
                    ),+
                };
                write!(f, "{}", c)
            }
        }
    };
}

impl From<ErrorKind> for GameError {
    fn from(e: crossterm::ErrorKind) -> Self {
        GameError::TerminalError(e)
    }
}

impl From<Error> for GameError {
    fn from(e: Error) -> Self {
        GameError::IoError(None, e)
    }
}

impl From<ParseIntError> for GameError {
    fn from(_: ParseIntError) -> Self {
        GameError::IncorrectInput
    }
}
