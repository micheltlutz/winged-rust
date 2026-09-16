//! The node tree.
//!
//! Ports `core/HTMLTag.swift` (state), `core/Fragment.swift` and `core/RawHTML.swift`.
//!
//! Winged-Swift models fragments and raw markup as **subclasses** of `HTMLTag` that
//! override `write(into:)`. Rust models the same thing as a closed enum with a `match` in
//! the writer. That is strictly better here: the tree becomes `Send + Sync` for free,
//! which Winged-Swift's own `ROADMAP.md` lists as an unresolved 3.0 problem ("tag trees
//! are not `Sendable`"), and which is what makes the `parallel` feature sound.

use crate::core::attribute::Attribute;
use crate::core::element::Element;
use crate::core::escape::write_escaped_text;
use crate::core::render::{Render, RenderOptions};
use crate::core::tags::{is_void, is_whitespace_sensitive};

/// A node in the HTML tree.
///
/// # Examples
/// ```
/// use winged_rust::prelude::*;
/// let node = Node::from(div().text("hi"));
/// assert_eq!(node.render(), "<div>hi</div>");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    /// An element with a tag name, attributes and children.
    Element(Element),
    /// Escaped text. The escaping already happened — see [`crate::core::escape`].
    Text(String),
    /// Verbatim markup, never escaped. For imported snippets: an SVG, an embed code.
    ///
    /// Unlike [`Node::Fragment`], this is an opaque string, so pretty printing can only
    /// indent the whole blob. Reach for a fragment when you want the children indented.
    Raw(String),
    /// An HTML comment. **New in the Rust port** — Winged-Swift has no comment node.
    ///
    /// The content has `--` neutralised so the comment cannot be closed early.
    Comment(String),
    /// A transparent group that renders its children with no wrapper element.
    ///
    /// Use it to return several nodes from one expression — a `map` of cards, the body of
    /// an `if`, a group of `<meta>` tags — without introducing an extra `<div>`.
    Fragment(Vec<Node>),
}

impl Node {
    /// Creates a text node, escaping the content.
    #[must_use]
    pub fn text(content: impl AsRef<str>) -> Self {
        let raw = content.as_ref();
        let mut escaped = String::with_capacity(raw.len());
        write_escaped_text(&mut escaped, raw, false);
        Self::Text(escaped)
    }

    /// Creates a raw markup node. The content is **not** escaped.
    #[must_use]
    pub fn raw(content: impl Into<String>) -> Self {
        Self::Raw(content.into())
    }

    /// Creates a comment node.
    ///
    /// Any `--` in the content is replaced with `- -`, so the comment cannot terminate
    /// early and inject markup.
    ///
    /// # Examples
    /// ```
    /// use winged_rust::prelude::*;
    /// assert_eq!(Node::comment("a -- b").render(), "<!-- a - - b -->");
    /// ```
    #[must_use]
    pub fn comment(content: impl AsRef<str>) -> Self {
        Self::Comment(content.as_ref().replace("--", "- -"))
    }

    /// Creates a transparent group of nodes.
    #[must_use]
    pub fn fragment(children: impl IntoIterator<Item = Node>) -> Self {
        Self::Fragment(children.into_iter().collect())
    }

    /// Whether this node renders to nothing.
    ///
    /// The pretty-print writer uses this indirectly: it renders each child into a scratch
    /// buffer and skips it if the result is empty, which is what stops an empty fragment —
    /// the body of a false `if` — from leaving a blank line behind.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Element(_) | Self::Comment(_) => false,
            Self::Text(s) | Self::Raw(s) => s.is_empty(),
            Self::Fragment(children) => children.iter().all(Self::is_empty),
        }
    }
}

impl From<Element> for Node {
    fn from(element: Element) -> Self {
        Self::Element(element)
    }
}

impl From<&str> for Node {
    fn from(text: &str) -> Self {
        Self::text(text)
    }
}

impl From<String> for Node {
    fn from(text: String) -> Self {
        Self::text(text)
    }
}

impl Render for Node {
    fn write_into(&self, out: &mut String, options: &RenderOptions, depth: usize) {
        match self {
            // Escaped text and verbatim markup write the same way; they differ only in
            // when the escaping happened, which is at construction.
            Self::Text(s) | Self::Raw(s) => {
                if options.pretty {
                    options.write_indent(out, depth);
                }
                out.push_str(s);
            }
            Self::Comment(s) => {
                if options.pretty {
                    options.write_indent(out, depth);
                }
                out.push_str("<!-- ");
                out.push_str(s);
                out.push_str(" -->");
            }
            Self::Fragment(children) => write_fragment(children, out, options, depth),
            Self::Element(element) => write_element(element, out, options, depth),
        }
    }
}

/// Renders a fragment's children with no wrapper.
///
/// In pretty mode the children sit at the *parent's* depth and are joined with newlines,
/// with empty children skipped entirely.
fn write_fragment(children: &[Node], out: &mut String, options: &RenderOptions, depth: usize) {
    if !options.pretty {
        for child in children {
            child.write_into(out, options, depth);
        }
        return;
    }

    let mut first = true;
    for child in children {
        let mut rendered = String::new();
        child.write_into(&mut rendered, options, depth);
        if rendered.is_empty() {
            continue;
        }
        if !first {
            out.push('\n');
        }
        out.push_str(&rendered);
        first = false;
    }
}

fn write_element(element: &Element, out: &mut String, options: &RenderOptions, depth: usize) {
    if options.pretty {
        write_element_pretty(element, out, options, depth);
    } else {
        write_element_compact(element, out, options);
    }
}

fn write_attributes(attributes: &[Attribute], out: &mut String) {
    for attribute in attributes {
        attribute.write_into(out);
    }
}

/// Appends the closing token of a void element.
fn write_void_suffix(out: &mut String, options: &RenderOptions) {
    out.push_str(if options.xhtml_self_closing {
        " />"
    } else {
        ">"
    });
}

fn write_element_compact(element: &Element, out: &mut String, options: &RenderOptions) {
    out.push('<');
    out.push_str(element.tag());
    write_attributes(element.attributes(), out);

    // A void element takes no content and no children — both are silently dropped, which
    // is what Winged-Swift does.
    if is_void(element.tag()) {
        write_void_suffix(out, options);
        return;
    }

    out.push('>');
    if let Some(content) = element.content() {
        out.push_str(content);
    }
    for child in element.children() {
        child.write_into(out, options, 0);
    }
    out.push_str("</");
    out.push_str(element.tag());
    out.push('>');
}

fn write_element_pretty(
    element: &Element,
    out: &mut String,
    options: &RenderOptions,
    depth: usize,
) {
    // `<pre>`, `<code>` and `<textarea>` render every whitespace character they contain,
    // so indenting their children would change the text the browser displays.
    if is_whitespace_sensitive(element.tag()) {
        options.write_indent(out, depth);
        write_element_compact(element, out, options);
        return;
    }

    options.write_indent(out, depth);
    out.push('<');
    out.push_str(element.tag());
    write_attributes(element.attributes(), out);

    if is_void(element.tag()) {
        write_void_suffix(out, options);
        return;
    }

    out.push('>');

    if element.children().is_empty() {
        if let Some(content) = element.content() {
            out.push_str(content);
        }
        out.push_str("</");
        out.push_str(element.tag());
        out.push('>');
        return;
    }

    if let Some(content) = element.content() {
        out.push('\n');
        options.write_indent(out, depth + 1);
        out.push_str(content);
    }

    for child in element.children() {
        // Rendering into a scratch buffer keeps empty nodes — an empty fragment from a
        // false `if` — from leaving a blank line behind.
        let mut rendered = String::new();
        child.write_into(&mut rendered, options, depth + 1);
        if !rendered.is_empty() {
            out.push('\n');
            out.push_str(&rendered);
        }
    }

    out.push('\n');
    options.write_indent(out, depth);
    out.push_str("</");
    out.push_str(element.tag());
    out.push('>');
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::{code, div, img, p, pre, span};

    #[test]
    fn the_tree_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Node>();
        assert_send_sync::<Element>();
    }

    #[test]
    fn text_nodes_are_escaped() {
        assert_eq!(Node::text("a & b").render(), "a &amp; b");
    }

    /// Ports `HTMLEscapeTests.testRawHTMLIsNotEscaped`.
    #[test]
    fn raw_nodes_are_never_escaped() {
        assert_eq!(Node::raw("<b>bold</b>").render(), "<b>bold</b>");
        assert_eq!(Node::raw("<b>bold</b>").render_pretty(), "<b>bold</b>");
    }

    #[test]
    fn a_comment_cannot_close_itself_early() {
        let rendered = Node::comment("a --> b").render();
        assert_eq!(rendered.matches("-->").count(), 1);
        assert!(rendered.ends_with("-->"));
    }

    /// Ports `FragmentTests.testEmptyFragmentLeavesNoBlankLine`.
    #[test]
    fn an_empty_fragment_leaves_no_blank_line() {
        let tree = div()
            .child(p().text("a"))
            .child(Node::fragment([]))
            .child(p().text("b"));
        assert_eq!(
            tree.render_pretty(),
            "<div>\n  <p>a</p>\n  <p>b</p>\n</div>"
        );
    }

    #[test]
    fn a_fragment_renders_its_children_without_a_wrapper() {
        let tree = Node::fragment([p().text("a").into(), p().text("b").into()]);
        assert_eq!(tree.render(), "<p>a</p><p>b</p>");
        assert_eq!(tree.render_pretty(), "<p>a</p>\n<p>b</p>");
    }

    /// Ports the void-element cases in `TagCatalogTests`.
    #[test]
    fn void_elements_take_no_children() {
        let tree = img().attr("src", "/a.png").child(span().text("dropped"));
        assert_eq!(tree.render(), r#"<img src="/a.png">"#);
    }

    #[test]
    fn xhtml_mode_closes_void_elements_with_a_slash() {
        let options = RenderOptions::compact().with_xhtml_self_closing(true);
        assert_eq!(
            img().attr("src", "/a.png").render_with(&options),
            r#"<img src="/a.png" />"#
        );
    }

    /// Ports `WhitespaceTests`. Indenting inside `<pre>` would change what the browser shows.
    #[test]
    fn whitespace_sensitive_tags_are_not_indented_inside() {
        let tree = div().child(pre().child(code().text("let page = html { }")));
        assert_eq!(
            tree.render_pretty(),
            "<div>\n  <pre><code>let page = html { }</code></pre>\n</div>"
        );
    }

    /// Ports `PrettyPrintTests.testContentBeforeChildren`.
    #[test]
    fn content_goes_on_its_own_line_before_children() {
        let tree = div().text("lead").child(p().text("body"));
        assert_eq!(tree.render_pretty(), "<div>\n  lead\n  <p>body</p>\n</div>");
    }

    #[test]
    fn an_element_with_content_and_no_children_stays_on_one_line() {
        assert_eq!(p().text("hi").render_pretty(), "<p>hi</p>");
    }

    #[test]
    fn compact_is_the_default_for_an_element() {
        let tree = div().child(p().text("a"));
        assert_eq!(tree.render(), "<div><p>a</p></div>");
    }
}
