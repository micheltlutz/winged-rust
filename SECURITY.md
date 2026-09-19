# Security policy

## Reporting a vulnerability

Report privately through GitHub:
**[Open a security advisory](https://github.com/micheltlutz/winged-rust/security/advisories/new)**

Please do not open a public issue for a vulnerability.

Include what you have: the input, the rendered output you expected, the output you got, and
the crate version. A failing test is the fastest possible report.

You should get an acknowledgement within a few days. If a fix is warranted it ships in a
patch release, and the advisory is published once users have had a chance to upgrade.

## Supported versions

The latest released version. There are no long-term support branches.

## What counts as a vulnerability here

This crate's security surface is narrow and specific: **it generates HTML strings, and its
one safety guarantee is that text and attribute values you pass in cannot become markup.**

These are vulnerabilities:

- Any input to `text()`, `Attribute::new`, `Element::attr`, or the escaping functions that
  survives into the output as executable markup or as an attribute that breaks out of its
  quoting.
- A class, id, style or `data-*` value that can inject an additional attribute.
- `Node::comment` content that can terminate the comment early.
- `StaticSiteGenerator` writing outside its output directory, or `clean()` deleting outside
  it.

### Rendering depth is no longer bounded by the stack

The writer walks an explicit work stack instead of recursing, and `Element` tears its
subtree down the same way, so nesting depth costs heap — bounded by a tree that already
fits in memory — rather than stack. A 100,000-level tree renders and is freed in the test
suite, in both render modes.

Before this, a deep enough tree exhausted the stack, and a stack overflow is a process
**abort**, not a catchable panic — `catch_unwind` would not have saved you. That made
nesting depth a denial-of-service vector anywhere it could be influenced by untrusted
input. Fixed in [#33](https://github.com/micheltlutz/winged-rust/issues/33); Winged-Swift
still has the recursive shape.

**What deep trees still cost is output size.** Pretty mode writes one indent string per
level on every line, so output grows with the square of the depth: a 100,000-level tree
pretty-prints to tens of gigabytes. If depth comes from untrusted input, bound it, or
render compact, or set an empty indent with
`RenderOptions::pretty().with_indent("")`.

One residual, and it is not a stack limit on rendering: a bare `Node::Fragment` chain with
no element anywhere in it is still freed recursively. Giving `Node` its own `Drop` would
forbid `match node { Node::Element(element) => element }` — a partial move this crate's own
`static_site` example performs — so the iterative teardown lives on `Element`. Every
fragment reachable through an element, which is every fragment in a document, is freed
iteratively.

These are **not** vulnerabilities, because they are documented behaviour:

- `raw_text()` and `Node::Raw` not escaping their input. That is their entire purpose; they
  are named and documented so that an audit can grep for them.
- `<script>` and `<style>` content not being escaped by default, matching Winged-Swift.
- `Attribute::raw` not escaping. It exists for library-controlled keys.
- The library not validating URLs. `link_to("javascript:alert(1)")` renders what you asked
  for. URL policy is on the roadmap, not in the current guarantee.
- Rendering a tree deeper than the documented 256 levels. See the limitation above — it is
  a known bound, not an undisclosed one.

## For users

- Pass untrusted data through `text()` and `attr()`, never `raw_text()` or `Node::Raw`.
- If you must inject markup you did not author, sanitize it with a real HTML sanitizer
  first — this crate escapes, it does not sanitize.
- Run `winged_rust::accessibility::audit` in development; several of its findings are also
  signs of markup being assembled by hand.
