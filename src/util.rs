macro_rules! try_opt {
    ($opt:expr) => {
        match $opt {
            Some(v) => v,
            None => return None,
        }
    };
}

pub(crate) use try_opt;
