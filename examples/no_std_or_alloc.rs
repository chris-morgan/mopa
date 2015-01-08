#![feature(lang_items)]
#![no_std]

#[macro_use] #[no_link]
extern crate mopa;

extern crate core;
extern crate libc;

trait Panic { }

trait PanicAny: Panic + core::any::Any { }

mopafy!(PanicAny, core = core);

impl Panic for int { }

impl<T: Panic + core::any::Any + 'static> PanicAny for T { }

#[start]
fn start(_argc: int, _argv: *const *const u8) -> int {
    let p = &2i as &PanicAny;
    if p.is::<int>() {
        0
    } else {
        1
    }
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() {}
