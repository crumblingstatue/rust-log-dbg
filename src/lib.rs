/// Just like std's dbg, except calls `log::log!`
#[macro_export]
macro_rules! ldbg {
    ($lvl:expr, $val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                ::log::log!($lvl, "{} = {:#?}",
                   ::core::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($lvl:expr, $($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}
