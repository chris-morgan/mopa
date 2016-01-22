// This example depends on the no_std feature being enabled on the crate;
// without it, we have to go and chop everything off so that it can compile.
// If you are basing something off this example, please note that all the `feature = "no_std"`
// cfg-gating is a workaround for Cargo until https://github.com/rust-lang/cargo/issues/1570 lands.
// Do not include it if you copy any code.

#![cfg_attr(feature = "no_std", feature(lang_items, start, alloc))]
#![cfg_attr(feature = "no_std", no_std)]

#[cfg(not(feature = "no_std"))]
fn main() { }

#[cfg(feature = "no_std")]
#[macro_use]
extern crate mopa;

#[cfg(feature = "no_std")]
extern crate alloc;

#[cfg(feature = "no_std")]
mod silly_wrapper_to_save_writing_the_whole_cfg_incantation_on_every_item {
    trait Panic { fn panic(&self) { } }

    trait PanicAny: Panic + ::mopa::Any { }

    mopafy!(PanicAny, core = core, alloc = alloc);

    impl Panic for i32 { }

    impl<T: Panic + ::mopa::Any + 'static> PanicAny for T { }

    #[start]
    fn start(_argc: isize, _argv: *const *const u8) -> isize {
        let p: &PanicAny = &2;
        if p.is::<i32>() {
            0
        } else {
            1
        }
    }

    #[lang = "stack_exhausted"] extern fn stack_exhausted() {}
    #[lang = "eh_personality"] extern fn eh_personality() {}
    #[lang = "panic_fmt"] extern fn panic_fmt() {}
}
