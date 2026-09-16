# Changelog

All notable changes to this project are documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this
project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Core engine** — `Node`, `Element`, `Attribute`, the `Render` trait and `RenderOptions`.
  Rendering is buffered: the whole tree writes into one `String`.
- **Three escapers** — `escape_text`, `escape_attribute` and `escape_xml`. Escaping is
  applied once, when content enters the tree.
- **93 HTML elements**, generated from a single `define_elements!` table, plus typed
  constructors for the shapes that carry required arguments (`image`, `iframe_titled`,
  `label_for`, `link_to`, `stylesheet`, `button_typed`, `input_named`, `script_src`).
- **`html!` macro** — nested markup syntax with attributes, `@if` / `@else` and `@for`.
  Expands to the same tree the builder API produces.
- **`Document`** and the **`Layout`** trait.
- **SEO** — `open_graph`, `open_graph_article`, `twitter_card`, `common`, and `SeoBuilder`.
- **Accessibility** — a `Role` enum and `audit`, which reports images without `alt`,
  buttons and links with no accessible name, iframes without a title, and inputs with
  nothing a label can attach to.
- **`sitemap`** and **`feed`** — XML sitemaps, sitemap indexes and RSS 2.0.
- **`ssg`** — `StaticSiteGenerator` with atomic writes, behind `feature = "ssg"`.
- **`parallel`** — `rayon`-backed bulk page generation, behind `feature = "parallel"`.
- **`wasm`** — `wasm-bindgen` bindings wrapping the real tree, behind `feature = "wasm"`.
- **`Node::Comment`** — an HTML comment node, which Winged-Swift does not have. `--` in the
  content is neutralised so a comment cannot close early.
- **Golden-file tests** reproducing all four Winged-Swift fixtures byte for byte, with
  `WINGED_UPDATE_FIXTURES=1` to regenerate.
- **NPM package** — `web`, `nodejs` and `bundler` builds, 20 KB gzipped, with TypeScript
  typings. `npm/smoke.mjs` renders the golden marketing page through WebAssembly and diffs
  it against the same fixture the Rust tests use, so native and WASM output cannot drift
  apart unnoticed.
- **Browser and Node examples**, both runnable with no bundler.
- **`scripts/build-wasm.sh`** — all three targets, with a gzipped size budget enforced in
  CI.
- **`scripts/verify-npm.sh`** — `tsc --noEmit` over the generated typings, then installs a
  packed tarball into a throwaway project rather than linking the workspace, which is what
  catches a wrong `files` allowlist or a broken `exports` map.
- **`scripts/verify.sh`** — one command that is a superset of CI.
- **`scripts/generate-tag-catalog.sh --check`** — documentation drift fails the build.

### Changed from Winged-Swift 2.0.0

Every deliberate difference is listed with its reason in [`PORTING.md`](PORTING.md).
The short version:

- `data_attrs` and `aria_attrs` produce deterministic output; the Swift versions iterate a
  `Dictionary` and do not.
- The node tree is `Send + Sync`. Winged-Swift's `ROADMAP.md` lists its tree not being
  `Sendable` as an unresolved 3.0 problem.
- `seo::common` emits the `<title>` it is given. `SEO.common` accepts a title and drops it.
- `StaticSiteGenerator::clean` refuses empty, root and `..`-containing paths, and page
  paths cannot escape the output directory.
- `generate_multiple` reports every failure rather than the first.
- Writes are atomic.
- The process-wide `HTMLTag.xhtmlSelfClosing` switch is not ported; pass
  `RenderOptions::with_xhtml_self_closing` instead.

### Known limitations

- The renderer is recursive, so rendering a tree more than roughly 2,000 levels deep
  overflows the stack and aborts. 256 levels is guaranteed and covered by a test — deeper
  than browsers themselves handle. Tracked in
  [#33](https://github.com/micheltlutz/winged-rust/issues/33).

### Fixed relative to Winged-Swift

- `Scripts/generate-tag-catalog.sh` matches `public class [A-Za-z]+: HTMLTag`, which rejects
  digits, so `H1`–`H6` are silently missing from its checked-in catalog. The Rust generator
  covers all 93 tags, and a test asserts the headings are present.
