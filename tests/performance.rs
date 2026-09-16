//! A cheap render-performance guard that is safe to gate CI on.
//!
//! Ports the instinct behind
//! `Winged-Swift/Tests/WingedSwiftTests/RenderPerformanceTests.swift`, which asserts on
//! **output size** rather than elapsed time. A size assertion is deterministic across
//! machines and CI runners; a timing assertion is not, which is why the criterion suite in
//! `benches/render.rs` is not part of `cargo test`.
//!
//! What this catches: an accidental change that starts emitting extra whitespace, repeats
//! an attribute, or silently drops children — all of which move the byte count long before
//! anyone notices them in a diff.

use winged_rust::prelude::*;

fn wide_tree(count: usize) -> Element {
    ul().children_from((0..count).map(|i| li().add_class("item").text(format!("Item {i}"))))
}

#[test]
fn compact_output_size_is_stable() {
    let rendered = wide_tree(1_000).render();

    // <ul> + </ul> = 9 bytes; each item is `<li class="item">Item N</li>`.
    let expected: usize = 9
        + (0..1_000)
            .map(|i| format!(r#"<li class="item">Item {i}</li>"#).len())
            .sum::<usize>();

    assert_eq!(
        rendered.len(),
        expected,
        "compact render emitted unexpected bytes"
    );
}

#[test]
fn pretty_output_adds_exactly_one_line_per_node() {
    let rendered = wide_tree(1_000).render_pretty();
    // One line for `<ul>`, one per item, one for `</ul>`.
    assert_eq!(rendered.lines().count(), 1_002);
}

#[test]
fn rendering_is_stable_across_repeated_calls() {
    let tree = wide_tree(500);
    let first = tree.render();
    for _ in 0..10 {
        assert_eq!(tree.render(), first, "render is not deterministic");
    }
}

/// The renderer is recursive, so nesting depth is bounded by the stack.
///
/// 256 is the depth this guarantees, chosen because it is already deeper than browsers
/// themselves handle — Chrome and Firefox both flatten nesting beyond roughly 512 elements,
/// so markup this deep does not render as written in a browser either.
///
/// Deeper trees work in practice (1,000 levels renders fine on a main thread's 8 MiB
/// stack) but not on the 2 MiB stack the test harness gives a spawned thread, which is why
/// the guaranteed figure is conservative. Making the renderer iterative is tracked
/// separately.
#[test]
fn nesting_to_the_documented_depth_renders() {
    let mut node = p().text("bottom");
    for _ in 0..256 {
        node = div().child(node);
    }
    assert!(node.render().len() > 256);
    assert!(node.render_pretty().lines().count() > 256);
}
