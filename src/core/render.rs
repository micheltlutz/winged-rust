//! Rendering options and the [`Render`] trait.
//!
//! Ports `core/RenderOptions.swift` and the render half of `core/HTMLTag.swift`.
//!
//! `WINGED_RUST_SPEC.md` §2 claims Winged-Swift has a `protocol HTMLRenderable`. It does
//! not — the only protocol in that library is `Layout`. The real design is a class
//! hierarchy whose single overridable primitive is
//! `write(into:options:indentLevel:)`, and that is what [`Render::write_into`] ports.
//! Everything else is a provided method on top of it, so the whole tree renders into one
//! buffer with one allocation.

/// How a node tree is turned into markup.
///
/// A value passed into the render call, never global state — two threads can render the
/// same tree with different settings at the same time. Winged-Swift's deprecated
/// process-wide `HTMLTag.xhtmlSelfClosing` switch is deliberately not ported.
///
/// # Examples
/// ```
/// use winged_rust::core::RenderOptions;
/// let compact = RenderOptions::compact();
/// let pretty = RenderOptions::pretty();
/// let four_spaces = RenderOptions::pretty().with_indent("    ");
/// # let _ = (compact, pretty, four_spaces);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderOptions {
    /// When true, children go on their own lines and are indented.
    pub pretty: bool,
    /// One indentation level. Only used when `pretty` is true. Defaults to two spaces.
    pub indent: String,
    /// When true, void elements close with ` />` instead of `>`.
    pub xhtml_self_closing: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            pretty: false,
            indent: "  ".to_string(),
            xhtml_self_closing: false,
        }
    }
}

impl RenderOptions {
    /// Minified output on a single line. The default, and what you should ship.
    #[must_use]
    pub fn compact() -> Self {
        Self::default()
    }

    /// Indented, human-readable output.
    #[must_use]
    pub fn pretty() -> Self {
        Self {
            pretty: true,
            ..Self::default()
        }
    }

    /// Sets the indentation string.
    #[must_use]
    pub fn with_indent(mut self, indent: impl Into<String>) -> Self {
        self.indent = indent.into();
        self
    }

    /// Closes void elements with ` />` instead of `>`.
    #[must_use]
    pub fn with_xhtml_self_closing(mut self, yes: bool) -> Self {
        self.xhtml_self_closing = yes;
        self
    }

    /// Appends `depth` levels of indentation to a buffer.
    pub(crate) fn write_indent(&self, out: &mut String, depth: usize) {
        for _ in 0..depth {
            out.push_str(&self.indent);
        }
    }
}

/// Anything that can be written as HTML.
///
/// Implement [`write_into`](Render::write_into); the rest comes free.
pub trait Render {
    /// Writes this node and its subtree into an existing buffer.
    ///
    /// This is the primitive every other method here is built on. Writing into a shared
    /// buffer avoids allocating an intermediate string per node.
    fn write_into(&self, out: &mut String, options: &RenderOptions, depth: usize);

    /// Renders as a single line of markup.
    ///
    /// Note the asymmetry with [`crate::Document::render`], which defaults to *pretty*.
    /// That difference is in the Swift original and the golden fixtures depend on it.
    fn render(&self) -> String {
        self.render_with(&RenderOptions::compact())
    }

    /// Renders with indentation and line breaks.
    fn render_pretty(&self) -> String {
        self.render_with(&RenderOptions::pretty())
    }

    /// Renders with the given options.
    fn render_with(&self, options: &RenderOptions) -> String {
        let mut out = String::with_capacity(1024);
        self.write_into(&mut out, options, 0);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_is_the_default() {
        let options = RenderOptions::default();
        assert!(!options.pretty);
        assert_eq!(options.indent, "  ");
        assert!(!options.xhtml_self_closing);
    }

    /// Ports `RenderOptionsTests.testCustomIndent`.
    #[test]
    fn the_indent_string_is_configurable() {
        let options = RenderOptions::pretty().with_indent("    ");
        let mut out = String::new();
        options.write_indent(&mut out, 2);
        assert_eq!(out, "        ");
    }

    /// Ports `RenderOptionsTests.testValueSemantics`. Options are a value: changing a
    /// copy must not affect the original.
    #[test]
    fn options_have_value_semantics() {
        let base = RenderOptions::compact();
        let derived = base.clone().with_xhtml_self_closing(true);
        assert!(!base.xhtml_self_closing);
        assert!(derived.xhtml_self_closing);
    }
}
