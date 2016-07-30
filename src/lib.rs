// This is largely taken from the Rust distribution, with only comparatively
// minor additions and alterations. Therefore, their copyright notice follows:
//
//     Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
//     file at the top-level directory of this distribution and at
//     http://rust-lang.org/COPYRIGHT.
//
//     Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//     http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//     <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//     option. This file may not be copied, modified, or distributed
//     except according to those terms.
//
// I have kept my additions under the same terms (being rather fond of MIT/Apache-2.0 myself).

//! **MOPA: My Own Personal Any.** A macro to implement all the `Any` methods on your own trait.
//!
//! You like `Any`—its ability to store any `'static` type as a trait object and then downcast it
//! back to the original type is very convenient, and in fact you need it for whatever misguided
//! reason. But it’s not enough. What you *really* want is your own trait object type with `Any`’s
//! functionality glued onto it. Maybe you have a `Person` trait and you want your people to be
//! able to do various things, but you also want to be able to conveniently downcast the person to
//! its original type, right? Alas, you can’t write a type like `Box<Person + Any>` (at present,
//! anyway). So what do you do instead? Do you give up? No, no! No, no! Enter MOPA.
//!
//! > There once was a quite friendly trait  
//! > Called `Person`, with much on its plate.  
//! >     “I need to be `Any`  
//! >     To downcast to `Benny`—  
//! > But I’m not, so I guess I’ll just wait.”
//!
//! A pitiful tale, isn’t it? Especially given that there was a bear chasing it with intent to eat
//! it. Fortunately now you can *mopafy* `Person` in three simple steps:
//!
//! 1. Add the `mopa` crate to your `Cargo.toml` as usual and your crate root like so:
//!
//!    ```rust
//!    #[macro_use]
//!    extern crate mopa;
//!    #[macro_use]
//!    extern crate parse_generics_shim;
//!    # fn main() { }
//!    ```
//!
//! 2. Make `Any` (`mopa::Any`, not `std::any::Any`) a supertrait of `Person`;
//!
//! 3. `mopafy!(Person);`.
//!
//! And lo, you can now write `person.is::<Benny>()` and `person.downcast_ref::<Benny>()` and so on
//! to your heart’s content. Simple, huh?
//!
//! Oh, by the way, it was actually the person on the bear’s plate. There wasn’t really anything on
//! `Person`’s plate after all.
//!
//! ```rust
//! #[macro_use]
//! extern crate mopa;
//! #[macro_use]
//! extern crate parse_generics_shim;
//!
//! struct Bear {
//!     // This might be a pretty fat bear.
//!     fatness: u16,
//! }
//!
//! impl Bear {
//!     fn eat(&mut self, person: Box<Person>) {
//!         self.fatness = (self.fatness as i16 + person.weight()) as u16;
//!     }
//! }
//!
//! trait Person: mopa::Any {
//!     fn panic(&self);
//!     fn yell(&self) { println!("Argh!"); }
//!     fn sleep(&self);
//!     fn weight(&self) -> i16;
//! }
//!
//! mopafy!(Person);
//!
//! struct Benny {
//!     // (Benny is not a superhero. He can’t carry more than 256kg of food at once.)
//!     kilograms_of_food: u8,
//! }
//!
//! impl Person for Benny {
//!     fn panic(&self) { self.yell() }
//!     fn sleep(&self) { /* ... */ }
//!     fn weight(&self) -> i16 {
//!         // Who’s trying to find out? I’m scared!
//!         self.yell();
//!         self.kilograms_of_food as i16 + 60
//!     }
//! }
//!
//! struct Chris;
//!
//! impl Chris {
//!     // Normal people wouldn’t be brave enough to hit a bear but Chris might.
//!     fn hit(&self, bear: &mut Bear) {
//!         println!("Chris hits the bear! How brave! (Or maybe stupid?)");
//!         // Meh, boundary conditions, what use are they in examples?
//!         // Chris clearly hits quite hard. Poor bear.
//!         bear.fatness -= 1;
//!     }
//! }
//!
//! impl Person for Chris {
//!     fn panic(&self) { /* ... */ }
//!     fn sleep(&self) { /* ... */ }
//!     fn weight(&self) -> i16 { -5 /* antigravity device! cool! */ }
//! }
//!
//! fn simulate_simulation(person: Box<Person>, bear: &mut Bear) {
//!     if person.is::<Benny>() {
//!         // None of the others do, but Benny knows this particular
//!         // bear by reputation and he’s *really* going to be worried.
//!         person.yell()
//!     }
//!     // If it happens to be Chris, he’ll hit the bear.
//!     person.downcast_ref::<Chris>().map(|chris| chris.hit(bear));
//!     bear.eat(person);
//! }
//!
//! fn main() {
//!     let mut bear = Bear { fatness: 10 };
//!     simulate_simulation(Box::new(Benny { kilograms_of_food: 5 }), &mut bear);
//!     simulate_simulation(Box::new(Chris), &mut bear);
//! }
//! ```
//!
//! Now *should* you do something like this? Probably not. Enums are probably a better solution for
//! this particular case as written; frankly I believe that almost the only time you should
//! downcast an `Any` trait object (or a mopafied trait object) is with a generic parameter, when
//! producing something like `AnyMap`, for example. If you control *all* the code, `Any` trait
//! objects are probably not the right solution; they’re good for cases with user-defined
//! types across a variety of libraries. But the question of purpose and suitability is open, and I
//! don’t have a really good example of such a use case here at present. TODO.
#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;
#[macro_use]
extern crate parse_generics_shim;

/// Implementation details of the `mopafy!` macro.
#[doc(hidden)]
pub mod __ {
    pub use core::any::TypeId;
    // Option and Result are in the prelude, but they might have been overridden in the macro’s
    // scope, so we do it this way to avoid issues. (Result in particular gets overridden fairly
    // often.)
    pub use core::option::Option;
    pub use core::result::Result;
}

/// A type to emulate dynamic typing.
///
/// This is a simple wrapper around `core::any::Any` which exists for [technical reasons][#27745].
/// Every type that implements `core::any::Any` implements this `Any`.
///
/// See the [`core::any::Any` documentation](http://doc.rust-lang.org/core/any/trait.Any.html) for
/// more details.
///
/// Any traits to be mopafied must extend this trait (e.g. `trait Person: mopa::Any { }`).
///
/// If/when [#27745] is resolved, this trait may be replaced with a simple reexport of
/// `core::any::Any`. This will be a backwards-compatible change.
///
/// [#27745]: https://github.com/rust-lang/rust/issues/27745
pub trait Any: core::any::Any {
    /// Gets the `TypeId` of `self`. UNSTABLE; do not depend on it.
    #[doc(hidden)]
    fn __get_type_id(&self) -> __::TypeId;
}

impl<T: core::any::Any> Any for T {
    fn __get_type_id(&self) -> __::TypeId {
        __::TypeId::of::<T>()
    }
}

#[macro_export]
macro_rules! as_item {
    ($item:item) => {
        $item
    };
}

#[macro_export]
macro_rules! mopafy_internal {
    // Not using libstd or liballoc? You can get the &Any and &mut Any methods by specifying what
    // libcore is here, e.g. `mopafy!(Trait, core = core)`, but you won’t get the `Box<Any>`
    // methods.
    (
        $trait_:ident
        {
            constr: [ $($constr:tt)* ],
            params: [ $($args:tt)* ],
            $($_fields:tt)*
        },
    ) => {
        as_item! {
            #[allow(dead_code)]
            impl <$($constr)*> $trait_ <$($args)*> {
                /// Returns the boxed value if it is of type `T`, or `Err(Self)` if it isn't.
                #[inline]
                pub fn downcast<T: $trait_<$($args)*>>(self: Box<Self>) -> $crate::__::Result<Box<T>, Box<Self>> {
                    if self.is::<T>() {
                        unsafe {
                            $crate::__::Result::Ok(self.downcast_unchecked())
                        }
                    } else {
                        $crate::__::Result::Err(self)
                    }
                }

                /// Returns the boxed value, blindly assuming it to be of type `T`.
                /// If you are not *absolutely certain* of `T`, you *must not* call this.
                #[inline]
                pub unsafe fn downcast_unchecked<T: $trait_<$($args)*>>(self: Box<Self>) -> Box<T> {
                    Box::from_raw(Box::into_raw(self) as *mut T)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! mopafy_only_core_internal {
    // Not using libstd/liballoc? The core functionality can do without them; you will still have
    // the `&Any` and `&mut Any` methods but will lose the `Box<Any>` methods.
    (
        $trait_:ident
        {
            constr: [ $($constr:tt)* ],
            params: [ $($args:tt)* ],
            $($_fields:tt)*
        },
    ) => {
        as_item! {
            #[allow(dead_code)]
            impl <$($constr)*> $trait_ <$($args)*> {
                /// Returns true if the boxed type is the same as `T`
                #[inline]
                pub fn is<T: $trait_<$($args)*>>(&self) -> bool {
                    $crate::__::TypeId::of::<T>() == $crate::Any::__get_type_id(self)
                }

                /// Returns some reference to the boxed value if it is of type `T`, or
                /// `None` if it isn't.
                #[inline]
                pub fn downcast_ref<T: $trait_<$($args)*>>(&self) -> $crate::__::Option<&T> {
                    if self.is::<T>() {
                        unsafe {
                            $crate::__::Option::Some(self.downcast_ref_unchecked())
                        }
                    } else {
                        $crate::__::Option::None
                    }
                }

                /// Returns a reference to the boxed value, blindly assuming it to be of type `T`.
                /// If you are not *absolutely certain* of `T`, you *must not* call this.
                #[inline]
                pub unsafe fn downcast_ref_unchecked<T: $trait_<$($args)*>>(&self) -> &T {
                    &*(self as *const Self as *const T)
                }

                /// Returns some mutable reference to the boxed value if it is of type `T`, or
                /// `None` if it isn't.
                #[inline]
                pub fn downcast_mut<T: $trait_<$($args)*>>(&mut self) -> $crate::__::Option<&mut T> {
                    if self.is::<T>() {
                        unsafe {
                            $crate::__::Option::Some(self.downcast_mut_unchecked())
                        }
                    } else {
                        $crate::__::Option::None
                    }
                }

                /// Returns a mutable reference to the boxed value, blindly assuming it to be of type `T`.
                /// If you are not *absolutely certain* of `T`, you *must not* call this.
                #[inline]
                pub unsafe fn downcast_mut_unchecked<T: $trait_<$($args)*>>(&mut self) -> &mut T {
                    &mut *(self as *mut Self as *mut T)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! mopafy_only_core {
    ($trait_:ident $($t:tt)*) => {
        parse_generics_shim! {
            { .. },
            then mopafy_only_core_internal!($trait_),
            $($t)*
        }
    };
}

/// The macro for implementing all the `Any` methods on your own trait.
///
/// # Instructions for use
///
/// 1. Make sure your trait extends `mopa::Any` (e.g. `trait Trait: mopa::Any { }`)
///
/// 2. Mopafy your trait (see the next subsection for specifics).
///
/// 3. …
///
/// 4. Profit!
///
/// ## Mopafication techniques
///
/// There are three ways of mopafying traits, depending on what libraries you are using.
///
/// 1. If you are a **normal person**:
///
///    ```rust
///    # #[macro_use] extern crate mopa;
///    # #[macro_use] extern crate parse_generics_shim;
///    trait Trait: mopa::Any { }
///    mopafy!(Trait);
///    trait Params<A, B>: mopa::Any { }
///    mopafy!(Params<A, B>);
///    # fn main() { }
///    ```
///
/// 2. If you are using **libcore** but not libstd (`#![no_std]`) or liballoc, write this:
///
///    ```rust
///    # #[macro_use] extern crate mopa;
///    # #[macro_use] extern crate parse_generics_shim;
///    # trait Trait: mopa::Any { }
///    mopafy_only_core!(Trait);
///    trait Params<A, B>: mopa::Any { }
///    mopafy_only_core!(Params<A, B>);
///    # fn main() { }
///    ```
///
///    Unlike the other two techniques, this only gets you the `&Any` and `&mut Any` methods; the
///    `Box<Any>` methods require liballoc.
///
/// 3. If you are using **libcore and liballoc** but not libstd (`#![no_std]`), bring
///    `alloc::boxed::Box` into scope and use `mopafy!` as usual:
///
///    ```rust,ignore
///    # // This doctest is ignored so that it doesn't break tests on the stable/beta rustc
///    # // channels where #[feature] isn’t allowed.
///    # #![feature(alloc)]
///    # #[macro_use] extern crate mopa;
///    # #[macro_use] extern crate parse_generics_shim;
///    # extern crate alloc;
///    # trait Trait: mopa::Any { }
///    use alloc::boxed::Box;
///    mopafy!(Trait);
///    trait Params<A, B>: mopa::Any { }
///    mopafy!(Params<A, B>);
///    # fn main() { }
///    ```
#[macro_export]
macro_rules! mopafy {
    // Implement the full suite of `Any` methods: those of `&Any`, `&mut Any` and `Box<Any>`.
    //
    // If you’re not using libstd, you’ll need to `use alloc::boxed::Box;`, or forego the
    // `Box<Any>` methods by just using `mopafy_only_core!(Trait);`.
    ($trait_:ident $($t:tt)*) => {
        mopafy_only_core!($trait_ $($t)*);
        parse_generics_shim! {
            { .. },
            then mopafy_internal!($trait_),
            $($t)*
        }
    };
}

#[cfg(test)]
mod tests {
    use std::prelude::v1::*;

    trait Person: super::Any {
        fn weight(&self) -> i16;
    }

    mopafy!(Person);

    #[derive(Clone, Debug, PartialEq)]
    struct Benny {
        // (Benny is not a superhero. He can’t carry more than 256kg of food at once.)
        kilograms_of_food: u8,
    }

    impl Person for Benny {
        fn weight(&self) -> i16 {
            self.kilograms_of_food as i16 + 60
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    struct Chris;

    impl Person for Chris {
        fn weight(&self) -> i16 { -5 /* antigravity device! cool! */ }
    }

    trait Parameterized<A, B>: super::Any {
        fn test(&self, a: A, b: &B) -> i32;
    }

    mopafy!(Parameterized<A, B>);

    impl <'a, B> Parameterized<&'a i32, B> for Benny {
        fn test(&self, x: &'a i32, _: &B) -> i32 {
            *x
        }
    }

    impl <A, B> Parameterized<A, B> for Chris {
        fn test(&self, _: A, _: &B) -> i32 {
            0
        }
    }

    trait Float {}
    impl Float for f32 {}
    impl Float for f64 {}

    trait Deep<F: Float> {}
    struct DeepStruct;
    impl<F: Float> Deep<F> for DeepStruct {}

    trait Constrained<X, F: Float, D: Deep<F>>: super::Any {
        // Don't ask me why
        fn roundness(&self) -> i16;
    }

    mopafy!(Constrained<X, F: Float, D: Deep<F>>);

    impl<F: Float, D: Deep<F>> Constrained<u8, F, D> for Benny {
        fn roundness(&self) -> i16 {
            2i16
        }
    }

    impl<X, F: Float, D: Deep<F>> Constrained<X, F, D> for Chris {
        fn roundness(&self) -> i16 {
            5i16
        }
    }

    #[test]
    fn test_ref() {
        let benny = Benny { kilograms_of_food: 13 };
        let benny_ptr: *const Benny = &benny;
        let person: &Person = &benny;

        assert!(person.is::<Benny>());
        assert_eq!(person.downcast_ref::<Benny>().map(|x| x as *const Benny), Some(benny_ptr));
        assert_eq!(unsafe { person.downcast_ref_unchecked::<Benny>() as *const Benny }, benny_ptr);

        assert!(!person.is::<Chris>());
        assert_eq!(person.downcast_ref::<Chris>(), None);
    }

    #[test]
    fn test_mut() {
        let mut benny = Benny { kilograms_of_food: 13 };
        let benny_ptr: *const Benny = &benny;
        let person: &mut Person = &mut benny;
        assert!(person.is::<Benny>());
        assert_eq!(person.downcast_ref::<Benny>().map(|x| x as *const Benny), Some(benny_ptr));
        assert_eq!(person.downcast_mut::<Benny>().map(|x| &*x as *const Benny), Some(benny_ptr));
        assert_eq!(unsafe { person.downcast_ref_unchecked::<Benny>() as *const Benny }, benny_ptr);
        assert_eq!(unsafe { &*person.downcast_mut_unchecked::<Benny>() as *const Benny }, benny_ptr);

        assert!(!person.is::<Chris>());
        assert_eq!(person.downcast_ref::<Chris>(), None);
        assert_eq!(person.downcast_mut::<Chris>(), None);
    }

    #[test]
    fn test_box() {
        let mut benny = Benny { kilograms_of_food: 13 };
        let mut person: Box<Person> = Box::new(benny.clone());
        assert!(person.is::<Benny>());
        assert_eq!(person.downcast_ref::<Benny>(), Some(&benny));
        assert_eq!(person.downcast_mut::<Benny>(), Some(&mut benny));
        assert_eq!(person.downcast::<Benny>().map(|x| *x).ok(), Some(benny.clone()));

        person = Box::new(benny.clone());
        assert_eq!(unsafe { person.downcast_ref_unchecked::<Benny>() }, &benny);
        assert_eq!(unsafe { person.downcast_mut_unchecked::<Benny>() }, &mut benny);
        assert_eq!(unsafe { *person.downcast_unchecked::<Benny>() }, benny);

        person = Box::new(benny.clone());
        assert!(!person.is::<Chris>());
        assert_eq!(person.downcast_ref::<Chris>(), None);
        assert_eq!(person.downcast_mut::<Chris>(), None);
        assert!(person.downcast::<Chris>().err().is_some());
    }

    #[test]
    fn parameterized() {
        let i123 = 123;
        let mut benny = Benny { kilograms_of_food: 13 };
        let mut person: Box<Parameterized<&i32, String>> = Box::new(benny.clone());
        assert!(person.is::<Benny>());
        assert_eq!(person.downcast_ref::<Benny>(), Some(&benny));
        assert_eq!(person.downcast_mut::<Benny>(), Some(&mut benny));
        assert_eq!(person.downcast::<Benny>().map(|x| *x).ok(), Some(benny.clone()));

        person = Box::new(benny.clone());
        assert_eq!(unsafe { person.downcast_ref_unchecked::<Benny>() }, &benny);
        assert_eq!(unsafe { person.downcast_mut_unchecked::<Benny>() }, &mut benny);
        assert_eq!(unsafe { *person.downcast_unchecked::<Benny>() }, benny);

        person = Box::new(benny.clone());
        assert!(!person.is::<Chris>());
        assert_eq!(person.downcast_ref::<Chris>(), None);
        assert_eq!(person.downcast_mut::<Chris>(), None);

        assert_eq!(person.test(&i123, &"".into()), 123);
        person = Box::new(Chris);
        assert_eq!(person.test(&i123, &"".into()), 0);

        assert!(person.downcast::<Benny>().is_err());
    }

    #[test]
    fn constrained() {
        let mut benny = Benny { kilograms_of_food: 13 };
        let mut person: Box<Constrained<u8, f32, DeepStruct>> = Box::new(benny.clone());
        assert!(person.is::<Benny>());
        assert_eq!(person.downcast_ref::<Benny>(), Some(&benny));
        assert_eq!(person.downcast_mut::<Benny>(), Some(&mut benny));
        assert_eq!(person.downcast::<Benny>().map(|x| *x).ok(), Some(benny.clone()));

        person = Box::new(benny.clone());
        assert_eq!(unsafe { person.downcast_ref_unchecked::<Benny>() }, &benny);
        assert_eq!(unsafe { person.downcast_mut_unchecked::<Benny>() }, &mut benny);
        assert_eq!(unsafe { *person.downcast_unchecked::<Benny>() }, benny);

        person = Box::new(benny.clone());
        assert!(!person.is::<Chris>());
        assert_eq!(person.downcast_ref::<Chris>(), None);
        assert_eq!(person.downcast_mut::<Chris>(), None);

        assert_eq!(person.roundness(), 2i16);
        person = Box::new(Chris);
        assert_eq!(person.roundness(), 5i16);

        assert!(person.downcast::<Benny>().is_err());
    }
}
