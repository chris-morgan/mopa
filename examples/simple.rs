#![feature(core)]

#[macro_use] #[no_link]
extern crate mopa;

use std::any::Any;

trait PanicAny: Any { }

mopafy!(PanicAny);

impl PanicAny for i32 { }

fn main() {
    let p = &2 as &PanicAny;
    println!("{}", p.is::<i32>());
}
