//! HashSet creation utilities.
//!
//! Requires the `set` feature to be enabled.
//!
//! The `set!` macro provides a convenient way to create `HashSet` instances
//! with initial values, automatically handling deduplication and supporting
//! any type that implements `Hash + Eq`.

/// A convenience macro for creating `HashSet` instances with initial values.
///
/// Requires the `set` feature to be enabled.
///
/// This macro provides two ways to create a `HashSet`:
/// - Create an empty set
/// - Create a set with initial values
///
/// # Examples
///
/// ## Creating an empty set
/// ```
/// # use smacro::set;
/// use std::collections::HashSet;
///
/// let empty: HashSet<i32> = set!();
/// assert!(empty.is_empty());
/// ```
///
/// ## Creating a set with values
/// ```
/// # use smacro::set;
/// let numbers = set!(1, 2, 3, 4);
///
/// assert_eq!(numbers.len(), 4);
/// assert!(numbers.contains(&1));
/// assert!(numbers.contains(&2));
/// assert!(numbers.contains(&3));
/// assert!(numbers.contains(&4));
/// ```
///
/// ## Trailing commas are supported
/// ```
/// # use smacro::set;
/// let fruits = set!("apple", "banana", "orange",);
///
/// assert_eq!(fruits.len(), 3);
/// assert!(fruits.contains("apple"));
/// ```
///
/// ## Works with any type that implements `Hash + Eq`
/// ```
/// # use smacro::set;
/// #[derive(Hash, PartialEq, Eq)]
/// struct Point { x: i32, y: i32 }
///
/// let points = set!(
///     Point { x: 0, y: 0 },
///     Point { x: 1, y: 1 },
///     Point { x: 2, y: 2 }
/// );
///
/// assert_eq!(points.len(), 3);
/// ```
///
/// ## Duplicate values are automatically deduplicated
/// ```
/// # use smacro::set;
/// let numbers = set!(1, 2, 2, 3, 3, 3);
///
/// assert_eq!(numbers.len(), 3); // Only unique values remain
/// assert!(numbers.contains(&1));
/// assert!(numbers.contains(&2));
/// assert!(numbers.contains(&3));
/// ```
///
/// # Performance Note
///
/// This macro creates a new `HashSet` and inserts each element individually.
/// For large sets, consider using `HashSet::from_iter()` with an iterator
/// for potentially better performance.
///
/// # Type Inference
///
/// When creating an empty set, you may need to specify the type explicitly:
/// ```
/// # use smacro::set;
/// use std::collections::HashSet;
///
/// let empty: HashSet<String> = set!();
/// // or
/// let empty = set!() as HashSet<String>;
/// ```
#[macro_export]
macro_rules! set {
    () => {
        std::collections::HashSet::new()
    };

    ($($e:expr),+ $(,)?) => {
        {
            let mut set = std::collections::HashSet::new();
            $(
                set.insert($e);
            )+
            set
        }
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn empty_set() {
        let s: HashSet<i32> = set!();
        assert!(s.is_empty());
    }

    #[test]
    fn set_with_values() {
        let s = set!(1, 2, 3);
        assert_eq!(s.len(), 3);
        assert!(s.contains(&1));
        assert!(s.contains(&2));
        assert!(s.contains(&3));
    }

    #[test]
    fn set_with_trailing_comma() {
        let s = set!(10, 20, 30,);
        assert_eq!(s.len(), 3);
        assert!(s.contains(&10));
        assert!(s.contains(&20));
        assert!(s.contains(&30));
    }

    #[test]
    fn set_with_strings() {
        let s = set!("a", "b", "c");
        assert_eq!(s.len(), 3);
        assert!(s.contains("a"));
        assert!(s.contains("b"));
        assert!(s.contains("c"));
    }

    #[test]
    fn set_with_duplicates() {
        let s = set!(1, 2, 2, 3, 3, 3);
        assert_eq!(s.len(), 3);
        assert!(s.contains(&1));
        assert!(s.contains(&2));
        assert!(s.contains(&3));
    }

    #[test]
    fn set_with_mixed_expressions() {
        let x = 5;
        let s = set!(1 + 1, x, 3 * 2);
        assert_eq!(s.len(), 3);
        assert!(s.contains(&2));
        assert!(s.contains(&5));
        assert!(s.contains(&6));
    }

    #[test]
    fn set_type_inference() {
        let _s1: HashSet<i32> = set!();
        let _s2 = set!() as HashSet<String>;
    }
}
