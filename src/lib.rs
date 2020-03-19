// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This crate provides a number of traits with default implementations
//! for most of the standard library's methods on array like data structures.
//! All you need to do to apply them to your own array like data structure
//! is to implement `HasLength` and `Index<usize>` (and `IndexMut<usize>` for
//! mutable operations), which means you need a `len()` method and an `index()`
//! method, and the `Array` trait will provide default methods for
//! everything else, implemented using just those two methods.
//!
//! Note that if you can implement `Deref<Target=[A]>` for your data type,
//! you don't need this, all of these methods will be provided by the primitive
//! slice type. This crate exists to make it easy to write array like data types
//! where you can't deref to a slice because the data isn't laid out in one
//! continuous memory array. `std::collections::VecDeque` is a very basic example
//! of this: it's implemented using two `Vec`s, so there's no way to get a single slice
//! out of it. A vector trie like `im::Vector` is another example, where the
//! elements are laid out across multiple fixed size nodes in a tree structure.
//!
//! Speaking of `VecDeque`, this crate provides `Array`/`ArrayMut`
//! implementations for it, so if you ever needed to sort a `VecDeque`,
//! now you can.
//!
//! # Performance Notes
//!
//! Many of these methods may have smarter implementations for your specific
//! data type. In this case, you should provide your own implementations of
//! these. In particular, providing your own `get` and `get_mut` using native
//! `get_unchecked` and `get_unchecked_mut` implementations with bounds
//! checking added is almost always going to be better than the
//! default implementation, which adds bounds checking to an `index` call,
//! most likely leading to bounds being checked twice.
//!
//! The sorting algorithm provided is an implementation of optimal quicksort
//! with randomised pivots, which should be a safe choice for any array-like, but
//! there may well be better algoritms available for your particular data type.
//! In particular, the quicksort isn't stable, which is why `ArrayMut` only
//! provides `sort_unstable` and not `sort`.
//!
//! # Example
//!
//! ```rust
//! # use array_ops::*;
//! # use std::ops::{Index, IndexMut};
//! #[derive(PartialEq, Eq, Debug)]
//! struct MyNewtypedVec<A>(Vec<A>);
//!
//! impl<A> From<Vec<A>> for MyNewtypedVec<A> {
//!     fn from(vec: Vec<A>) -> Self {
//!         Self(vec)
//!     }
//! }
//!
//! impl<A> HasLength for MyNewtypedVec<A> {
//!     fn len(&self) -> usize {
//!         self.0.len()
//!     }
//! }
//!
//! impl<A> Index<usize> for MyNewtypedVec<A> {
//!     type Output = A;
//!     fn index(&self, index: usize) -> &A {
//!         self.0.index(index)
//!     }
//! }
//!
//! impl<A> IndexMut<usize> for MyNewtypedVec<A> {
//!     fn index_mut(&mut self, index: usize) -> &mut A {
//!         self.0.index_mut(index)
//!     }
//! }
//!
//! impl<A> Array for MyNewtypedVec<A> {}
//! impl<A> ArrayMut for MyNewtypedVec<A> {}
//!
//! let mut my_vec = MyNewtypedVec::from(vec![3, 1, 3, 3, 7]);
//! assert!(my_vec.starts_with(&[3, 1, 3]));
//! my_vec.sort_unstable();
//! let expected = MyNewtypedVec::from(vec![1, 3, 3, 3, 7]);
//! assert_eq!(expected, my_vec);
//! ```

#![forbid(rust_2018_idioms)]
#![deny(nonstandard_style)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![cfg_attr(test, deny(warnings))]

mod array;
mod sort;
mod std_types;

pub use self::array::*;
