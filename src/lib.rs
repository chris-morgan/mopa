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
//!    ```rust,ignore
//!    #[macro_use] #[no_link]
//!    extern crate mopa;
//!    ```
//!
//! 2. Make `Any` a supertrait of `Person`;
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
//! #![feature(core)]
//! #[macro_use] #[no_link]
//! extern crate mopa;
//!
//! use std::any::Any;
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
//! trait Person: Any {
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
//! downcast an Any trait object (or a mopafied trait object) is with a generic parameter, when
//! producing something like `AnyMap`, for example. If you control *all* the code, `Any` trait
//! objects are probably not the right solution; they’re good for cases with user-defined
//! types across a variety of libraries. But the question of purpose and suitability is open, and I
//! don’t have a really good example of such a use case here at present. TODO.

/// The macro for implementing all the `Any` methods on your own trait.
///
/// # Instructions for use
///
/// 1. Make sure your trait extends `Any` (e.g. `trait Trait: Any { }`)
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
///    # #![feature(core)]
///    # #[macro_use] #[no_link] extern crate mopa;
///    # trait Trait: std::any::Any { }
///    mopafy!(Trait);
///    # fn main() { }
///    ```
///
/// 2. If you are using **libcore** but not libstd (`#![no_std]`) or liballoc:
///
///    ```rust
///    # #![feature(core)]
///    # #[macro_use] #[no_link] extern crate mopa;
///    # extern crate core;
///    # trait Trait: core::any::Any { }
///    mopafy!(Trait, core = core);
///    # fn main() { }
///    ```
///
///    (This is akin to `mopafy!(Trait, core = std)` if you were using libstd.)
///
///    Unlike the other two techniques, this only gets you the `&Any` and `&mut Any` methods; the
///    `Box<Any>` methods require liballoc.
///
/// 3. If you are using **libcore and liballoc** but not libstd (`#![nostd]`):
///
///    ```rust
///    # #![feature(core, alloc)]
///    # #[macro_use] #[no_link] extern crate mopa;
///    # extern crate core;
///    # extern crate alloc;
///    # trait Trait: core::any::Any { }
///    mopafy!(Trait, core = core, alloc = alloc);
///    # fn main() { }
///    ```
///
///    (This is akin to `mopafy!(Trait, core = std, alloc = std)` if you were using libstd; in
///    fact, the first form is just sugar for this very thing.)
///
///    This gets you all the methods.
#[macro_export]
macro_rules! mopafy {
    // Using libstd like a normal person? Here’s what you want, just a simple `mopafy!(Trait)`.
    ($trait_:ident) => {
        mopafy!($trait_, core = std, alloc = std);
    };

    // Not using libstd or liballoc? You can get the &Any and &mut Any methods by specifying what
    // libcore is here, e.g. `mopafy!(Trait, core = core)`, but you won’t get the `Box<Any>`
    // methods.
    ($trait_:ident, core = $core:ident) => {
        impl $trait_ {
            /// Returns true if the boxed type is the same as `T`
            #[inline]
            pub fn is<T: $trait_>(&self) -> bool {
                ::$core::any::TypeId::of::<T>() == ::$core::any::TypeId::of::<Self>()
            }

            /// Returns some reference to the boxed value if it is of type `T`, or
            /// `None` if it isn't.
            #[inline]
            pub fn downcast_ref<T: $trait_>(&self) -> ::$core::option::Option<&T> {
                if self.is::<T>() {
                    unsafe {
                        ::$core::option::Option::Some(self.downcast_ref_unchecked())
                    }
                } else {
                    ::$core::option::Option::None
                }
            }

            /// Returns a reference to the boxed value, blindly assuming it to be of type `T`.
            /// If you are not *absolutely certain* of `T`, you *must not* call this.
            #[inline]
            pub unsafe fn downcast_ref_unchecked<T: $trait_>
                                                (&self) -> &T {
                let trait_object: ::$core::raw::TraitObject = ::$core::mem::transmute(self);
                ::$core::mem::transmute(trait_object.data)
            }

            /// Returns some mutable reference to the boxed value if it is of type `T`, or
            /// `None` if it isn't.
            #[inline]
            pub fn downcast_mut<T: $trait_>(&mut self) -> ::$core::option::Option<&mut T> {
                if self.is::<T>() {
                    unsafe {
                        ::$core::option::Option::Some(self.downcast_mut_unchecked())
                    }
                } else {
                    ::$core::option::Option::None
                }
            }

            /// Returns a mutable reference to the boxed value, blindly assuming it to be of type `T`.
            /// If you are not *absolutely certain* of `T`, you *must not* call this.
            #[inline]
            pub unsafe fn downcast_mut_unchecked<T: $trait_>
                                                (&mut self) -> &mut T {
                let trait_object: ::$core::raw::TraitObject = ::$core::mem::transmute(self);
                ::$core::mem::transmute(trait_object.data)
            }
        }
    };

    // Not using libstd? You can get the Box<Any> methods by specifying what liballoc is here,
    // e.g. `mopafy!(Trait, alloc = alloc)`
    ($trait_:ident, core = $core:ident, alloc = $alloc:ident) => {
        mopafy!($trait_, core = $core);

        impl $trait_ {
            /// Returns the boxed value if it is of type `T`, or `Err(Self)` if it isn't.
            #[inline]
            pub fn downcast<T: $trait_>(self: ::$alloc::boxed::Box<Self>)
                    -> ::$core::result::Result<::$alloc::boxed::Box<T>,
                                               ::$alloc::boxed::Box<Self>> {
                if self.is::<T>() {
                    unsafe {
                        ::$core::result::Result::Ok(self.downcast_unchecked())
                    }
                } else {
                    ::$core::result::Result::Err(self)
                }
            }

            /// Returns the boxed value, blindly assuming it to be of type `T`.
            /// If you are not *absolutely certain* of `T`, you *must not* call this.
            #[inline]
            pub unsafe fn downcast_unchecked<T: $trait_>(self: ::$alloc::boxed::Box<Self>)
                    -> ::$alloc::boxed::Box<T> {
                let trait_object: ::$core::raw::TraitObject = ::$core::mem::transmute(self);
                ::$core::mem::transmute(trait_object.data)
            }
        }
    };
}
