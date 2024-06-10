//! ## Symmetry utilities
//!
//! Contains utilities for working with symmetries.

/// Selects an expression based on `VERT`.
macro_rules! vh {
    ($a:expr, $b:expr$(,)?) => {
        if !VERT {
            $a
        } else {
            $b
        }
    };
}

/// Selects an expression based on `SWAP`.
macro_rules! xy {
    ($a:expr, $b:expr$(,)?) => {
        if !SWAP {
            $a
        } else {
            $b
        }
    };
    ($a_b:expr) => {{
        let (a, b) = $a_b;
        if !SWAP {
            a
        } else {
            b
        }
    }};
}

/// Selects an expression based on `FLIP`.
macro_rules! f {
    ($pos:expr, $neg:expr$(,)?) => {
        if !FLIP {
            $pos
        } else {
            $neg
        }
    };
}

/// Selects an expression based on `FX`.
macro_rules! fx {
    ($pos:expr, $neg:expr$(,)?) => {
        if !FX {
            $pos
        } else {
            $neg
        }
    };
}

/// Selects an expression based on `FY`.
macro_rules! fy {
    ($pos:expr, $neg:expr$(,)?) => {
        if !FY {
            $pos
        } else {
            $neg
        }
    };
}

/// Ensures `v1` and `v2` are sorted based on `$f`,
/// otherwise short-circuits with `$sc`.
macro_rules! sorted {
    ($f:ident, $v1:ident, $v2:ident, $sc:expr) => {{
        match $f {
            false if $v1 < $v2 => ($v1, $v2),
            true if $v2 < $v1 => ($v2, $v1),
            _ => return $sc,
        }
    }};
}

pub(crate) use {f, fx, fy, sorted, vh, xy};
