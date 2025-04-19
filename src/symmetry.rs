//! ## Symmetry utilities

/// Selects an expression based on `V`.
macro_rules! vh {
    ($a:expr, $b:expr$(,)?) => {
        if !V {
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

/// Selects an expression based on `F`.
macro_rules! f {
    ($pos:expr, $neg:expr$(,)?) => {
        if !F {
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

pub(crate) use {f, fx, fy, vh, xy};
