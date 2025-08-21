#[macro_export]
macro_rules! status {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).yellow().italic());
    };
}
