//! Whole-page assembly.
//!
//! Ports `Winged-Swift/Sources/WingedSwift/core/Document.swift`.

use crate::core::{Element, Node, Render, RenderOptions};
use crate::elements::{body, head, html_tag};

/// A complete HTML document: a language, a `<head>` and a `<body>`.
///
/// The `Document` owns the doctype — [`Document::render`] prepends `<!DOCTYPE html>\n`,
/// while [`Document::root`] gives you the bare `<html>` element without it.
///
/// # Rendering default
///
/// **`Document::render` defaults to pretty, while [`Element::render`] defaults to
/// compact.** That asymmetry is in the Swift original and the golden fixtures depend on it,
/// so it is preserved rather than tidied up.
///
/// # Examples
/// ```
/// use winged_rust::prelude::*;
/// use winged_rust::Document;
///
/// let page = Document::new(Some("pt-BR"))
///     .head_children([title().text("RideKeeper")])
///     .body_children([h1().text("Track every service")]);
///
/// assert!(page.render().starts_with("<!DOCTYPE html>\n<html lang=\"pt-BR\">"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    /// The document language, rendered as `<html lang="…">`. Omitted when `None`.
    pub lang: Option<String>,
    /// The `<head>` element.
    pub head: Element,
    /// The `<body>` element.
    pub body: Element,
}

impl Document {
    /// Creates an empty document with the given language.
    #[must_use]
    pub fn new(lang: Option<&str>) -> Self {
        Self {
            lang: lang.map(ToString::to_string),
            head: head(),
            body: body(),
        }
    }

    /// Creates a document from an existing `<head>` and `<body>`.
    #[must_use]
    pub fn with_parts(lang: Option<&str>, head: Element, body: Element) -> Self {
        Self {
            lang: lang.map(ToString::to_string),
            head,
            body,
        }
    }

    /// Appends nodes to the `<head>`.
    #[must_use]
    pub fn head_children<N: Into<Node>>(mut self, children: impl IntoIterator<Item = N>) -> Self {
        self.head = self.head.children_from(children);
        self
    }

    /// Appends nodes to the `<body>`.
    #[must_use]
    pub fn body_children<N: Into<Node>>(mut self, children: impl IntoIterator<Item = N>) -> Self {
        self.body = self.body.children_from(children);
        self
    }

    /// The `<html>` element, **without** the doctype.
    ///
    /// Kept separate from [`render`](Self::render) because the static site generator and
    /// several tests want the tree rather than the serialized page.
    #[must_use]
    pub fn root(&self) -> Element {
        let root = match &self.lang {
            Some(lang) => html_tag().attr("lang", lang),
            None => html_tag(),
        };
        root.child(self.head.clone()).child(self.body.clone())
    }

    /// Renders the document with the given options, prefixed by the doctype.
    #[must_use]
    pub fn render_with(&self, options: &RenderOptions) -> String {
        let mut out = String::with_capacity(2048);
        out.push_str("<!DOCTYPE html>\n");
        self.root().write_into(&mut out, options, 0);
        out
    }

    /// Renders the document **pretty-printed** — the Swift default for a document.
    #[must_use]
    pub fn render(&self) -> String {
        self.render_with(&RenderOptions::pretty())
    }

    /// Renders the document on a single line.
    #[must_use]
    pub fn render_compact(&self) -> String {
        self.render_with(&RenderOptions::compact())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::{h1, title};

    /// Ports `DocumentTests.testDoctypeAndLang`.
    #[test]
    fn a_document_owns_the_doctype_and_the_lang_attribute() {
        let page = Document::new(Some("pt-BR"));
        assert!(
            page.render()
                .starts_with("<!DOCTYPE html>\n<html lang=\"pt-BR\">")
        );
    }

    /// Ports `DocumentTests.testOptionalLang`.
    #[test]
    fn the_lang_attribute_is_omitted_when_absent() {
        let page = Document::new(None);
        assert!(page.render().starts_with("<!DOCTYPE html>\n<html>"));
    }

    /// Ports `DocumentTests.testRootHasNoDoctype`.
    #[test]
    fn root_returns_the_tree_without_the_doctype() {
        let page = Document::new(Some("en"));
        assert!(!page.root().render().contains("DOCTYPE"));
        assert!(page.root().render().starts_with("<html lang=\"en\">"));
    }

    /// Ports `DocumentTests.testPrettyIsTheDefault`. The asymmetry with `Element::render`
    /// is deliberate — see the type docs.
    #[test]
    fn a_document_renders_pretty_by_default_unlike_an_element() {
        let page = Document::new(Some("en")).head_children([title().text("T")]);
        assert!(page.render().contains('\n'));
        assert!(
            !page
                .render_compact()
                .trim_start_matches("<!DOCTYPE html>\n")
                .contains('\n')
        );
    }

    #[test]
    fn head_and_body_children_land_in_the_right_place() {
        let page = Document::new(Some("en"))
            .head_children([title().text("T")])
            .body_children([h1().text("H")]);
        let rendered = page.render();
        let head_at = rendered.find("<title>").expect("title rendered");
        let body_at = rendered.find("<h1>").expect("h1 rendered");
        assert!(head_at < body_at);
    }

    #[test]
    fn a_document_is_a_value_and_can_be_shared_across_threads() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Document>();

        let page = Document::new(Some("en"));
        let copy = page.clone().head_children([title().text("changed")]);
        assert!(!page.render().contains("changed"));
        assert!(copy.render().contains("changed"));
    }
}
