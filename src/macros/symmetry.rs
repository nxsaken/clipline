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

/// Calls `$fn_pos($arg...)` if `!YX && !FX` or `YX && !FY`,
/// or `$fn_neg($arg...)` if `!YX && FX` or `YX && FY`.
///
/// Optionally assigns to `$assign_x` and `$assign_y`.
macro_rules! yxf {
    (
        $(= $assign_x:expr, $assign_y:expr;)?
        $fn_pos:path, $fn_neg:path;
        ($($arg:expr),* $(,)?) $(;)?
    ) => {
        yx! {
            $($assign_x =)? fx!($fn_pos($($arg),*), $fn_neg($($arg),*)),
            $($assign_y =)? fy!($fn_pos($($arg),*), $fn_neg($($arg),*)),
        }
    };
}

/// Calls `$fn_pos($a, $b)` if `!YX && !FY` or `YX && !FX`,
/// or `$fn_neg($a, $b)` if `!YX && FY` or `YX && FX`.
///
/// Optionally assigns to `$assign_y` and `$assign_x`.
macro_rules! xyf {
    (
        $(= $assign_y:expr, $assign_x:expr;)?
        $fn_pos:path, $fn_neg:path;
        ($($arg:expr),* $(,)?) $(;)?
    ) => {
        yx! {
            $($assign_y =)? fy!($fn_pos($($arg),*), $fn_neg($($arg),*)),
            $($assign_x =)? fx!($fn_pos($($arg),*), $fn_neg($($arg),*)),
        }
    };
}

pub(crate) use {f, fx, fy, v, xyf, yx, yxf};
