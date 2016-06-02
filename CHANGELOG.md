Unreleased (2016-06-03)
=======================

Plenty of breaking changes.

- The `no_std` Cargo feature was removed; `#[no_std]` has stabilised in Rust,
  and so the `mopa` crate is now *always* `#[no_std]`.
  Note that the `no_std_examples` Cargo feature is still around, because of
  shortcomings in Cargo itself. It’s not intended for client use, though; only
  for being able to run the `no_std` and `no_std_or_alloc` examples from this
  repository.

- `mopa` trait no longer needs the `no_std` feature in order not to depend on 

- `mopafy!` syntax changed for advanced cases, simplifying things a little.

  - `mopafy!(Trait)` is unchanged.

  - `mopafy!(Trait, core = name_of_libcore_crate)` → `mopafy!(Trait, only core)`.
    (That’s the literal token `core`, not the name of the libcore crate.)

  - `mopafy!(Trait, core = name_of_libcore_crate, alloc = name_of_liballoc_crate)` →
    `use alloc::boxed::Box; mopafy!(Trait);`

0.2.2 (2016-04-05)
==================

- Dead code warnings suppressed.

0.2.1 (2016-01-22)
==================

- Update for Rust `#[no_std]` compatibility.

0.2.0 (2015-05-13)
==================

- Support beta/stable.

- Traits being mopafied now need to extend `mopa::Any`, not `std::any::Any`.
  This is a breaking change.

- Users of `#[no_std]` will now need to enable the `no_std` Cargo feature on
  this crate.

- `#![feature(core)]` is no longer necessary.

0.1.1–0.1.8 (2015-01-07–2015-04-14)
===================================

Updates to cope with Rust language changes.
This is ancient history, before Rust 1.0.0.

0.1.0 (2015-01-06)
==================

Initial release. Supports nightly channel only.
(OK, so at this point there *was* only nightly.)
