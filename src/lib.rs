//! # smacro
//!
//! A collection of convenient Rust macros designed to reduce boilerplate
//! and improve code ergonomics for common operations.
//!
//! This crate provides simple, intuitive macros for creating common data
//! structures and performing frequent operations with minimal syntax.
//!
//! ## Quick Start
//!
//! ```rust
//! use smacro::{s, set, map};
//!
//! // String creation
//! let greeting = s!("Hello, {}!", "world");
//!
//! // HashSet creation
//! let numbers = set!(1, 2, 3, 4);
//!
//! // HashMap creation
//! let config = map![
//!     "debug" => "true",
//!     "port" => "8080",
//! ];
//! ```
//!
//! ## Available Macros
//!
//! - [`s!`] - Create `String` instances with various input types
//! - [`set!`] - Create `HashSet` instances with initial values
//! - [`map!`] - Create `HashMap` instances with key-value pairs
//!

// Re-export all macros
pub mod s;

#[cfg(feature = "map")]
pub mod map;

#[cfg(feature = "set")]
pub mod set;
