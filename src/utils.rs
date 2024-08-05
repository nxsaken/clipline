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

/// Short-circuits with [`None`] if the `condition` is true.
macro_rules! reject_if {
    ($condition:expr) => {
        if $condition {
            return None;
        }
    };
}

pub(crate) use {map, reject_if};
