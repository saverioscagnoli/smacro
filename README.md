# smacro

A collection of convenient Rust macros to reduce boilerplate and improve ergonomics.

By default, only macros with `no_std` support are enabled.
To enable more, just add the corresponding features.

## Macros

### `s!`

A versatile macro for creating `String` instances with minimal syntax.

```rust
use smacro::s;

// Empty string
let empty = s!();

// Convert anything with ToString
let hello = s!("Hello, world!");
let number = s!(42);

// Format strings
let name = "Alice";
let greeting = s!("Hello, {}!", name);
```

### `set!` - Requires `set` feature

Create `HashSet` instances with initial values.

```rust
use smacro::set;

// Empty set
let empty = set!();

// Set with values
let numbers = set!(1, 2, 3, 4);
let fruits = set!("apple", "banana", "orange");
```

### `map!` - Requires `map` feature

Create `HashMap` instances with key-value pairs.

```rust
use smacro::map;

// Empty map
let empty = map![];

// Map with initial data
let colors = map![
    "red" => "#FF0000",
    "green" => "#00FF00",
    "blue" => "#0000FF"
];
```

## License

MIT, see `LICENSE`
