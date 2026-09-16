//! Fast, type-safe HTML generation for Rust and WebAssembly.
//!
//! A port of [Winged-Swift](https://github.com/micheltlutz/Winged-Swift) 2.0.0: the same
//! composite tree, the same escape-by-default guarantee, the same compact and pretty
//! render modes — compiled to native code or to `wasm32-unknown-unknown`.
//!
//! # Getting started
//!
//! ```
//! use winged_rust::prelude::*;
//!
//! let card = article()
//!     .add_class("card p-4")
//!     .child(h1().text("Welcome to WingedRust!").add_class("text-2xl"))
//!     .child(p().text("Ultra-fast HTML generation compiled to WebAssembly."));
//!
//! println!("{}", card.render_pretty());
//! ```
//!
//! # Escaping
//!
//! Text content and attribute values are escaped **when they enter the tree**, not when it
//! is rendered, so nothing is escaped twice and nothing is missed. [`Node::Raw`] is the
//! documented way to inject markup you already trust.
//!
//! ```
//! use winged_rust::prelude::*;
//! assert_eq!(p().text("<script>").render(), "<p>&lt;script&gt;</p>");
//! ```
//!
//! # Render modes
//!
//! [`Render::render`] produces a single line; [`Render::render_pretty`] indents. Both take
//! their configuration from a [`RenderOptions`] value rather than global state, so two
//! threads can render the same tree differently at the same time.
//!
//! # Parity
//!
//! The crate is verified against Winged-Swift's own golden fixtures — the same bytes, from
//! the same page, built through both APIs. Behavioural differences that were introduced on
//! purpose are listed in `PORTING.md`.

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod core;
pub mod document;
pub mod elements;
pub mod layout;
pub mod prelude;

pub use crate::core::{Attribute, Element, Node, Render, RenderOptions};
pub use crate::document::Document;
pub use crate::layout::Layout;
