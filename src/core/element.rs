//! The [`Element`] type and its fluent builder API.
//!
//! Ports the state half of `core/HTMLTag.swift`, all of `core/CSSHelpers.swift` and all of
//! `core/AttributeHelpers.swift`. Those three files are the *complete* chainable surface
//! of Winged-Swift — there is nothing else.

use crate::core::attribute::Attribute;
use crate::core::escape::{write_escaped_attribute, write_escaped_text};
use crate::core::node::Node;
use crate::core::render::{Render, RenderOptions};

/// An HTML element: a tag name, attributes, optional text content, and children.
///
/// Every builder method takes `self` and returns `Self`, so they chain. Content and
/// attribute values are escaped as they go in, not when the element is rendered.
///
/// # Examples
/// ```
/// use winged_rust::prelude::*;
/// let card = div()
///     .add_class("card")
///     .set_id("hero")
///     .child(h1().text("Welcome"))
///     .child(p().text("Fuel, tyres & chain."));
/// assert_eq!(
///     card.render(),
///     r#"<div class="card" id="hero"><h1>Welcome</h1><p>Fuel, tyres &amp; chain.</p></div>"#
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    tag: String,
    attributes: Vec<Attribute>,
    content: Option<String>,
    children: Vec<Node>,
}

impl Element {
    /// Creates an element with the given tag name.
    pub fn new(tag: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            attributes: Vec::new(),
            content: None,
            children: Vec::new(),
        }
    }

    // MARK: - Accessors

    /// The tag name.
    #[must_use]
    pub fn tag(&self) -> &str {
        &self.tag
    }

    /// The attributes, in insertion order.
    #[must_use]
    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }

    /// The escaped text content, if any.
    #[must_use]
    pub fn content(&self) -> Option<&str> {
        self.content.as_deref()
    }

    /// The child nodes.
    #[must_use]
    pub fn children(&self) -> &[Node] {
        &self.children
    }

    // MARK: - Content

    /// Sets the text content, escaping it.
    ///
    /// Replaces any content already set.
    #[must_use]
    pub fn text(mut self, content: impl AsRef<str>) -> Self {
        let raw = content.as_ref();
        let mut escaped = String::with_capacity(raw.len());
        write_escaped_text(&mut escaped, raw, false);
        self.content = Some(escaped);
        self
    }

    /// Sets the text content **without** escaping it.
    ///
    /// The documented escape hatch for markup you already trust. `<script>` and `<style>`
    /// use this by default.
    #[must_use]
    pub fn raw_text(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Appends a child node.
    #[must_use]
    pub fn child(mut self, child: impl Into<Node>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends several child nodes.
    #[must_use]
    pub fn children_from<N: Into<Node>>(mut self, children: impl IntoIterator<Item = N>) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    // MARK: - Attributes

    /// Appends an attribute. Does not deduplicate.
    #[must_use]
    pub fn add_attribute(mut self, attribute: Attribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    /// Appends `key="value"`, escaping the value. Does not deduplicate.
    ///
    /// Ports `setAttribute(key:value:)`.
    #[must_use]
    pub fn attr(self, key: impl Into<String>, value: impl AsRef<str>) -> Self {
        self.add_attribute(Attribute::new(key, value))
    }

    /// Appends a boolean attribute, which renders as a bare key.
    #[must_use]
    pub fn bool_attr(self, key: impl Into<String>) -> Self {
        self.add_attribute(Attribute::boolean(key))
    }

    /// Appends `data-{key}="{value}"`.
    #[must_use]
    pub fn data_attr(self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.attr(format!("data-{}", key.as_ref()), value)
    }

    /// Appends several `data-*` attributes.
    ///
    /// Takes an ordered iterator rather than a map. Winged-Swift's `dataAttributes(_:)`
    /// takes a Swift `Dictionary`, so its rendered attribute order is nondeterministic
    /// between runs; this signature makes the output stable. See `PORTING.md`.
    #[must_use]
    pub fn data_attrs<K: AsRef<str>, V: AsRef<str>>(
        mut self,
        data: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        for (key, value) in data {
            self = self.data_attr(key, value);
        }
        self
    }

    /// Appends `aria-{key}="{value}"`.
    #[must_use]
    pub fn aria_attr(self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.attr(format!("aria-{}", key.as_ref()), value)
    }

    /// Appends several `aria-*` attributes, in the order given.
    #[must_use]
    pub fn aria_attrs<K: AsRef<str>, V: AsRef<str>>(
        mut self,
        aria: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        for (key, value) in aria {
            self = self.aria_attr(key, value);
        }
        self
    }

    // MARK: - Replacing helpers
    //
    // `set_id`, `set_style` and `set_role` remove any existing value before appending.
    // Every other helper appends without deduplication — that asymmetry is Winged-Swift's.

    /// Sets `id`, replacing any existing one.
    #[must_use]
    pub fn set_id(self, id: impl AsRef<str>) -> Self {
        self.replace_attribute("id", id.as_ref())
    }

    /// Sets `style`, replacing any existing one.
    #[must_use]
    pub fn set_style(self, style: impl AsRef<str>) -> Self {
        self.replace_attribute("style", style.as_ref())
    }

    /// Sets `role`, replacing any existing one.
    #[must_use]
    pub fn set_role(self, role: impl AsRef<str>) -> Self {
        self.replace_attribute("role", role.as_ref())
    }

    fn replace_attribute(mut self, key: &str, value: &str) -> Self {
        self.attributes.retain(|a| a.key() != key);
        self.attributes.push(Attribute::new(key, value));
        self
    }

    // MARK: - Classes

    /// Appends a class name to `class`, space-joined.
    ///
    /// Only the new value is escaped — the existing attribute is already escaped, and
    /// re-escaping it would double-encode. `Tests/WingedSwiftTests/CSSHelpersTests.swift`
    /// has a regression test for exactly that.
    #[must_use]
    pub fn add_class(mut self, class_name: impl AsRef<str>) -> Self {
        let raw = class_name.as_ref();
        if let Some(existing) = self.attributes.iter_mut().find(|a| a.key() == "class") {
            let mut merged = existing.value().to_string();
            merged.push(' ');
            write_escaped_attribute(&mut merged, raw);
            // `Attribute::raw` because `merged` is already escaped.
            *existing = Attribute::raw("class", merged);
        } else {
            self.attributes.push(Attribute::new("class", raw));
        }
        self
    }

    /// Appends several class names.
    #[must_use]
    pub fn add_classes<S: AsRef<str>>(mut self, class_names: impl IntoIterator<Item = S>) -> Self {
        for name in class_names {
            self = self.add_class(name);
        }
        self
    }
}

impl Render for Element {
    fn write_into(&self, out: &mut String, options: &RenderOptions, depth: usize) {
        crate::core::node::write_element_tree(self, out, options, depth);
    }
}

/// Tears the subtree down iteratively.
///
/// The derived drop glue recurses once per nesting level, so a tree deep enough to need
/// the iterative writer would abort while being *freed* instead — after rendering fine.
/// Draining into an explicit worklist keeps teardown flat.
///
/// Each node has its children moved out before it goes out of scope, so the `Drop` that
/// runs for it finds nothing left to recurse into.
impl Drop for Element {
    fn drop(&mut self) {
        let mut pending = core::mem::take(&mut self.children);
        while let Some(node) = pending.pop() {
            match node {
                Node::Element(mut element) => pending.append(&mut element.children),
                Node::Fragment(mut children) => pending.append(&mut children),
                Node::Text(_) | Node::Raw(_) | Node::Comment(_) => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::{button, div, p, span};

    /// Ports `CSSHelpersTests.testAddClassAppends`.
    #[test]
    fn add_class_appends_to_one_attribute() {
        assert_eq!(
            div().add_class("a").add_class("b").render(),
            r#"<div class="a b"></div>"#
        );
    }

    /// Ports `CSSHelpersTests.testAddClassDoesNotDoubleEscape`.
    #[test]
    fn chaining_add_class_does_not_double_escape() {
        let rendered = div().add_class("a&b").add_class("c").render();
        assert_eq!(rendered, r#"<div class="a&amp;b c"></div>"#);
        assert!(!rendered.contains("&amp;amp;"));
    }

    /// Ports `CSSHelpersTests.testAddClassCannotInjectAnAttribute`.
    #[test]
    fn a_quote_in_a_class_name_cannot_break_out() {
        let rendered = div().add_class(r#"a" onload="alert(1)"#).render();
        assert!(!rendered.contains("onload=\""));
        assert!(rendered.contains("&quot;"));
    }

    #[test]
    fn add_classes_appends_all_of_them_in_order() {
        assert_eq!(
            div().add_classes(["card", "p-4", "shadow"]).render(),
            r#"<div class="card p-4 shadow"></div>"#
        );
    }

    /// Ports `CSSHelpersTests.testSetIdReplaces`.
    #[test]
    fn set_id_replaces_rather_than_appending() {
        let rendered = div().set_id("first").set_id("second").render();
        assert_eq!(rendered, r#"<div id="second"></div>"#);
    }

    #[test]
    fn set_style_and_set_role_also_replace() {
        let rendered = div()
            .set_style("color:red")
            .set_style("color:blue")
            .set_role("main")
            .render();
        assert_eq!(rendered, r#"<div style="color:blue" role="main"></div>"#);
    }

    /// Ports `AttributeHelpersTests.testDataAttribute` and `testAriaAttribute`.
    #[test]
    fn data_and_aria_attributes_get_their_prefixes() {
        let rendered = span()
            .data_attr("id", "7")
            .aria_attr("label", "Close")
            .render();
        assert_eq!(rendered, r#"<span data-id="7" aria-label="Close"></span>"#);
    }

    /// The improvement over Swift: ordered input means stable output.
    #[test]
    fn bulk_attribute_order_is_stable() {
        let build = || {
            div()
                .data_attrs([("a", "1"), ("b", "2"), ("c", "3")])
                .render()
        };
        let expected = r#"<div data-a="1" data-b="2" data-c="3"></div>"#;
        for _ in 0..16 {
            assert_eq!(build(), expected);
        }
    }

    /// Ports `HTMLTagTests.testAttributesAreNotDeduplicated`.
    #[test]
    fn plain_attributes_are_appended_without_deduplication() {
        assert_eq!(
            div().attr("data-x", "1").attr("data-x", "2").render(),
            r#"<div data-x="1" data-x="2"></div>"#
        );
    }

    #[test]
    fn text_is_escaped_and_raw_text_is_not() {
        assert_eq!(p().text("<b>").render(), "<p>&lt;b&gt;</p>");
        assert_eq!(p().raw_text("<b>").render(), "<p><b></p>");
    }

    #[test]
    fn boolean_attributes_render_bare() {
        assert_eq!(
            button().bool_attr("disabled").render(),
            "<button disabled></button>"
        );
    }

    #[test]
    fn children_from_appends_a_sequence() {
        let list = div().children_from([p().text("a"), p().text("b")]);
        assert_eq!(list.render(), "<div><p>a</p><p>b</p></div>");
    }
}
