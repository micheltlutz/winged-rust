//! Constructors for every HTML element Winged-Swift supports.
//!
//! Winged-Swift has one Swift file per tag — 93 of them — because Swift needs a class per
//! tag to hang a typed initializer on. Rust does not: all 93 reduce to a handful of shapes,
//! so they are generated from one table here. The public surface is the same; the source is
//! roughly 200 lines instead of 93 files.
//!
//! One tag is renamed back to its HTML spelling: Winged-Swift calls it `VarTag` because
//! `var` is a Swift keyword, which Rust's `var` is not. `<main>` keeps a suffix — see
//! [`main_tag`] for why.

use crate::core::Element;

/// Generates a zero-argument constructor per tag.
macro_rules! define_elements {
    ($($(#[$meta:meta])* $name:ident => $tag:literal),* $(,)?) => {
        $(
            $(#[$meta])*
            ///
            /// # Examples
            /// ```
            /// use winged_rust::prelude::*;
            #[doc = concat!("let el = ", stringify!($name), "();")]
            #[doc = concat!("assert_eq!(el.tag(), ", stringify!($tag), ");")]
            /// ```
            #[must_use]
            pub fn $name() -> Element {
                Element::new($tag)
            }
        )*

        /// Every tag this crate can build, as `(constructor name, tag name)`.
        ///
        /// Used by `scripts/generate-tag-catalog.sh` and by the catalog freshness test.
        pub const ALL_TAGS: &[(&str, &str)] = &[$((stringify!($name), $tag)),*];
    };
}

define_elements! {
    // MARK: - Inline semantics
    /// An abbreviation or acronym: `<abbr>`.
    abbr => "abbr",
    /// A cited creative work: `<cite>`.
    cite => "cite",
    /// Deleted text: `<del>`.
    del => "del",
    /// Stress emphasis: `<em>`.
    em => "em",
    /// Text in an alternate voice: `<i>`.
    i => "i",
    /// Inserted text: `<ins>`.
    ins => "ins",
    /// Keyboard input: `<kbd>`.
    kbd => "kbd",
    /// Marked or highlighted text: `<mark>`.
    mark => "mark",
    /// A short inline quotation: `<q>`.
    q => "q",
    /// Sample program output: `<samp>`.
    samp => "samp",
    /// Side comments and fine print: `<small>`.
    small => "small",
    /// A generic inline container: `<span>`.
    span => "span",
    /// Strong importance: `<strong>`.
    strong => "strong",
    /// Subscript: `<sub>`.
    sub => "sub",
    /// Superscript: `<sup>`.
    sup => "sup",
    /// A machine-readable date or time: `<time>`.
    time => "time",
    /// A variable name: `<var>`. Winged-Swift calls this `VarTag`.
    var => "var",
    /// A line-break opportunity: `<wbr>`.
    wbr => "wbr",

    // MARK: - Void and leaf elements
    /// A line break: `<br>`.
    br => "br",
    /// A thematic break: `<hr>`.
    hr => "hr",
    /// An image: `<img>`.
    img => "img",
    /// An external resource link: `<link>`.
    link => "link",
    /// The document base URL: `<base>`.
    base => "base",
    /// Document metadata: `<meta>`. See [`crate::seo`] for the typed constructors.
    meta => "meta",
    /// The document title: `<title>`.
    title => "title",
    /// A script: `<script>`. Content is **not** escaped by default — use
    /// [`Element::raw_text`](crate::core::Element::raw_text).
    script => "script",
    /// Embedded CSS: `<style>`. Content is **not** escaped by default.
    style => "style",
    /// External content: `<embed>`.
    embed => "embed",

    // MARK: - Headings
    /// A top-level heading: `<h1>`.
    h1 => "h1",
    /// A second-level heading: `<h2>`.
    h2 => "h2",
    /// A third-level heading: `<h3>`.
    h3 => "h3",
    /// A fourth-level heading: `<h4>`.
    h4 => "h4",
    /// A fifth-level heading: `<h5>`.
    h5 => "h5",
    /// A sixth-level heading: `<h6>`.
    h6 => "h6",

    // MARK: - Sectioning and flow
    /// Contact information: `<address>`.
    address => "address",
    /// A self-contained composition: `<article>`.
    article => "article",
    /// Tangential content: `<aside>`.
    aside => "aside",
    /// An extended quotation: `<blockquote>`.
    blockquote => "blockquote",
    /// The document body: `<body>`.
    body => "body",
    /// A drawing surface: `<canvas>`.
    canvas => "canvas",
    /// A dialog box: `<dialog>`.
    dialog => "dialog",
    /// A generic block container: `<div>`.
    div => "div",
    /// A figure caption: `<figcaption>`.
    figcaption => "figcaption",
    /// Self-contained content with an optional caption: `<figure>`.
    figure => "figure",
    /// A footer: `<footer>`.
    footer => "footer",
    /// Document metadata container: `<head>`.
    head => "head",
    /// Introductory content: `<header>`.
    header => "header",
    /// The dominant content of the body: `<main>`.
    ///
    /// Named `main_tag` rather than `main` because a binary crate's own `fn main` shadows a
    /// glob-imported `main()`, so `main().child(…)` fails to compile in exactly the place
    /// people write it first. Winged-Swift calls it `MainTag` for the analogous reason —
    /// `main` is a Swift keyword.
    main_tag => "main",
    /// Navigation links: `<nav>`.
    nav => "nav",
    /// Fallback for disabled scripting: `<noscript>`.
    noscript => "noscript",
    /// A paragraph: `<p>`.
    p => "p",
    /// A responsive image container: `<picture>`.
    picture => "picture",
    /// A thematic grouping: `<section>`.
    section => "section",
    /// The document root: `<html>`.
    html_tag => "html",

    // MARK: - Lists
    /// An unordered list: `<ul>`.
    ul => "ul",
    /// An ordered list: `<ol>`.
    ol => "ol",
    /// A list item: `<li>`.
    li => "li",
    /// A description list: `<dl>`.
    dl => "dl",
    /// A description term: `<dt>`.
    dt => "dt",
    /// A description detail: `<dd>`.
    dd => "dd",

    // MARK: - Interactive
    /// A hyperlink: `<a>`.
    a => "a",
    /// A button: `<button>`.
    button => "button",
    /// A disclosure widget: `<details>`.
    details => "details",
    /// A disclosure summary: `<summary>`.
    summary => "summary",

    // MARK: - Tables
    /// A table: `<table>`.
    table => "table",
    /// A table row: `<tr>`.
    tr => "tr",
    /// A table cell: `<td>`.
    td => "td",
    /// A table header cell: `<th>`.
    th => "th",
    /// A table caption: `<caption>`.
    caption => "caption",
    /// A column group: `<colgroup>`.
    colgroup => "colgroup",
    /// A table column: `<col>`.
    col => "col",
    /// The table body: `<tbody>`.
    tbody => "tbody",
    /// The table footer: `<tfoot>`.
    tfoot => "tfoot",
    /// The table header: `<thead>`.
    thead => "thead",

    // MARK: - Forms
    /// A form: `<form>`.
    form => "form",
    /// A form control: `<input>`.
    input => "input",
    /// A multi-line text control: `<textarea>`.
    textarea => "textarea",
    /// A caption for a form control: `<label>`.
    label => "label",
    /// A group of form controls: `<fieldset>`.
    fieldset => "fieldset",
    /// A caption for a fieldset: `<legend>`.
    legend => "legend",
    /// A drop-down list: `<select>`.
    select => "select",
    /// An option in a select: `<option>`.
    option => "option",
    /// A group of options: `<optgroup>`.
    optgroup => "optgroup",
    /// A list of predefined options: `<datalist>`.
    datalist => "datalist",
    /// The result of a calculation: `<output>`.
    output => "output",
    /// A scalar measurement within a range: `<meter>`.
    meter => "meter",
    /// Task completion progress: `<progress>`.
    progress => "progress",

    // MARK: - Media
    /// Embedded sound: `<audio>`.
    audio => "audio",
    /// Embedded video: `<video>`.
    video => "video",
    /// A media resource: `<source>`.
    source => "source",
    /// A timed text track: `<track>`.
    track => "track",
    /// A nested browsing context: `<iframe>`. Prefer [`iframe_titled`], which cannot be
    /// built without the `title` an assistive technology needs.
    iframe => "iframe",

    // MARK: - Code
    /// Inline code: `<code>`. Whitespace-sensitive.
    code => "code",
    /// Preformatted text: `<pre>`. Whitespace-sensitive.
    pre => "pre",
}

// MARK: - Typed constructors
//
// The shapes that carry required or defaulted arguments in Winged-Swift and do not fit the
// table above. Rust has no default arguments, so a defaulted parameter becomes either an
// `Option` or a separate constructor — never a silently different default.

/// A hyperlink with its `href`: `<a href="…">`.
///
/// # Examples
/// ```
/// use winged_rust::prelude::*;
/// assert_eq!(link_to("/pricing").text("Pricing").render(), r#"<a href="/pricing">Pricing</a>"#);
/// ```
#[must_use]
pub fn link_to(href: impl AsRef<str>) -> Element {
    a().attr("href", href)
}

/// An image with its `src` and `alt`: `<img src="…" alt="…">`.
///
/// `alt` is required rather than optional. An image without an alternative text is the
/// single most common accessibility defect in generated HTML, and Winged-Swift's own
/// `ROADMAP.md` asks for a lint that catches it. Requiring it here is cheaper than linting
/// for it later. Pass `""` deliberately for a decorative image.
#[must_use]
pub fn image(src: impl AsRef<str>, alt: impl AsRef<str>) -> Element {
    img().attr("src", src).attr("alt", alt)
}

/// A `<script src="…">`.
#[must_use]
pub fn script_src(src: impl AsRef<str>) -> Element {
    script().attr("src", src)
}

/// A stylesheet link: `<link href="…" rel="stylesheet">`.
#[must_use]
pub fn stylesheet(href: impl AsRef<str>) -> Element {
    link().attr("href", href).attr("rel", "stylesheet")
}

/// A `<button type="…">`. Winged-Swift defaults the type to `"button"`.
#[must_use]
pub fn button_typed(button_type: impl AsRef<str>) -> Element {
    button().attr("type", button_type)
}

/// An `<input type="…" name="…">`.
#[must_use]
pub fn input_named(input_type: impl AsRef<str>, name: impl AsRef<str>) -> Element {
    input().attr("type", input_type).attr("name", name)
}

/// A `<label for="…">`.
///
/// Named `for_id` because `for` is a Rust keyword; the rendered attribute is still `for`.
#[must_use]
pub fn label_for(for_id: impl AsRef<str>) -> Element {
    label().attr("for", for_id)
}

/// An `<iframe>` that cannot be built without a `title`.
///
/// Winged-Swift's `Iframe` makes `title:` a required initializer parameter for the same
/// reason: a frame with no title is unusable with a screen reader. `loading="lazy"` is
/// applied, matching the Swift default.
#[must_use]
pub fn iframe_titled(src: impl AsRef<str>, title_text: impl AsRef<str>) -> Element {
    iframe()
        .attr("src", src)
        .attr("title", title_text)
        .attr("loading", "lazy")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Render;

    #[test]
    fn the_catalog_covers_every_generated_tag() {
        assert!(
            ALL_TAGS.len() >= 90,
            "expected ~93 tags, found {}",
            ALL_TAGS.len()
        );
    }

    /// `Scripts/generate-tag-catalog.sh` in Winged-Swift uses an awk pattern that rejects
    /// digits, so `H1`–`H6` are silently missing from its checked-in catalog. Guard against
    /// the same gap here.
    #[test]
    fn the_catalog_includes_the_headings() {
        for heading in ["h1", "h2", "h3", "h4", "h5", "h6"] {
            assert!(
                ALL_TAGS.iter().any(|(_, tag)| *tag == heading),
                "{heading} missing from ALL_TAGS"
            );
        }
    }

    #[test]
    fn no_tag_is_listed_twice() {
        let mut tags: Vec<&str> = ALL_TAGS.iter().map(|(_, tag)| *tag).collect();
        tags.sort_unstable();
        let before = tags.len();
        tags.dedup();
        assert_eq!(before, tags.len(), "duplicate tag in ALL_TAGS");
    }

    /// `var` needs no suffix in Rust; `<main>` does, because `fn main` shadows it.
    #[test]
    fn the_renamed_tags_still_emit_their_html_names() {
        assert_eq!(main_tag().render(), "<main></main>");
        assert_eq!(var().render(), "<var></var>");
    }

    #[test]
    fn typed_constructors_set_their_attributes() {
        assert_eq!(link_to("/x").render(), r#"<a href="/x"></a>"#);
        assert_eq!(
            image("/a.png", "A cat").render(),
            r#"<img src="/a.png" alt="A cat">"#
        );
        assert_eq!(
            label_for("email").text("E-mail").render(),
            r#"<label for="email">E-mail</label>"#
        );
        assert_eq!(
            stylesheet("/s.css").render(),
            r#"<link href="/s.css" rel="stylesheet">"#
        );
    }

    #[test]
    fn an_iframe_built_through_the_typed_constructor_always_has_a_title() {
        let rendered = iframe_titled("/embed", "A map").render();
        assert!(rendered.contains(r#"title="A map""#));
        assert!(rendered.contains(r#"loading="lazy""#));
    }
}
