// #![cfg(no_std)]
// #![feature(lang_items, start, core, alloc, no_std)]
// #![no_std]

// #[macro_use]
// extern crate mopa;

// extern crate core;
// extern crate alloc;

// trait Panic { fn panic(&self) { } }

// trait PanicAny: Panic + core::any::Any { }

// mopafy!(PanicAny, core = core, alloc = alloc);

// impl Panic for i32 { }

// impl<T: Panic + core::any::Any + 'static> PanicAny for T { }

// #[start]
// fn start(_argc: isize, _argv: *const *const u8) -> isize {
//     let p: &PanicAny = &2;
//     if p.is::<i32>() {
//         0
//     } else {
//         1
//     }
// }

// #[lang = "stack_exhausted"] extern fn stack_exhausted() {}
// #[lang = "eh_personality"] extern fn eh_personality() {}
// #[lang = "panic_fmt"] extern fn panic_fmt() {}

fn main() {}