#[macro_export]
macro_rules! debug_print {
    ($debug:expr, $($arg:tt)*) => {
        if $debug {
            println!($($arg)*);
        }
    };
}
