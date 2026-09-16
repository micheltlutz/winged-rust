//! Byte-for-byte parity with Winged-Swift.
//!
//! The four files in `tests/fixtures/` are copied verbatim out of
//! `Winged-Swift/Tests/WingedSwiftTests/Fixtures/`. They are the strongest correctness gate
//! this crate has: if the Rust renderer reproduces them exactly, the port is proven rather
//! than asserted.
//!
//! `marketing-pretty.html` alone exercises the doctype, `lang`, seventeen `<meta>` tags,
//! nested navigation, a table with caption/thead/tbody, a boolean `open` attribute, a form
//! with fieldset/legend/label/input/button, whitespace-sensitive `<pre><code>`, a figure,
//! void elements, and `&` entity escaping in both text and attribute position.
//!
//! Run with `WINGED_UPDATE_FIXTURES=1 cargo test --test golden` to regenerate. A
//! regenerated fixture showing up in a pull-request diff is the signal that markup changed,
//! which is what makes markup changes reviewable instead of invisible.

use std::path::{Path, PathBuf};

use winged_rust::Document;
use winged_rust::prelude::*;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

/// Compares `actual` against the named fixture, or rewrites it under
/// `WINGED_UPDATE_FIXTURES=1`.
fn assert_matches_fixture(name: &str, actual: &str) {
    let path = fixtures_dir().join(name);

    if std::env::var_os("WINGED_UPDATE_FIXTURES").is_some() {
        std::fs::write(&path, actual).expect("fixture is writable");
        return;
    }

    let expected = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read fixture {}: {e}", path.display()));

    if expected != actual {
        panic!("{}", render_diff(name, &expected, actual));
    }
}

/// A line-oriented diff. `assertion failed` on a 2.7 KB string tells you nothing.
fn render_diff(name: &str, expected: &str, actual: &str) -> String {
    let mut report = format!("fixture {name} does not match\n");
    let expected_lines: Vec<&str> = expected.lines().collect();
    let actual_lines: Vec<&str> = actual.lines().collect();

    for i in 0..expected_lines.len().max(actual_lines.len()) {
        match (expected_lines.get(i), actual_lines.get(i)) {
            (Some(e), Some(a)) if e == a => {}
            (e, a) => {
                report.push_str(&format!(
                    "line {}:\n  expected: {:?}\n  actual:   {:?}\n",
                    i + 1,
                    e.unwrap_or(&"<missing>"),
                    a.unwrap_or(&"<missing>")
                ));
            }
        }
    }
    report.push_str("\nRegenerate with WINGED_UPDATE_FIXTURES=1 cargo test --test golden\n");
    report
}

/// The marketing page from `Winged-Swift/Tests/WingedSwiftTests/GoldenFileTests.swift`,
/// rebuilt with the Rust builder API.
fn marketing_page() -> Document {
    let description = "Motorcycle maintenance companion";
    let og_image = "https://ridekeeper.example/og.jpg";

    Document::new(Some("pt-BR"))
        .head_children([
            meta().attr("charset", "UTF-8"),
            meta()
                .attr("name", "viewport")
                .attr("content", "width=device-width, initial-scale=1.0"),
            meta()
                .attr("name", "description")
                .attr("content", description),
            meta()
                .attr("name", "robots")
                .attr("content", "index, follow"),
            meta()
                .attr("name", "keywords")
                .attr("content", "swift, motorcycle"),
            meta().attr("name", "author").attr("content", "Michel Lutz"),
            meta()
                .attr("property", "og:title")
                .attr("content", "RideKeeper"),
            meta()
                .attr("property", "og:description")
                .attr("content", description),
            meta()
                .attr("property", "og:image")
                .attr("content", og_image),
            meta()
                .attr("property", "og:url")
                .attr("content", "https://ridekeeper.example"),
            meta()
                .attr("property", "og:type")
                .attr("content", "website"),
            meta()
                .attr("name", "twitter:card")
                .attr("content", "summary_large_image"),
            meta()
                .attr("name", "twitter:title")
                .attr("content", "RideKeeper"),
            meta()
                .attr("name", "twitter:description")
                .attr("content", description),
            meta()
                .attr("name", "twitter:image")
                .attr("content", og_image),
            meta()
                .attr("name", "twitter:site")
                .attr("content", "@micheltlutz"),
            title().text("RideKeeper — track every service"),
            stylesheet("/css/style.css"),
        ])
        .body_children([
            header().child(
                nav()
                    .child(link_to("/").add_class("logo").text("RideKeeper"))
                    .child(
                        ul().child(li().child(link_to("/").text("Home")))
                            .child(li().child(link_to("/pricing").text("Pricing & plans"))),
                    ),
            ),
            main()
                .child(
                    section()
                        .set_id("hero")
                        .child(h1().text("Track every service"))
                        .child(p().text("Fuel, tyres & chain — all in one place."))
                        .child(
                            link_to("https://apps.example/app")
                                .add_class("button")
                                .text("Download"),
                        ),
                )
                .child(
                    table()
                        .child(caption().text("Plans"))
                        .child(
                            thead().child(tr().child(th().text("Plan")).child(th().text("Price"))),
                        )
                        .child(
                            tbody()
                                .child(tr().child(td().text("Free")).child(td().text("R$ 0")))
                                .child(
                                    tr().child(td().text("Pro")).child(td().text("R$ 9,90/mês")),
                                ),
                        ),
                )
                .child(
                    details()
                        .bool_attr("open")
                        .child(summary().text("Is my data private?"))
                        .child(p().text("Yes — everything syncs through your own iCloud account.")),
                )
                .child(
                    form().attr("action", "/subscribe").child(
                        fieldset()
                            .child(legend().text("Newsletter"))
                            .child(label_for("email").text("E-mail"))
                            .child(input_named("email", "email").bool_attr("required"))
                            .child(button_typed("submit").text("Subscribe")),
                    ),
                )
                .child(pre().child(code().text("let page = html { }")))
                .child(
                    figure()
                        .child(image("/img/app.png", "App screenshot"))
                        .child(figcaption().text("The garage screen")),
                ),
            footer().child(p().text("© 2026 RideKeeper — built with Swift & WingedSwift")),
        ])
}

#[test]
fn marketing_page_matches_the_pretty_fixture() {
    assert_matches_fixture("marketing-pretty.html", &marketing_page().render());
}

#[test]
fn marketing_page_matches_the_compact_fixture() {
    assert_matches_fixture("marketing-compact.html", &marketing_page().render_compact());
}

/// The two modes must describe the same document. Stripping whitespace between tags from
/// the pretty output should land on the compact one.
#[test]
fn the_two_render_modes_agree_on_content() {
    let pretty = marketing_page().render();
    let compact = marketing_page().render_compact();

    let squashed: String = pretty
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .join("")
        .replacen("<!DOCTYPE html>", "<!DOCTYPE html>\n", 1);

    // `<pre>` content is the one place where the two legitimately differ, and this page's
    // `<pre>` has no internal newlines, so the comparison is exact here.
    assert_eq!(squashed, compact);
}
