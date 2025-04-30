/// Delegates `$call` to all `$Enum::$Variant($me)`.
macro_rules! variant {
    ($Enum:ident::{$($Variant:ident),* $(,)?}, $self:ident, $me:ident => $call:expr) => {
        match $self {
            $($Enum::$Variant($me) => $call,)*
        }
    };
}

/// An [`Option::map`] for `const` contexts.
macro_rules! map {
    ($opt:expr, |$some:ident| $body:expr $(,)?) => {
        match $opt {
            None => None,
            Some($some) => Some($body),
        }
    };
    ($opt:expr, $func:expr $(,)?) => {
        match $opt {
            None => None,
            Some(me) => Some($func(me)),
        }
    };
}

/// Short-circuits with [`None`] or `$ret` if `$cond` is `true`.
macro_rules! return_if {
    ($cond:expr) => {
        if $cond {
            return None;
        }
    };
    ($cond:expr, $ret:expr) => {
        if $cond {
            return $ret;
        }
    };
}

/// Unwraps an [`Option`], or short-circuits with [`None`] or `$ret`.
macro_rules! unwrap_or_return {
    ($opt:expr) => {{
        let Some(me) = $opt else {
            return None;
        };
        me
    }};
    ($opt:expr, $ret:expr) => {
        let Some(me) = $opt else {
            return $ret;
        };
        me
    };
}

pub(crate) use {map, return_if, unwrap_or_return, variant};
