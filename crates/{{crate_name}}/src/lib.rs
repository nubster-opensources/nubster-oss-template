//! {{project-description}}

/// Returns a greeting string for the given name.
///
/// # Examples
///
/// ```
/// assert_eq!({{crate_name}}::greet("world"), "Hello, world!");
/// ```
#[must_use]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}
