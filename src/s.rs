//! String creation utilities.
//!
//! The `s!` macro provides a convenient way to create `String` instances
//! with minimal boilerplate, supporting empty strings, direct conversion,
//! and format string syntax.

/// A convenience macro for creating `String` instances with various input types.
///
/// This macro provides three different ways to create a `String`:
/// - Create an empty string
/// - Convert any type implementing `ToString` to a `String`
/// - Use format string syntax with arguments
///
/// # Examples
///
/// ## Creating an empty string
/// ```
/// # use smacro::s;
/// let empty = s!();
/// assert_eq!(empty, String::new());
/// ```
///
/// ## Converting a value to string
/// ```
/// # use smacro::s;
/// let hello = s!("Hello, world!");
/// let number = s!(42);
/// let boolean = s!(true);
///
/// assert_eq!(hello, "Hello, world!".to_string());
/// assert_eq!(number, "42".to_string());
/// assert_eq!(boolean, "true".to_string());
/// ```
///
/// ## Format string with arguments
/// ```
/// # use smacro::s;
/// let name = "Alice";
/// let age = 30;
/// let greeting = s!("Hello, {}! You are {} years old.", name, age);
///
/// assert_eq!(greeting, "Hello, Alice! You are 30 years old.".to_string());
/// ```
///
/// ## More complex formatting
/// ```
/// # use smacro::s;
/// let x = 3.14159;
/// let formatted = s!("Pi is approximately {:.2}", x);
///
/// assert_eq!(formatted, "Pi is approximately 3.14".to_string());
/// ```
///
/// # Performance Note
///
/// This macro is a thin wrapper around standard Rust string creation methods:
/// - `s!()` calls `String::new()`
/// - `s!(expr)` calls `expr.to_string()`
/// - `s!(format_str, args...)` calls `format!(format_str, args...)`
///
/// There is no additional overhead compared to calling these methods directly.
#[macro_export]
macro_rules! s {
    () => {
        String::new()
    };
    ($e:expr) => {
        $e.to_string()
    };
    ($($arg:tt)*) => {
        format!($($arg)*)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_empty_string() {
        let empty = s!();
        assert_eq!(empty, String::new());
    }

    #[test]
    fn test_string_literal() {
        let hello = s!("Hello, world!");
        assert_eq!(hello, "Hello, world!".to_string());
    }

    #[test]
    fn test_format_string() {
        let name = "Alice";
        let greeting = s!("Hello, {}!", name);
        assert_eq!(greeting, "Hello, Alice!".to_string());
    }

    #[test]
    fn test_multiple_arguments() {
        let a = 5;
        let b = 10;
        let result = s!("The sum of {} and {} is {}.", a, b, a + b);
        assert_eq!(result, "The sum of 5 and 10 is 15.".to_string());
    }

    #[test]
    fn test_integer_conversion() {
        let num = s!(42);
        assert_eq!(num, "42".to_string());
    }

    #[test]
    fn test_boolean_conversion() {
        let bool_str = s!(true);
        assert_eq!(bool_str, "true".to_string());
    }

    #[test]
    fn test_float_formatting() {
        let pi = 3.14159;
        let formatted = s!("Pi: {:.2}", pi);
        assert_eq!(formatted, "Pi: 3.14".to_string());
    }
}
