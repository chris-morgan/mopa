#[macro_use] #[no_link]
extern crate mopa;

use std::any::Any;

trait PanicAny: Any { }

mopafy!(PanicAny);

impl PanicAny for int { }

fn main() {
    let p = &2i as &PanicAny;
    println!("{}", p.is::<int>());
}
