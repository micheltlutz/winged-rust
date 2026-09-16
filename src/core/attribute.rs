//! HTML attributes.
//!
//! Ports `Winged-Swift/Sources/WingedSwift/core/Attribute.swift`.

use crate::core::escape::write_escaped_attribute;

/// A single HTML attribute.
///
/// Values are escaped when the attribute is built, not when it is rendered — see
/// [`crate::core::escape`].
///
/// # Examples
/// ```
/// use winged_rust::core::Attribute;
/// assert_eq!(Attribute::new("href", "/a?x=1&y=2").to_string(), r#" href="/a?x=1&amp;y=2""#);
/// assert_eq!(Attribute::boolean("required").to_string(), " required");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    key: String,
    value: String,
    is_boolean: bool,
}

impl Attribute {
    /// Creates an attribute, escaping the value.
    pub fn new(key: impl Into<String>, value: impl AsRef<str>) -> Self {
        let raw = value.as_ref();
        let mut escaped = String::with_capacity(raw.len());
        write_escaped_attribute(&mut escaped, raw);
        Self {
            key: key.into(),
            value: escaped,
            is_boolean: false,
        }
    }

    /// Creates an attribute **without** escaping the value.
    ///
    /// Winged-Swift uses this for the keys it controls itself — `Meta`'s `name`,
    /// `property`, `charset` and `http-equiv` are all inserted with `escape: false`.
    /// Never pass user data here.
    pub fn raw(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            is_boolean: false,
        }
    }

    /// Creates a boolean attribute, which renders as a bare key.
    ///
    /// `hidden`, `checked`, `required`, `open`, `controls`, `autoplay`, `muted`.
    pub fn boolean(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: String::new(),
            is_boolean: true,
        }
    }

    /// The attribute name.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// The escaped attribute value. Empty for boolean attributes.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Whether this attribute renders as a bare key.
    #[must_use]
    pub fn is_boolean(&self) -> bool {
        self.is_boolean
    }

    /// Appends ` key="value"`, or ` key` for a boolean attribute, to a buffer.
    pub(crate) fn write_into(&self, out: &mut String) {
        out.push(' ');
        out.push_str(&self.key);
        if !self.is_boolean {
            out.push_str("=\"");
            out.push_str(&self.value);
            out.push('"');
        }
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        self.write_into(&mut out);
        f.write_str(&out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_is_escaped_at_construction() {
        let attr = Attribute::new("title", r#"a "quoted" & <tagged> value"#);
        assert_eq!(attr.value(), "a &quot;quoted&quot; &amp; <tagged> value");
    }

    /// Ports the boolean-attribute cases in `HTML14FeaturesTests`.
    #[test]
    fn boolean_attributes_render_as_a_bare_key() {
        assert_eq!(Attribute::boolean("required").to_string(), " required");
        assert!(Attribute::boolean("open").is_boolean());
    }

    #[test]
    fn raw_attributes_are_not_escaped() {
        assert_eq!(Attribute::raw("property", "og:title").value(), "og:title");
    }

    #[test]
    fn non_boolean_attributes_render_with_a_quoted_value() {
        assert_eq!(
            Attribute::new("class", "card p-4").to_string(),
            r#" class="card p-4""#
        );
    }

    #[test]
    fn a_quote_in_the_value_cannot_break_out_of_the_attribute() {
        let attr = Attribute::new("class", r#"a" onload="alert(1)"#);
        assert!(!attr.value().contains('"'));
    }
}
