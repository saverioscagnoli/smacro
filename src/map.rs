//! HashMap creation utilities.
//!
//!
//! The `map!` macro provides a convenient way to create `HashMap` instances
//! with initial key-value pairs, supporting any types that implement the
//! required traits for HashMap keys and values.

/// A convenience macro for creating `HashMap` instances with initial key-value pairs.
///
/// Requires the `map` feature to be enabled.
///
/// This macro provides two ways to create a `HashMap`:
/// - Create an empty map
/// - Create a map with initial key-value pairs
///
/// # Examples
///
/// ## Creating an empty map
/// ```
/// # use smacro::map;
/// use std::collections::HashMap;
///
/// let empty: HashMap<String, i32> = map![];
/// assert!(empty.is_empty());
/// ```
///
/// ## Creating a map with key-value pairs
/// ```
/// # use smacro::map;
/// let colors = map![
///     "red" => "#FF0000",
///     "green" => "#00FF00",
///     "blue" => "#0000FF"
/// ];
///
/// assert_eq!(colors.len(), 3);
/// assert_eq!(colors["red"], "#FF0000");
/// assert_eq!(colors["green"], "#00FF00");
/// assert_eq!(colors["blue"], "#0000FF");
/// ```
///
/// ## Trailing commas are supported
/// ```
/// # use smacro::map;
/// let scores = map![
///     "Alice" => 95,
///     "Bob" => 87,
///     "Charlie" => 92,
/// ];
///
/// assert_eq!(scores.len(), 3);
/// assert_eq!(scores["Alice"], 95);
/// ```
///
/// ## Works with any types that implement the required traits
/// ```
/// # use smacro::map;
/// let coordinates = map![
///     1 => (10, 20),
///     2 => (30, 40),
///     3 => (50, 60)
/// ];
///
/// assert_eq!(coordinates[&1], (10, 20));
/// assert_eq!(coordinates[&2], (30, 40));
/// ```
///
/// ## Using expressions as keys and values
/// ```
/// # use smacro::map;
/// let x = 5;
/// let calculations = map![
///     x => x * 2,
///     x + 1 => x * 3,
///     x + 2 => x * 4
/// ];
///
/// assert_eq!(calculations[&5], 10);
/// assert_eq!(calculations[&6], 15);
/// assert_eq!(calculations[&7], 20);
/// ```
///
/// # Duplicate Keys
///
/// If duplicate keys are provided, the last value wins (same behavior as `HashMap::insert`):
/// ```
/// # use smacro::map;
/// let overrides = map![
///     "key" => "first",
///     "key" => "second"
/// ];
///
/// assert_eq!(overrides["key"], "second");
/// ```
///
/// # Type Inference
///
/// When creating an empty map, you may need to specify the types explicitly:
/// ```
/// # use smacro::map;
/// use std::collections::HashMap;
///
/// let empty: HashMap<String, i32> = map![];
/// // or
/// let empty = map![] as HashMap<String, i32>;
/// ```
#[macro_export]
macro_rules! map {
    [] => {
        std::collections::HashMap::new()
    };
    [$($key:expr => $value:expr),+ $(,)?] => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )+
            map
        }
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn empty_map() {
        let m: HashMap<String, i32> = map![];
        assert!(m.is_empty());
    }

    #[test]
    fn map_with_string_keys() {
        let m = map![
            "hello" => "world",
            "foo" => "bar"
        ];
        assert_eq!(m.len(), 2);
        assert_eq!(m["hello"], "world");
        assert_eq!(m["foo"], "bar");
    }

    #[test]
    fn map_with_trailing_comma() {
        let m = map![
            1 => "one",
            2 => "two",
            3 => "three",
        ];
        assert_eq!(m.len(), 3);
        assert_eq!(m[&1], "one");
        assert_eq!(m[&2], "two");
        assert_eq!(m[&3], "three");
    }

    #[test]
    fn map_with_expressions() {
        let x = 10;
        let m = map![
            x => x * 2,
            x + 1 => x * 3
        ];
        assert_eq!(m[&10], 20);
        assert_eq!(m[&11], 30);
    }

    #[test]
    fn map_with_duplicate_keys() {
        let m = map![
            "key" => "first",
            "key" => "second"
        ];
        assert_eq!(m.len(), 1);
        assert_eq!(m["key"], "second");
    }

    #[test]
    fn map_type_inference() {
        let _m1: HashMap<String, i32> = map![];
        let _m2 = map![] as HashMap<i32, String>;
    }

    #[test]
    fn map_with_mixed_types() {
        let m = map![
            1 => "one".to_string(),
            2 => "two".to_string(),
            3 => "three".to_string()
        ];
        assert_eq!(m.len(), 3);
        assert_eq!(m[&1], "one");
    }
}
