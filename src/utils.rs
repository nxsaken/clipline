//! ## Utilities
//!
//! Common utilities.

/// Maps over an [`Option`].
macro_rules! map {
    ($option:expr, $some:pat => $mapped:expr$(,)?) => {
        match $option {
            None => None,
            Some($some) => Some($mapped),
        }
    };
    ($option:expr, $func:expr$(,)?) => {
        match $option {
            None => None,
            Some(me) => Some($func(me)),
        }
    };
}

pub(crate) use map;
