#[macro_export]
macro_rules! s {
    ($lit:expr) => {
        String::from($lit)
    };
}
