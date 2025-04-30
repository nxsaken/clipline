/// Selects an expression based on `V`.
macro_rules! v {
    ($h:expr, $v:expr $(,)?) => {
        if !V {
            $h
        } else {
            $v
        }
    };
}

/// Selects an expression based on `YX`.
macro_rules! yx {
    ($x:expr, $y:expr $(,)?) => {
        if !YX {
            $x
        } else {
            $y
        }
    };
    ($x_y:expr) => {{
        let (x, y) = $x_y;
        if !YX {
            x
        } else {
            y
        }
    }};
}

/// Selects an expression based on `F`.
macro_rules! f {
    ($pos:expr, $neg:expr $(,)?) => {
        if !F {
            $pos
        } else {
            $neg
        }
    };
}

/// Selects an expression based on `FX`.
macro_rules! fx {
    ($pos:expr, $neg:expr $(,)?) => {
        if !FX {
            $pos
        } else {
            $neg
        }
    };
}

/// Selects an expression based on `FY`.
macro_rules! fy {
    ($pos:expr, $neg:expr $(,)?) => {
        if !FY {
            $pos
        } else {
            $neg
        }
    };
}

pub(crate) use {f, fx, fy, v, yx};
