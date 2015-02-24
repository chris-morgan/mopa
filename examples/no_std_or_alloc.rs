#![feature(lang_items, start, core, libc, no_std)]
#![no_std]

#[macro_use] #[no_link]
extern crate mopa;

extern crate core;
extern crate libc;

trait Panic { fn panic(&self) { } }

trait PanicAny: Panic + core::any::Any { }

mopafy!(PanicAny, core = core);

impl Panic for i32 { }

impl<T: Panic + core::any::Any + 'static> PanicAny for T { }

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    let p = &2 as &PanicAny;
    if p.is::<i32>() {
        0
    } else {
        1
    }
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() {}
