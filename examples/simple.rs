#![feature(phase)]

#[phase(plugin)]
extern crate mopa;

use std::any::Any;

trait Panic { }

trait PanicAny: Panic + Any { }

mopafy!(PanicAny);

impl Panic for int { }

impl<T: Panic + Any + 'static> PanicAny for T { }

fn main() {
    let p = &2i as &PanicAny;
    println!("{}", p.is::<int>());
}
